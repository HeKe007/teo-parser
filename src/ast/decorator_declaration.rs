use crate::ast::argument_declaration::ArgumentListDeclaration;
use crate::ast::availability::Availability;
use crate::ast::callable_variant::CallableVariant;
use crate::ast::comment::Comment;
use crate::ast::generics::{GenericsConstraint, GenericsDeclaration};
use crate::ast::identifier::Identifier;
use crate::ast::reference_space::ReferenceSpace;
use crate::ast::span::Span;

#[derive(Debug)]
pub struct DecoratorDeclaration {
    pub span: Span,
    pub path: Vec<usize>,
    pub string_path: Vec<String>,
    pub define_availability: Availability,
    pub comment: Option<Comment>,
    pub exclusive: bool,
    pub unique: bool,
    pub decorator_class: ReferenceSpace,
    pub identifier: Identifier,
    pub generics_declaration: Option<GenericsDeclaration>,
    pub argument_list_declaration: Option<ArgumentListDeclaration>,
    pub generics_constraint: Option<GenericsConstraint>,
    pub variants: Vec<DecoratorDeclarationVariant>,
}

impl DecoratorDeclaration {

    pub fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub fn id(&self) -> usize {
        *self.path.last().unwrap()
    }

    pub fn str_path(&self) -> Vec<&str> {
        self.string_path.iter().map(AsRef::as_ref).collect()
    }

    pub fn namespace_str_path(&self) -> Vec<&str> {
        self.string_path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }

    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    pub fn callable_variants(&self) -> Vec<CallableVariant> {
        if self.has_variants() {
            self.variants.iter().map(|v| CallableVariant {
                generics_declarations: if let Some(generics_declaration) = v.generics_declaration.as_ref() {
                    vec![generics_declaration]
                } else {
                    vec![]
                },
                argument_list_declaration: v.argument_list_declaration.as_ref(),
                generics_constraints: if let Some(generics_constraint) = v.generics_constraint.as_ref() {
                    vec![generics_constraint]
                } else {
                    vec![]
                },
                pipeline_input: None,
                pipeline_output: None,
            }).collect()
        } else {
            vec![CallableVariant {
                generics_declarations: if let Some(generics_declaration) = self.generics_declaration.as_ref() {
                    vec![generics_declaration]
                } else {
                    vec![]
                },
                argument_list_declaration: self.argument_list_declaration.as_ref(),
                generics_constraints: if let Some(generics_constraint) = self.generics_constraint.as_ref() {
                    vec![generics_constraint]
                } else {
                    vec![]
                },
                pipeline_input: None,
                pipeline_output: None,
            }]
        }
    }
}

#[derive(Debug)]
pub struct DecoratorDeclarationVariant {
    pub span: Span,
    pub comment: Option<Comment>,
    pub generics_declaration: Option<GenericsDeclaration>,
    pub argument_list_declaration: Option<ArgumentListDeclaration>,
    pub generics_constraint: Option<GenericsConstraint>,
}
