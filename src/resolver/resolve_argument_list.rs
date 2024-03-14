use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;
use maplit::{btreemap, btreeset};
use crate::ast::argument::ArgumentResolved;
use crate::ast::argument_list::ArgumentList;
use crate::ast::callable_variant::CallableVariant;
use crate::ast::generics::GenericsConstraint;
use crate::ast::type_info::TypeInfo;
use crate::ast::span::Span;
use crate::diagnostics::diagnostics::{DiagnosticsError, DiagnosticsLog, DiagnosticsWarning};
use crate::r#type::keyword::Keyword;
use crate::r#type::r#type::Type;
use crate::resolver::resolve_expression::resolve_expression;
use crate::resolver::resolver_context::ResolverContext;
use crate::traits::node_trait::NodeTrait;
use crate::traits::resolved::Resolve;

pub(super) fn resolve_argument_list<'a, 'b>(
    callable_span: Span,
    argument_list: Option<&'a ArgumentList>,
    callable_variants: Vec<CallableVariant<'a>>,
    keywords_map: &BTreeMap<Keyword, Type>,
    context: &'a ResolverContext<'a>,
    pipeline_type_context: Option<&'b TypeInfo>,
) -> Option<Type> {
    if let Some(argument_list) = argument_list {
        // errors for partial argument
        for partial_argument in argument_list.partial_arguments() {
            context.insert_diagnostics_error(partial_argument.span, "partial argument");
        }
        // errors for duplicated arguments
        argument_list.arguments().duplicates_by(|a| a.name().map(|n| n.name())).for_each(|a| {
            if let Some(name) = a.name() {
                context.insert_diagnostics_error(name.span(), "duplicated argument name");
            }
        });
    }

    // the main body starts
    let matched_variants = matched_callable_variants(&callable_variants, argument_list);
    let only_to_match = if callable_variants.len() == 1 {
        Some(callable_variants.first().unwrap())
    } else if matched_variants.len() == 1 {
        Some(*matched_variants.first().unwrap())
    } else {
        None
    };
    if let Some(only_to_match) = only_to_match {
        let (errors, warnings, t, _) = try_resolve_argument_list_for_callable_variant(
            callable_span,
            argument_list,
            only_to_match,
            keywords_map,
            context,
            pipeline_type_context,
        );
        for error in errors {
            context.insert_diagnostics_error(*error.span(), error.message());
        }
        for warning in warnings {
            context.insert_diagnostics_error(*warning.span(), warning.message());
        }
        return t;
    } else {
        for callable_variant in &callable_variants {
            let (errors, warnings, t, matched) = try_resolve_argument_list_for_callable_variant(
                callable_span,
                argument_list,
                callable_variant,
                keywords_map,
                context,
                pipeline_type_context,
            );
            if matched {
                for error in errors {
                    context.insert_diagnostics_error(*error.span(), error.message());
                }
                for warning in warnings {
                    context.insert_diagnostics_warning(*warning.span(), warning.message());
                }
                return t;
            }
        }
        context.insert_diagnostics_error(callable_span, "callable variant not found for arguments");
        return Some(Type::Undetermined);
    }
}

fn try_resolve_argument_list_for_callable_variant<'a, 'b>(
    callable_span: Span,
    argument_list: Option<&'a ArgumentList>,
    callable_variant: &CallableVariant<'a>,
    keywords_map: &BTreeMap<Keyword, Type>,
    context: &'a ResolverContext<'a>,
    type_info: Option<&'b TypeInfo>,
) -> (Vec<DiagnosticsError>, Vec<DiagnosticsWarning>, Option<Type>, bool) {
    // declare errors and warnings
    let mut matched = false;
    let mut errors = vec![];
    let warnings = vec![];
    // collect generics identifiers
    let mut generic_identifiers = btreeset!{};
    for g in &callable_variant.generics_declarations {
        for i in g.identifiers() {
            generic_identifiers.insert(i.name().to_string());
        }
    }
    let mut generics_map = btreemap!{};
    let mut passed_in = None;
    // figure out generics by guessing
    if let Some(type_info) = type_info {
        passed_in = Some(type_info.passed_in.clone());
        if type_info.passed_in.contains_generics() {
            if !callable_variant.pipeline_input.as_ref().unwrap().contains_generics() {
                passed_in = Some(callable_variant.pipeline_input.as_ref().unwrap().clone());
            }
        } else {
            if let Some(pipeline_input) = &callable_variant.pipeline_input {
                if pipeline_input.contains_generics() {
                    guess_extend_and_check(
                        callable_span,
                        callable_variant,
                        pipeline_input,
                        passed_in.as_ref().unwrap(),
                        &mut generics_map,
                        keywords_map,
                        &mut errors,
                        &mut matched,
                        context,
                    );
                }
            }
        }
    }
    // test input type matching
    if let Some(pipeline_input) = &callable_variant.pipeline_input {
        let expected = pipeline_input.replace_keywords(keywords_map).replace_generics(&generics_map);
        let found = passed_in.as_ref().unwrap().replace_generics(&generics_map).replace_keywords(keywords_map);
        if !expected.is_undetermined() && !expected.test(&found) {
            errors.push(context.generate_diagnostics_error(callable_span, format!("unexpected pipeline input: expect {expected}, found {found}")));
        }
    }
    // normal process handling
    if let Some(argument_list_declaration) = callable_variant.argument_list_declaration {
        let mut declaration_names: Vec<&str> = argument_list_declaration.argument_declarations().map(|d| d.name().name()).collect();
        // match named arguments
        if let Some(argument_list) = argument_list {
            for named_argument in argument_list.arguments().filter(|a| a.name.is_some()) {
                if let Some(argument_declaration) = argument_list_declaration.get(named_argument.name().unwrap().name()) {
                    let desired_type_original = argument_declaration.type_expr().resolved();
                    let mut desired_type = flatten_field_type_reference(desired_type_original.replace_keywords(keywords_map).replace_generics(&generics_map), context);
                    resolve_expression(named_argument.value(), context, &desired_type, keywords_map);
                    if !desired_type.test(named_argument.value().resolved().r#type()) {
                        if !desired_type.is_undetermined() && !named_argument.value().resolved().r#type.is_undetermined() {
                            errors.push(context.generate_diagnostics_error(named_argument.value().span(), format!("expect {}, found {}", desired_type, named_argument.value().resolved().r#type())))
                        }
                    } else {
                        if desired_type.is_field_name() {
                            desired_type = named_argument.value().resolved().r#type().clone();
                        }
                        if desired_type_original.is_generic_item() && (desired_type.is_synthesized_enum_reference() || desired_type.is_synthesized_enum() || desired_type.is_field_name() || desired_type.is_shape_field()) {
                            generics_map.insert(desired_type_original.as_generic_item().unwrap().to_owned(), named_argument.value().resolved().r#type.clone());
                            // generics constraint checking
                            let generics_constraint_checking_result = validate_generics_map_with_constraint_info(callable_span, &generics_map, keywords_map, &callable_variant.generics_constraints, context);
                            for e in generics_constraint_checking_result.0 {
                                errors.push(e);
                            }
                            if !matched {
                                matched = generics_constraint_checking_result.1;
                            }
                        } else if desired_type_original.contains_generics() && desired_type.contains_generics() {
                            guess_extend_and_check(
                                callable_span,
                                callable_variant,
                                &desired_type,
                                named_argument.value().resolved().r#type(),
                                &mut generics_map,
                                keywords_map,
                                &mut errors,
                                &mut matched,
                                context,
                            );
                        }
                    }
                    named_argument.resolve(ArgumentResolved {
                        name: named_argument.name().unwrap().name.clone().to_string(),
                        expect: desired_type.replace_generics(&generics_map),
                        completion_expect: if desired_type_original.is_generic_item() && desired_type.is_field_name() {
                            figure_out_constraint_type_for_field_name(callable_variant, &desired_type_original, &generics_map)
                        } else {
                            None
                        },
                    });
                    declaration_names = declaration_names.iter().filter(|d| (**d) != argument_declaration.name().name()).map(|s| *s).collect();
                } else {
                    let undetermined = Type::Undetermined;
                    resolve_expression(named_argument.value(), context, &undetermined, keywords_map);
                    errors.push(context.generate_diagnostics_error(named_argument.name().unwrap().span, "undefined argument"))
                }
            }
        }
        // remove named optional declarations and fire errors for named required declarations
        for name in declaration_names.clone() {
            if let Some(argument_declaration) = argument_list_declaration.get(name) {
                if !argument_declaration.name_optional {
                    if !argument_declaration.type_expr().resolved().is_optional() {
                        errors.push(context.generate_diagnostics_error(callable_span, format!("missing argument '{}'", name)));
                    }
                    declaration_names = declaration_names.iter().filter(|d| (**d) != argument_declaration.name().name()).map(|s| *s).collect();
                }
            }
        }
        // match unnamed arguments
        if let Some(argument_list) = argument_list {
            for unnamed_argument in argument_list.arguments().filter(|a| a.name.is_none()) {
                if let Some(name) = declaration_names.first() {
                    if let Some(argument_declaration) = argument_list_declaration.get(name) {
                        let desired_type_original = argument_declaration.type_expr().resolved();
                        let mut desired_type = flatten_field_type_reference(desired_type_original.replace_keywords(keywords_map).replace_generics(&generics_map), context);

                        resolve_expression(unnamed_argument.value(), context, &desired_type, keywords_map);
                        if !desired_type.test(unnamed_argument.value().resolved().r#type()) {
                            if !desired_type.is_undetermined() && !unnamed_argument.value().resolved().r#type().is_undetermined() {
                                errors.push(context.generate_diagnostics_error(unnamed_argument.value().span(), format!("expect {}, found {}", desired_type, unnamed_argument.value().resolved().r#type())))
                            }
                        } else {
                            if desired_type.is_field_name() {
                                desired_type = unnamed_argument.value().resolved().r#type().clone();
                            }
                            if desired_type_original.is_generic_item() && (desired_type.is_synthesized_enum_reference() || desired_type.is_synthesized_enum() || desired_type.is_field_name() || desired_type.is_shape_field()) {
                                generics_map.insert(desired_type_original.as_generic_item().unwrap().to_owned(), unnamed_argument.value().resolved().r#type().clone());
                                // generics constraint checking
                                let generics_constraint_checking_result = validate_generics_map_with_constraint_info(callable_span, &generics_map, keywords_map, &callable_variant.generics_constraints, context);
                                for e in generics_constraint_checking_result.0 {
                                    errors.push(e);
                                }
                                if !matched {
                                    matched = generics_constraint_checking_result.1;
                                }
                            } else if desired_type_original.contains_generics() && desired_type.contains_generics() {
                                guess_extend_and_check(
                                    callable_span,
                                    callable_variant,
                                    &desired_type,
                                    unnamed_argument.value().resolved().r#type(),
                                    &mut generics_map,
                                    keywords_map,
                                    &mut errors,
                                    &mut matched,
                                    context,
                                );
                            }
                        }
                        unnamed_argument.resolve(ArgumentResolved {
                            name: name.to_string(),
                            expect: desired_type.replace_generics(&generics_map),
                            completion_expect: if desired_type_original.is_generic_item() && desired_type.is_field_name() {
                                figure_out_constraint_type_for_field_name(callable_variant, &desired_type_original, &generics_map)
                            } else {
                                None
                            },
                        });
                        declaration_names = declaration_names.iter().filter(|d| *d != name).map(|s| *s).collect();
                    }
                } else {
                    errors.push(context.generate_diagnostics_error(unnamed_argument.span, "redundant argument"));
                }
            }
        }
        // fire errors for required unnamed declarations
        for declaration_name in declaration_names {
            if let Some(argument_declaration) = argument_list_declaration.get(declaration_name) {
                if !argument_declaration.type_expr().resolved().is_optional() {
                    errors.push(context.generate_diagnostics_error(callable_span, format!("missing argument '{}'", declaration_name)));
                }
            }
        }
    } else {
        if let Some(argument_list) = argument_list {
            if !argument_list.arguments.is_empty() {
                errors.push(context.generate_diagnostics_error(argument_list.span, "callable requires no arguments"));
            }
        }
    }
    (errors, warnings, callable_variant.pipeline_output.clone().map(|t| flatten_field_type_reference(t.replace_keywords(keywords_map).replace_generics(&generics_map), context)), matched)
}

fn guess_generics_by_pipeline_input_and_passed_in<'a>(unresolved: &'a Type, explicit: &'a Type) -> Result<BTreeMap<String, Type>, String> {
    if !unresolved.contains_generics() && !explicit.contains_generics() {
        return Ok(btreemap! {})
    }
    let mut unresolved = unresolved;
    let mut explicit = explicit;
    // direct match
    if let Some(identifier) = unresolved.as_generic_item() {
        return Ok(btreemap!{identifier.to_string() => explicit.clone()})
    }
    // unwrap optional
    if let Some(inner) = unresolved.as_optional() {
        unresolved = inner;
        if explicit.is_optional() {
            explicit = explicit.unwrap_optional();
        }
    }
    if let Some(identifier) = unresolved.as_generic_item() {
        return Ok(btreemap!{identifier.to_string() => explicit.clone()})
    }
    // unwrap in types
    if unresolved.is_array() && explicit.is_array() {
        return guess_generics_by_pipeline_input_and_passed_in(unresolved.as_array().unwrap(), explicit.as_array().unwrap());
    } else if unresolved.is_dictionary() && explicit.is_dictionary() {
        return guess_generics_by_pipeline_input_and_passed_in(unresolved.as_dictionary().unwrap(), explicit.as_dictionary().unwrap());
    } else if unresolved.is_pipeline() && explicit.is_pipeline() {
        let mut result = btreemap! {};
        result.extend(guess_generics_by_pipeline_input_and_passed_in(unresolved.as_pipeline().unwrap().0, explicit.as_pipeline().unwrap().0)?);
        result.extend(guess_generics_by_pipeline_input_and_passed_in(unresolved.as_pipeline().unwrap().1, explicit.as_pipeline().unwrap().1)?);
        return Ok(result);
    }
    Err(format!("cannot resolve generics: unresolved: {}, explicit: {}", unresolved, explicit))
}

fn validate_generics_map_with_constraint_info<'a>(
    span: Span,
    generics_map: &BTreeMap<String, Type>,
    keywords_map: &BTreeMap<Keyword, Type>,
    generics_constraints: &Vec<&GenericsConstraint>,
    context: &'a ResolverContext<'a>,
) -> (Vec<DiagnosticsError>, bool) {
    let mut matched = false;
    let mut results = vec![];
    for (name, t) in generics_map {
        for constraint in generics_constraints {
            for item in constraint.items() {
                if item.identifier().name() == name {
                    let mut generics_map_without_name = generics_map.clone();
                    generics_map_without_name.remove(name);
                    let (test_result, argument_satisfy) = item.type_expr().resolved().replace_generics(&generics_map_without_name).replace_keywords(keywords_map).constraint_test(t, context.schema);
                    if argument_satisfy {
                        matched = true;
                    }
                    if !test_result {
                        if argument_satisfy {
                            context.insert_diagnostics_error(span, format!("type {} doesn't satisfy {}", t, item.type_expr().resolved().replace_generics(&generics_map_without_name).replace_keywords(keywords_map)));
                        } else {
                            results.push(context.generate_diagnostics_error(span, format!("type {} doesn't satisfy {}", t, item.type_expr().resolved())))
                        }
                    }
                }
            }
        }
    }
    (results, matched)
}

fn guess_generics_by_constraints<'a>(
    generics_map: &BTreeMap<String, Type>,
    keywords_map: &BTreeMap<Keyword, Type>,
    generics_constraints: &Vec<&GenericsConstraint>,
) -> BTreeMap<String, Type> {
    let mut retval = btreemap! {};
    for constraint in generics_constraints {
        for item in constraint.items() {
            if !generics_map.contains_key(item.identifier().name()) {
                // special handles
                if item.type_expr().resolved().is_synthesized_enum_reference() {
                    retval.insert(item.identifier().name.clone(), Type::FieldName("".to_owned()));
                } else if item.type_expr().resolved().is_shape_field() {
                    retval.insert(item.identifier().name.clone(), Type::FieldName("".to_owned()));
                } else {
                    // normal handle
                    let new_type = item.type_expr().resolved().replace_keywords(keywords_map).replace_generics(generics_map).flatten();
                    if !new_type.contains_generics() {
                        retval.insert(item.identifier().name.clone(), new_type);
                    }
                }
            }
        }
    }
    retval
}

fn flatten_field_type_reference<'a>(t: Type, context: &'a ResolverContext<'a>) -> Type {
    t.replace_field_type(|container: &Type, reference: &Type| {
        if let Some(field_name) = reference.as_field_name() {
            match container {
                Type::ModelObject(reference) => {
                    let model = context.schema.find_top_by_path(reference.path()).unwrap().as_model().unwrap();
                    if let Some(field) = model.fields().find(|f| f.identifier().name() == field_name) {
                        field.type_expr().resolved().clone()
                    } else {
                        Type::Undetermined
                    }
                },
                Type::InterfaceObject(reference, types) => {
                    let interface = context.schema.find_top_by_path(reference.path()).unwrap().as_interface_declaration().unwrap();
                    let shape = interface.shape_from_generics(types);
                    shape.get(field_name).cloned().unwrap_or(Type::Undetermined)
                },
                Type::SynthesizedShape(shape) => {
                    shape.get(field_name).cloned().unwrap_or(Type::Undetermined)
                },
                Type::SynthesizedShapeReference(shape_reference) => {
                    if let Some(shape) = shape_reference.fetch_synthesized_definition(context.schema) {
                        flatten_field_type_reference(shape.clone(), context)
                    } else {
                        Type::Undetermined
                    }
                },
                Type::DeclaredSynthesizedShape(reference, inner) => {
                    if let Some(model_reference) = inner.as_model_object() {
                        let model = context.schema.find_top_by_path(model_reference.path()).unwrap().as_model().unwrap();
                        if let Some(shape) = model.resolved().declared_shapes.get(reference.string_path()) {
                            shape.get(field_name).cloned().unwrap_or(Type::Undetermined)
                        } else {
                            Type::Undetermined
                        }
                    } else {
                        Type::Undetermined
                    }
                }
                _ => Type::Undetermined
            }
        } else {
            Type::Undetermined
        }
    })
}

fn guess_extend_and_check<'a>(
    callable_span: Span,
    callable_variant: &CallableVariant,
    unresolved: &Type,
    explicit: &Type,
    generics_map: &mut BTreeMap<String, Type>,
    keywords_map: &BTreeMap<Keyword, Type>,
    errors: &mut Vec<DiagnosticsError>,
    matched: &mut bool,
    context: &'a ResolverContext<'a>,
) {
    match guess_generics_by_pipeline_input_and_passed_in(unresolved, explicit) {
        Ok(map) => {
            generics_map.extend(map);
        },
        Err(err) => {
            errors.push(context.generate_diagnostics_error(callable_span, err));
        }
    }
    // generics constraint checking
    let generics_constraint_checking_result = validate_generics_map_with_constraint_info(callable_span, &generics_map, keywords_map, &callable_variant.generics_constraints, context);
    for e in generics_constraint_checking_result.0 {
        errors.push(e);
    }
    if !*matched {
        *matched = generics_constraint_checking_result.1;
    }
    // guessing more by constraints
    generics_map.extend(guess_generics_by_constraints(&generics_map, keywords_map, &callable_variant.generics_constraints));
}

fn matched_callable_variants<'a, 'b>(callable_variants: &'b Vec<CallableVariant<'a>>, argument_list: Option<&'a ArgumentList>) -> Vec<&'b CallableVariant<'a>> {
    if let Some(argument_list) = argument_list {
        let mut occurred_names = btreeset!{};
        for argument in argument_list.arguments() {
            if let Some(name) = argument.name() {
                occurred_names.insert(name.name());
            }
        }
        callable_variants.iter().filter(|v| {
            if let Some(argument_list_declaration) = v.argument_list_declaration {
                let names: BTreeSet<&str> = argument_list_declaration.argument_declarations().map(|d| d.name().name()).collect();
                let result: Vec<&&str> = occurred_names.difference(&names).collect();
                result.is_empty()
            } else {
                false
            }
        }).collect()
    } else {
        callable_variants.iter().filter(|f| f.argument_list_declaration.is_none() || f.argument_list_declaration.unwrap().every_argument_is_optional()).collect()
    }
}

fn figure_out_constraint_type_for_field_name(callable_variant: &CallableVariant, t: &Type, generics_map: &BTreeMap<String, Type>) -> Option<Type> {
    let gen = t.as_generic_item().unwrap();
    for constraints in &callable_variant.generics_constraints {
        if let Some(item) = constraints.items().find(|i| i.identifier().name() == gen) {
            return Some(item.type_expr().resolved().replace_generics(generics_map).clone());
        }
    }
    None
}