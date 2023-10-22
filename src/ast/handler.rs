use std::cell::RefCell;
use crate::ast::availability::Availability;
use crate::ast::comment::Comment;
use crate::ast::decorator::Decorator;
use crate::ast::type_expr::{TypeExpr, TypeShape};
use crate::ast::identifier::Identifier;
use crate::ast::span::Span;

#[derive(Debug)]
pub struct HandlerGroupDeclaration {
    pub(crate) span: Span,
    pub(crate) path: Vec<usize>,
    pub(crate) string_path: Vec<String>,
    pub comment: Option<Comment>,
    pub identifier: Identifier,
    pub(crate) handler_declarations: Vec<HandlerDeclaration>,
    pub(crate) define_availability: Availability,
}

impl HandlerGroupDeclaration {

    pub fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }

    pub fn namespace_str_path(&self) -> Vec<&str> {
        self.string_path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }
}

#[derive(Debug)]
pub struct HandlerDeclaration {
    pub(crate) span: Span,
    pub(crate) path: Vec<usize>,
    pub string_path: Vec<String>,
    pub comment: Option<Comment>,
    pub decorators: Vec<Decorator>,
    pub(crate) empty_decorators_spans: Vec<Span>,
    pub identifier: Identifier,
    pub(crate) input_type: TypeExpr,
    pub(crate) output_type: TypeExpr,
    pub(crate) input_format: HandlerInputFormat,
    pub(crate) resolved: RefCell<Option<HandlerDeclarationResolved>>,
}

impl HandlerDeclaration {

    pub fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }

    pub fn namespace_str_path(&self) -> Vec<&str> {
        self.string_path.iter().rev().skip(2).rev().map(AsRef::as_ref).collect()
    }

    pub(crate) fn handler_group_id(&self) -> usize {
        *self.path.get(self.path.len() - 2).unwrap()
    }

    pub(crate) fn resolve(&self, resolved: HandlerDeclarationResolved) {
        *(unsafe { &mut *self.resolved.as_ptr() }) = Some(resolved);
    }

    pub(crate) fn resolved(&self) -> &HandlerDeclarationResolved {
        (unsafe { &*self.resolved.as_ptr() }).as_ref().unwrap()
    }

    pub(crate) fn is_resolved(&self) -> bool {
        self.resolved.borrow().is_some()
    }
}

#[derive(Debug)]
pub(crate) enum HandlerInputFormat {
    Json,
    Form,
}

impl HandlerInputFormat {

    pub(crate) fn is_json(&self) -> bool {
        match self {
            HandlerInputFormat::Json => true,
            _ => false,
        }
    }

    pub(crate) fn is_form(&self) -> bool {
        match self {
            HandlerInputFormat::Form => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct HandlerDeclarationResolved {
    pub(crate) input_shape: TypeShape,
    pub(crate) output_shape: TypeShape,
}