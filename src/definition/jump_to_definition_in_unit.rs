use crate::ast::reference::ReferenceType;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::ast::top::Top;
use crate::ast::unit::Unit;
use crate::definition::definition::Definition;
use crate::definition::jump_to_definition_in_expression::jump_to_definition_in_expression;
use crate::r#type::r#type::Type;
use crate::search::search_unit::search_unit;
use crate::utils::top_filter::top_filter_for_reference_type;

pub(super) fn jump_to_definition_in_unit<'a>(
    schema: &'a Schema,
    source: &'a Source,
    unit: &'a Unit,
    namespace_path: &Vec<&'a str>,
    line_col: (usize, usize),
    expect: &Type,
) -> Vec<Definition> {
    if unit.expressions.len() == 1 {
        jump_to_definition_in_expression(
            schema,
            source,
            unit.expressions.get(0).unwrap(),
            namespace_path,
            line_col,
            expect,
        )
    } else {
        search_unit(
            schema,
            source,
            unit,
            namespace_path,
            line_col,
            |argument_list, callable_container_path, callable_name| {

                vec![]
            },
            |subscript| {
                if subscript.expression.span().contains_line_col(line_col) {
                    let exp = Type::Undetermined;
                    jump_to_definition_in_expression(
                        schema,
                        source,
                        subscript.expression.as_ref(),
                        namespace_path,
                        line_col,
                        &exp,
                    )
                } else {
                    vec![]
                }
            },
            |span, identifier_container_path, identifier_name| {
                let top = schema.find_top_by_path(identifier_container_path).unwrap();
                match top {
                    Top::Config(config) => if let Some(identifier) = identifier_name {
                        let item = config.items.iter().find(|i| i.identifier.name() == identifier).unwrap();
                        vec![Definition {
                            path: schema.source(config.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: item.span,
                            identifier_span: item.identifier.span,
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(config.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: config.span,
                            identifier_span: config.identifier.as_ref().map_or(config.keyword.span, |i| i.span),
                        }]
                    },
                    Top::ConfigDeclaration(config_declaration) => if let Some(identifier) = identifier_name {
                        let item = config_declaration.fields.iter().find(|i| i.identifier.name() == identifier).unwrap();
                        vec![Definition {
                            path: schema.source(config_declaration.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: item.span,
                            identifier_span: item.identifier.span,
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(config_declaration.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: config_declaration.span,
                            identifier_span: config_declaration.identifier.span,
                        }]
                    },
                    Top::Constant(constant) => vec![Definition {
                        path: schema.source(constant.source_id()).unwrap().file_path.clone(),
                        selection_span: span,
                        target_span: constant.span,
                        identifier_span: constant.identifier.span,
                    }],
                    Top::Enum(r#enum) => if let Some(identifier) = identifier_name {
                        let member = r#enum.members.iter().find(|m| m.identifier.name() == identifier).unwrap();
                        vec![Definition {
                            path: schema.source(member.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: member.span,
                            identifier_span: member.identifier.span,
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(r#enum.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: r#enum.span,
                            identifier_span: r#enum.identifier.span,
                        }]
                    },
                    Top::Model(model) => if let Some(identifier) = identifier_name {
                        let field = model.fields.iter().find(|i| i.identifier.name() == identifier).unwrap();
                        vec![Definition {
                            path: schema.source(field.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: field.span,
                            identifier_span: field.identifier.span,
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(model.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: model.span,
                            identifier_span: model.identifier.span,
                        }]
                    },
                    Top::Interface(interface) => if let Some(identifier) = identifier_name {
                        let field = interface.fields.iter().find(|i| i.identifier.name() == identifier).unwrap();
                        vec![Definition {
                            path: schema.source(field.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: field.span,
                            identifier_span: field.identifier.span,
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(interface.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: interface.span,
                            identifier_span: interface.identifier.span,
                        }]
                    }
                    Top::Namespace(namespace) => if let Some(identifier) = identifier_name {
                        let top = namespace.find_top_by_name(identifier, &top_filter_for_reference_type(ReferenceType::Default)).unwrap();
                        vec![Definition {
                            path: schema.source(top.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: top.span(),
                            identifier_span: top.identifier_span().unwrap(),
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(namespace.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: namespace.span,
                            identifier_span: namespace.identifier.span,
                        }]
                    }
                    Top::StructDeclaration(struct_declaration) => if let Some(identifier) = identifier_name {
                        let method = struct_declaration.function_declarations.iter().find(|f| f.identifier.name() == identifier).unwrap();
                        vec![Definition {
                            path: schema.source(method.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: method.span,
                            identifier_span: method.identifier.span,
                        }]
                    } else {
                        vec![Definition {
                            path: schema.source(struct_declaration.source_id()).unwrap().file_path.clone(),
                            selection_span: span,
                            target_span: struct_declaration.span,
                            identifier_span: struct_declaration.identifier.span,
                        }]
                    },
                    _ => vec![]
                }
            },
            vec![]
        )
    }
}