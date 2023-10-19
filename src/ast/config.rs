use std::cell::RefCell;
use crate::ast::availability::Availability;
use crate::ast::config_item::ConfigItem;
use crate::ast::config_keyword::ConfigKeyword;
use crate::ast::expr::Expression;
use crate::ast::identifier::Identifier;
use crate::ast::info_provider::InfoProvider;
use crate::ast::span::Span;

#[derive(Debug)]
pub struct Config {
    pub span: Span,
    pub(crate) path: Vec<usize>,
    pub(crate) string_path: Vec<String>,
    pub define_availability: Availability,
    pub(crate) keyword: ConfigKeyword,
    pub identifier: Option<Identifier>,
    pub items: Vec<ConfigItem>,
    pub(crate) resolved: RefCell<Option<ConfigResolved>>,
}

impl Config {

    pub(crate) fn source_id(&self) -> usize {
        *self.path.first().unwrap()
    }

    pub(crate) fn id(&self) -> usize {
        *self.path.last().unwrap()
    }

    pub fn name(&self) -> &str {
        if let Some(identifier) = &self.identifier {
            identifier.name()
        } else {
            self.keyword.name()
        }
    }

    pub fn name_span(&self) -> Span {
        if let Some(identifier) = &self.identifier {
            identifier.span
        } else {
            self.keyword.span
        }
    }

    pub fn get_item(&self, name: impl AsRef<str>) -> Option<&Expression> {
        self.items.iter().find(|item| item.identifier.name() == name.as_ref() && item.is_available()).map(|item| &item.expression)
    }

    pub fn is_available(&self) -> bool {
        self.define_availability.contains(self.resolved().actual_availability)
    }

    pub(crate) fn resolve(&self, resolved: ConfigResolved) {
        *(unsafe { &mut *self.resolved.as_ptr() }) = Some(resolved);
    }

    pub(crate) fn resolved(&self) -> &ConfigResolved {
        (unsafe { &*self.resolved.as_ptr() }).as_ref().unwrap()
    }

    pub(crate) fn is_resolved(&self) -> bool {
        self.resolved.borrow().is_some()
    }
}

#[derive(Debug)]
pub(crate) struct ConfigResolved {
    pub(crate) actual_availability: Availability
}

impl InfoProvider for Config {

    fn namespace_str_path(&self) -> Vec<&str> {
        self.string_path.iter().rev().skip(1).rev().map(AsRef::as_ref).collect()
    }

    fn availability(&self) -> Availability {
        self.define_availability.bi_and(self.resolved().actual_availability)
    }
}