use crate::ast::handler::{HandlerDeclaration, HandlerGroupDeclaration};
use crate::ast::handler_template_declaration::HandlerTemplateDeclaration;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::definition::definition::Definition;
use crate::definition::jump_to_definition_in_type_expr::jump_to_definition_in_type_expr_kind;
use crate::search::search_availability::search_availability;
use crate::traits::node_trait::NodeTrait;

pub(super) fn jump_to_definition_in_handler_template_declaration(schema: &Schema, source: &Source, handler_template_declaration: &HandlerTemplateDeclaration, line_col: (usize, usize)) -> Vec<Definition> {
    let mut namespace_path: Vec<_> = handler_template_declaration.string_path.iter().map(|s| s.as_str()).collect();
    namespace_path.pop();
    let availability = search_availability(schema, source, &namespace_path);
    if let Some(input_type) = handler_template_declaration.input_type() {
        if input_type.span().contains_line_col(line_col) {
            return jump_to_definition_in_type_expr_kind(
                schema,
                source,
                &input_type.kind,
                &namespace_path,
                line_col,
                &vec![],
                availability
            );
        }
    }
    if handler_template_declaration.output_type().span().contains_line_col(line_col) {
        return jump_to_definition_in_type_expr_kind(
            schema,
            source,
            &handler_template_declaration.output_type().kind,
            &namespace_path,
            line_col,
            &vec![],
            availability
        );
    }
    vec![]
}
