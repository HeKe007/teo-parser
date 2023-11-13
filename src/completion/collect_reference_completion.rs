use std::sync::Arc;
use crate::availability::Availability;
use crate::ast::namespace::Namespace;
use crate::ast::schema::Schema;
use crate::ast::source::Source;

pub(super) fn collect_reference_completion_in_source(schema: &Schema, source: &Source, namespace_path: &Vec<&str>, user_typed_prefix: &Vec<&str>, filter: &Arc<dyn Fn(&Node) -> bool>, availability: Availability) -> Vec<Vec<usize>> {
    let mut examined_sources = vec![];
    collect_reference_completion_in_source_internal(schema, source, namespace_path, user_typed_prefix, filter, &mut examined_sources, availability)
}

fn collect_reference_completion_in_source_internal<'a>(schema: &'a Schema, source: &'a Source, namespace_path: &Vec<&str>, user_typed_prefix: &Vec<&str>, filter: &Arc<dyn Fn(&Node) -> bool>, examined_sources: &mut Vec<&'a str>, availability: Availability) -> Vec<Vec<usize>> {
    examined_sources.push(&source.file_path);
    let mut result = vec![];
    let mut namespace_path_mut = namespace_path.clone();
    loop {
        let mut combined = namespace_path_mut.clone();
        combined.extend(user_typed_prefix);
        if let Some(namespace) = source.find_child_namespace_by_string_path(&combined) {
            result.extend(collect_reference_completion_in_namespace(namespace, filter));
            namespace_path_mut.pop();
        }
        break
    }
    for top in source.children() {
        if let Some(namespace) = top.as_namespace() {
            if namespace.tops().iter().find(|t| filter(t)).is_some() {
                result.push(namespace.path.clone());
            }
        } else if let Some(import) = top.as_import() {
            if !examined_sources.contains(&import.file_path.as_str()) {
                if let Some(source) = schema.source_at_path(import.file_path.as_str()) {
                    result.extend(collect_reference_completion_in_source_internal(schema, source, namespace_path, user_typed_prefix, filter, examined_sources, availability));
                }
            }
        } else {
            if filter(top) {
                result.push(top.path().clone())
            }
        }
    }
    for builtin_source in schema.builtin_sources() {
        if !examined_sources.contains(&builtin_source.file_path.as_str()) {
            result.extend(collect_reference_completion_in_source_internal(schema, builtin_source, namespace_path, user_typed_prefix, filter, examined_sources, availability));
            if let Some(namespace) = builtin_source.find_child_namespace_by_string_path(&vec!["std"]) {
                result.extend(collect_reference_completion_in_namespace(namespace, filter));
            }
        }
    }
    result
}

fn collect_reference_completion_in_namespace(namespace: &Namespace, filter: &Arc<dyn Fn(&Node) -> bool>) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for top in namespace.tops() {
        if let Some(namespace) = top.as_namespace() {
            if namespace.tops().iter().find(|t| filter(t)).is_some() {
                result.push(namespace.path.clone());
            }
        } else {
            if filter(top) {
                result.push(top.path().clone())
            }
        }
    }
    result
}