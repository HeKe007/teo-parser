use crate::ast::argument_declaration::ArgumentListDeclaration;
use crate::ast::comment::Comment;
use crate::ast::generics::{GenericsConstraint, GenericsDeclaration};
use crate::ast::identifier::Identifier;
use crate::ast::r#type::TypeExpr;
use crate::ast::span::Span;

#[derive(Debug)]
pub(crate) struct FunctionDeclaration {
    pub(crate) span: Span,
    pub(crate) path: Vec<usize>,
    pub(crate) string_path: Vec<String>,
    pub(crate) comment: Option<Comment>,
    pub(crate) r#static: bool,
    pub(crate) identifier: Identifier,
    pub(crate) generics_declaration: Option<GenericsDeclaration>,
    pub(crate) argument_list_declaration: Option<ArgumentListDeclaration>,
    pub(crate) generics_constraint: Option<GenericsConstraint>,
    pub(crate) return_type: TypeExpr,
}

impl FunctionDeclaration {

    pub(crate) fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }
}