use std::cell::RefCell;
use crate::value::value::Value;

use crate::ast::identifier::Identifier;
use crate::ast::identifier_path::IdentifierPath;
use crate::ast::literals::DictionaryLiteral;
use crate::{declare_container_node, impl_container_node_defaults, node_child_fn, node_children_iter, node_children_iter_fn, node_optional_child_fn};
use crate::ast::doc_comment::DocComment;
use crate::format::Writer;
use crate::r#type::reference::Reference;
use crate::traits::has_availability::HasAvailability;
use crate::traits::info_provider::InfoProvider;
use crate::traits::resolved::Resolve;
use crate::traits::write::Write;

declare_container_node!(DataSet, named, availability,
    pub(crate) identifier: usize,
    pub auto_seed: bool,
    pub notrack: bool,
    pub(crate) groups: Vec<usize>,
    pub(crate) comment: Option<usize>,
);

impl_container_node_defaults!(DataSet, named, availability);

node_children_iter!(DataSet, DataSetGroup, GroupsIter, groups);

impl DataSet {

    node_optional_child_fn!(comment, DocComment);

    node_child_fn!(identifier, Identifier);

    node_children_iter_fn!(groups, GroupsIter);
}

impl InfoProvider for DataSet {
    fn namespace_skip(&self) -> usize {
        1
    }
}

declare_container_node!(DataSetGroup, named, availability,
    pub(crate) identifier_path: usize,
    pub(crate) records: Vec<usize>,
    pub(crate) resolved: RefCell<Option<Reference>>,
    pub(crate) comment: Option<usize>,
);

impl_container_node_defaults!(DataSetGroup, named, availability);

node_children_iter!(DataSetGroup, DataSetRecord, RecordsIter, records);

impl DataSetGroup {

    node_optional_child_fn!(comment, DocComment);

    node_child_fn!(identifier_path, IdentifierPath);

    node_children_iter_fn!(records, RecordsIter);
}

impl InfoProvider for DataSetGroup {
    fn namespace_skip(&self) -> usize {
        2
    }
}

impl Resolve<Reference> for DataSetGroup {
    fn resolved_ref_cell(&self) -> &RefCell<Option<Reference>> {
        &self.resolved
    }
}

declare_container_node!(DataSetRecord, named, availability,
    pub(crate) identifier: usize,
    pub(crate) dictionary: usize,
    pub(crate) resolved: RefCell<Option<Value>>,
    pub(crate) comment: Option<usize>,
);

impl_container_node_defaults!(DataSetRecord, named, availability);

impl DataSetRecord {

    node_optional_child_fn!(comment, DocComment);

    node_child_fn!(identifier, Identifier);

    node_child_fn!(dictionary, DictionaryLiteral);
}

impl InfoProvider for DataSetRecord {
    fn namespace_skip(&self) -> usize {
        3
    }
}

impl Resolve<Value> for DataSetRecord {
    fn resolved_ref_cell(&self) -> &RefCell<Option<Value>> {
        &self.resolved
    }
}

impl Write for DataSet {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}

impl Write for DataSetGroup {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}

impl Write for DataSetRecord {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }

    fn is_block_level_element(&self) -> bool {
        true
    }
}