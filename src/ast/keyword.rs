use crate::{declare_node, impl_node_defaults};
use crate::ast::span::Span;
use crate::format::Writer;
use crate::traits::write::Write;

declare_node!(Keyword, pub(crate) name: String);

impl_node_defaults!(Keyword);

impl Write for Keyword {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_content(self, self.name());
    }

    fn prefer_whitespace_after(&self) -> bool {
        self.name() != "variant"
    }
}

impl Keyword {

    pub(crate) fn new(name: impl AsRef<str>, span: Span, path: Vec<usize>) -> Self {
        Self {
            span,
            path,
            name: name.as_ref().to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn is_connector(&self) -> bool {
        self.name.as_str() == "connector"
    }

    pub fn is_server(&self) -> bool {
        self.name.as_str() == "server"
    }

    pub fn is_entity(&self) -> bool {
        self.name.as_str() == "entity"
    }

    pub fn is_client(&self) -> bool {
        self.name.as_str() == "client"
    }

    pub fn is_debug(&self) -> bool {
        self.name.as_str() == "debug"
    }

    pub fn is_admin(&self) -> bool {
        self.name.as_str() == "admin"
    }
}
