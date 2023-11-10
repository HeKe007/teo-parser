use std::sync::Arc;
use crate::ast::availability::Availability;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::ast::top::Top;
use crate::r#type::reference::Reference;
use crate::r#type::Type;

pub fn search_identifier_path_names_with_filter(
    identifier_path_names: &Vec<&str>,
    schema: &Schema,
    source: &Source,
    namespace_str_path: &Vec<&str>,
    filter: &Arc<dyn Fn(&Top) -> bool>,
    availability: Availability,
) -> Option<Type> {
    let mut used_sources = vec![];
    let reference = search_identifier_path_names_in_source(
        identifier_path_names,
        schema,
        filter,
        source,
        &mut used_sources,
        namespace_str_path,
        availability,
    );
    if reference.is_none() {
        for builtin_source in schema.builtin_sources() {
            if let Some(reference) = search_identifier_path_names_in_source(
                &identifier_path_names,
                schema,
                filter,
                builtin_source,
                &mut used_sources,
                &vec!["std"],
                availability,
            ) {
                return Some(reference);
            }
        }
    }
    reference
}

fn search_identifier_path_names_in_source(
    identifier_path_names: &Vec<&str>,
    schema: &Schema,
    filter: &Arc<dyn Fn(&Top) -> bool>,
    source: &Source,
    used_sources: &mut Vec<usize>,
    ns_str_path: &Vec<&str>,
    availability: Availability,
) -> Option<Type> {
    if used_sources.contains(&source.id) {
        return None;
    }
    used_sources.push(source.id);
    let mut ns_str_path_mut = ns_str_path.clone();
    loop {
        if ns_str_path_mut.is_empty() {
            if let Some(top) = source.find_top_by_string_path(identifier_path_names, filter, availability) {
                return Some(top_to_reference_type(top));
            }
        } else {
            if let Some(ns) = source.find_child_namespace_by_string_path(&ns_str_path_mut) {
                if let Some(top) = ns.find_top_by_string_path(identifier_path_names, filter, availability) {
                    return Some(top_to_reference_type(top));
                }
            }
        }
        if ns_str_path_mut.len() > 0 {
            ns_str_path_mut.pop();
        } else {
            break
        }
    }
    for import in source.imports() {
        // find with imports
        if let Some(from_source) = schema.sources().iter().find(|source| {
            import.file_path.as_str() == source.file_path.as_str()
        }).map(|s| *s) {
            if let Some(found) = search_identifier_path_names_in_source(identifier_path_names, schema, filter, from_source, used_sources, &ns_str_path, availability) {
                return Some(found)
            }
        }
    }
    None
}

fn top_to_reference_type(top: &Top) -> Type {
    match top {
        Top::Import(_) => Type::Undetermined,
        Top::Config(c) => Type::ConfigReference(Reference::new(c.path.clone(), c.string_path.clone())),
        Top::ConfigDeclaration(_) => Type::Undetermined,
        Top::Constant(c) => c.resolved().expression_resolved.r#type.clone(),
        Top::Enum(e) => Type::EnumReference(Reference::new(e.path.clone(), e.string_path.clone())),
        Top::Model(m) => Type::ModelReference(Reference::new(m.path.clone(), m.string_path.clone())),
        Top::DataSet(d) => Type::DataSetReference(d.string_path.clone()),
        Top::Middleware(m) => Type::MiddlewareReference(Reference::new(m.path.clone(), m.string_path.clone())),
        Top::HandlerGroup(_) => Type::Undetermined,
        Top::Interface(i) => if i.generics_declaration.is_none() {
            Type::InterfaceReference(Reference::new(i.path.clone(), i.string_path.clone()), vec![])
        } else {
            Type::Undetermined
        },
        Top::Namespace(n) => Type::NamespaceReference(n.string_path.clone()),
        Top::DecoratorDeclaration(_) => Type::Undetermined,
        Top::PipelineItemDeclaration(_) => Type::Undetermined,
        Top::StructDeclaration(s) => if s.generics_declaration.is_none() {
            Type::StructReference(Reference::new(s.path.clone(), s.string_path.clone()), vec![])
        } else {
            Type::Undetermined
        }
        Top::UseMiddlewareBlock(_) => Type::Undetermined,
    }
