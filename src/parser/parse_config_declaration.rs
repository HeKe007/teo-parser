use crate::ast::config_declaration::ConfigDeclaration;
use crate::{parse_append, parse_build_container_struct, parse_container_node_variables, parse_container_node_variables_cleanup, parse_insert, parse_insert_punctuation, parse_set_optional};
use crate::parser::parse_availability_end::parse_availability_end;
use crate::parser::parse_availability_flag::parse_availability_flag;
use crate::parser::parse_comment::parse_comment;
use crate::parser::parse_field::parse_field;
use crate::parser::parse_identifier::parse_identifier;
use crate::parser::parse_span::parse_span;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};
use crate::traits::identifiable::Identifiable;
use crate::traits::node_trait::NodeTrait;

pub(super) fn parse_config_declaration(pair: Pair<'_>, context: &mut ParserContext) -> ConfigDeclaration {
    parse_container_node_variables!(named);
    let mut comment: Option<usize> = None;
    let mut identifier: usize = 0;
    let mut fields: Vec<usize> = vec![];
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::BLOCK_OPEN => parse_insert_punctuation!("{"),
            Rule::BLOCK_CLOSE => parse_insert_punctuation!("}"),
            Rule::identifier => {
                let node = parse_identifier(&current, context);
                if context.current_string_path() != vec!["std".to_owned()] {
                    context.insert_error(node.span(), "config declarations are builtin and cannot be declared");
                }
                string_path = Some(context.next_parent_string_path(node.name()));
                identifier = node.id();
                children.insert(node.id(), node.into());
            },
            Rule::field_declaration => parse_insert!(parse_field(current, context), fields),
            Rule::triple_comment_block => parse_set_optional!(parse_comment(current, context), comment),
            Rule::availability_start => parse_append!(parse_availability_flag(current, context)),
            Rule::availability_end => parse_append!(parse_availability_end(current, context)),
            Rule::BLOCK_LEVEL_CATCH_ALL => context.insert_unparsed(parse_span(&current)),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    parse_container_node_variables_cleanup!(named);
    parse_build_container_struct!(ConfigDeclaration, named, availability,
        comment: comment,
        identifier: identifier,
        fields: fields,
    );
}
