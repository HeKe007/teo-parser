use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use crate::ast::arity::Arity;
use crate::ast::identifier_path::IdentifierPath;
use crate::ast::literals::EnumVariantLiteral;
use crate::ast::span::Span;
use crate::{declare_container_node, impl_container_node_defaults, impl_node_defaults, node_child_fn, node_children_iter, node_children_iter_fn, node_optional_child_fn};
use crate::ast::identifier::Identifier;
use crate::ast::node::Node;
use crate::format::Writer;
use crate::r#type::r#type::Type;
use crate::traits::identifiable::Identifiable;
use crate::traits::node_trait::NodeTrait;
use crate::traits::resolved::Resolve;
use crate::traits::write::Write;

#[derive(Debug, Clone, Copy)]
pub enum TypeOperator {
    BitOr,
}

impl Display for TypeOperator {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeOperator::BitOr => f.write_str("|"),
        }
    }
}

declare_container_node!(TypeBinaryOperation,
    pub(crate) lhs: usize,
    pub op: TypeOperator,
    pub(crate) rhs: usize,
);

impl_container_node_defaults!(TypeBinaryOperation);

impl TypeBinaryOperation {

    node_child_fn!(lhs, TypeExpr);

    node_child_fn!(rhs, TypeExpr);
}

impl Write for TypeBinaryOperation {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypeGroup,
    pub(crate) type_expr: usize,
    pub arity: Arity,
    pub item_optional: bool,
    pub collection_optional: bool,
);

impl_container_node_defaults!(TypeGroup);

impl TypeGroup {

    node_child_fn!(type_expr, TypeExpr);
}

impl Write for TypeGroup {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypeTuple,
    pub(crate) items: Vec<usize>,
    pub arity: Arity,
    pub item_optional: bool,
    pub collection_optional: bool,
);

impl_container_node_defaults!(TypeTuple);

node_children_iter!(TypeTuple, TypeExpr, ItemsIter, items);

impl TypeTuple {

    node_children_iter_fn!(items, ItemsIter);
}

impl Write for TypeTuple {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypeSubscript,
    pub(crate) container: usize,
    pub(crate) argument: usize,
    pub arity: Arity,
    pub item_optional: bool,
    pub collection_optional: bool,
);

impl_container_node_defaults!(TypeSubscript);

impl TypeSubscript {

    node_child_fn!(container, TypeExpr);

    node_child_fn!(argument, TypeExpr);
}

impl Write for TypeSubscript {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypeItem,
    pub(crate) identifier_path: usize,
    pub(crate) generics: Option<usize>,
    pub arity: Arity,
    pub item_optional: bool,
    pub collection_optional: bool,
);

impl_container_node_defaults!(TypeItem);

impl TypeItem {

    node_child_fn!(identifier_path, IdentifierPath);

    node_optional_child_fn!(generics, TypeGenerics);

    pub fn generic_items(&self) -> Vec<&TypeExpr> {
        if let Some(generics) = self.generics() {
            generics.type_exprs().collect()
        } else {
            vec![]
        }
    }
}

impl Write for TypeItem {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypeGenerics,
    pub(crate) type_exprs: Vec<usize>,
);

impl_node_defaults!(TypeGenerics);

node_children_iter!(TypeGenerics, TypeExpr, GenericsIter, type_exprs);

impl TypeGenerics {
    node_children_iter_fn!(type_exprs, GenericsIter);
}

impl Write for TypeGenerics {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypedEnum,
    pub(crate) members: Vec<usize>,
);

impl_container_node_defaults!(TypedEnum);

node_children_iter!(TypedEnum, EnumVariantLiteral, TypedEnumMembersIter, members);

impl TypedEnum {

    node_children_iter_fn!(members, TypedEnumMembersIter);
}

impl Write for TypedEnum {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypedShapeItem,
    pub(crate) identifier: usize,
    pub(crate) type_expr: usize,
);

impl_container_node_defaults!(TypedShapeItem);

impl TypedShapeItem {
    node_child_fn!(identifier, Identifier);
    node_child_fn!(type_expr, TypeExpr);
}

impl Write for TypedShapeItem {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

declare_container_node!(TypedShape,
    pub(crate) items: Vec<usize>,
    pub arity: Arity,
    pub item_optional: bool,
    pub collection_optional: bool,
);

impl_container_node_defaults!(TypedShape);

node_children_iter!(TypedShape, TypedShapeItem, TypedShapeItemsIter, items);

impl TypedShape {

    node_children_iter_fn!(items, TypedShapeItemsIter);
}

impl Write for TypedShape {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        writer.write_children(self, self.children.values());
    }
}

#[derive(Debug)]
pub enum TypeExprKind {
    Expr(Box<TypeExprKind>),
    BinaryOp(TypeBinaryOperation),
    TypeItem(TypeItem),
    TypeGroup(TypeGroup),
    TypeTuple(TypeTuple),
    TypeSubscript(TypeSubscript),
    FieldName(EnumVariantLiteral),
    TypedEnum(TypedEnum),
    TypedShape(TypedShape),
}

impl TypeExprKind {

    pub fn as_dyn_node_trait(&self) -> &dyn NodeTrait {
        match self {
            TypeExprKind::Expr(n) => n.as_ref(),
            TypeExprKind::BinaryOp(n) => n,
            TypeExprKind::TypeItem(n) => n,
            TypeExprKind::TypeGroup(n) => n,
            TypeExprKind::TypeTuple(n) => n,
            TypeExprKind::TypeSubscript(n) => n,
            TypeExprKind::FieldName(n) => n,
            TypeExprKind::TypedEnum(n) => n,
            TypeExprKind::TypedShape(n) => n,
        }
    }

    pub fn is_field_name(&self) -> bool {
        self.as_field_name().is_some()
    }

    pub fn as_field_name(&self) -> Option<&EnumVariantLiteral> {
        match self {
            Self::FieldName(e) => Some(e),
            _ => None,
        }
    }
}

impl Identifiable for TypeExprKind {

    fn path(&self) -> &Vec<usize> {
        self.as_dyn_node_trait().path()
    }
}

impl NodeTrait for TypeExprKind {
    fn span(&self) -> Span {
        self.as_dyn_node_trait().span()
    }

    fn children(&self) -> Option<&BTreeMap<usize, Node>> {
        self.as_dyn_node_trait().children()
    }
}

#[derive(Debug)]
pub struct TypeExpr {
    pub kind: TypeExprKind,
    pub resolved: RefCell<Option<Type>>,
}

impl TypeExpr {
    pub fn new(kind: TypeExprKind) -> Self {
        Self { kind, resolved: RefCell::new(None) }
    }
}

impl Identifiable for TypeExpr {
    fn path(&self) -> &Vec<usize> {
        self.kind.as_dyn_node_trait().path()
    }
}

impl NodeTrait for TypeExpr {
    fn span(&self) -> Span {
        self.kind.as_dyn_node_trait().span()
    }

    fn children(&self) -> Option<&BTreeMap<usize, Node>> {
        self.kind.as_dyn_node_trait().children()
    }
}

impl Resolve<Type> for TypeExpr {
    fn resolved_ref_cell(&self) -> &RefCell<Option<Type>> {
        &self.resolved
    }
}

impl Write for TypeExprKind {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        self.as_dyn_node_trait().write(writer);
    }

    fn write_output_with_default_writer(&self) -> String {
        self.as_dyn_node_trait().write_output_with_default_writer()
    }

    fn prefer_whitespace_before(&self) -> bool {
        self.as_dyn_node_trait().prefer_whitespace_before()
    }

    fn prefer_whitespace_after(&self) -> bool {
        self.as_dyn_node_trait().prefer_whitespace_after()
    }

    fn prefer_always_no_whitespace_before(&self) -> bool {
        self.as_dyn_node_trait().prefer_always_no_whitespace_before()
    }

    fn always_start_on_new_line(&self) -> bool {
        self.as_dyn_node_trait().always_start_on_new_line()
    }

    fn always_end_on_new_line(&self) -> bool {
        self.as_dyn_node_trait().always_end_on_new_line()
    }

    fn is_block_start(&self) -> bool {
        self.as_dyn_node_trait().is_block_start()
    }

    fn is_block_end(&self) -> bool {
        self.as_dyn_node_trait().is_block_end()
    }

    fn is_block_element_delimiter(&self) -> bool {
        self.as_dyn_node_trait().is_block_element_delimiter()
    }

    fn is_block_level_element(&self) -> bool {
        self.as_dyn_node_trait().is_block_level_element()
    }

    fn wrap(&self, content: &str, available_length: usize) -> String {
        self.as_dyn_node_trait().wrap(content, available_length)
    }
}

impl Write for TypeExpr {
    fn write<'a>(&'a self, writer: &mut Writer<'a>) {
        self.kind.as_dyn_node_trait().write(writer);
    }

    fn write_output_with_default_writer(&self) -> String {
        self.kind.as_dyn_node_trait().write_output_with_default_writer()
    }

    fn prefer_whitespace_before(&self) -> bool {
        self.kind.as_dyn_node_trait().prefer_whitespace_before()
    }

    fn prefer_whitespace_after(&self) -> bool {
        self.kind.as_dyn_node_trait().prefer_whitespace_after()
    }

    fn prefer_always_no_whitespace_before(&self) -> bool {
        self.kind.as_dyn_node_trait().prefer_always_no_whitespace_before()
    }

    fn always_start_on_new_line(&self) -> bool {
        self.kind.as_dyn_node_trait().always_start_on_new_line()
    }

    fn always_end_on_new_line(&self) -> bool {
        self.kind.as_dyn_node_trait().always_end_on_new_line()
    }

    fn is_block_start(&self) -> bool {
        self.kind.as_dyn_node_trait().is_block_start()
    }

    fn is_block_end(&self) -> bool {
        self.kind.as_dyn_node_trait().is_block_end()
    }

    fn is_block_element_delimiter(&self) -> bool {
        self.kind.as_dyn_node_trait().is_block_element_delimiter()
    }

    fn is_block_level_element(&self) -> bool {
        self.kind.as_dyn_node_trait().is_block_level_element()
    }

    fn wrap(&self, content: &str, available_length: usize) -> String {
        self.kind.as_dyn_node_trait().wrap(content, available_length)
    }
}

impl Display for TypeExprKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_dyn_node_trait(), f)
    }
}

impl Display for TypeExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl<'a> TryFrom<&'a Node> for &'a TypeExpr {
    type Error = &'static str;

    fn try_from(value: &'a Node) -> Result<Self, Self::Error> {
        match value {
            Node::TypeExpr(n) => Ok(n),
            _ => Err("convert failed"),
        }
    }
}

impl From<TypeExpr> for Node {
    fn from(value: TypeExpr) -> Self {
        Self::TypeExpr(value)
    }
}
