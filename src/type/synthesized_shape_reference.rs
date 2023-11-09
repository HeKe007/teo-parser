use std::fmt::{Display, Formatter};
use educe::Educe;
use serde::Serialize;
use crate::r#type::reference::Reference;

#[derive(Debug, Clone, Eq, Serialize)]
#[derive(Educe)]
#[educe(Hash, PartialEq)]
pub enum SynthesizedShapeReference {
    Args(Reference),
    FindManyArgs(Reference),
    FindFirstArgs(Reference),
    FindUniqueArgs(Reference),
    CreateArgs(Reference),
    UpdateArgs(Reference),
    UpsertArgs(Reference),
    CopyArgs(Reference),
    DeleteArgs(Reference),
    CreateManyArgs(Reference),
    UpdateManyArgs(Reference),
    CopyManyArgs(Reference),
    DeleteManyArgs(Reference),
    CountArgs(Reference),
    AggregateArgs(Reference),
    GroupByArgs(Reference),
    RelationFilter(Reference),
    ListRelationFilter(Reference),
    WhereInput(Reference),
    WhereUniqueInput(Reference),
    ScalarFieldEnum(Reference),
    ScalarWhereWithAggregatesInput(Reference),
    CountAggregateInputType(Reference),
    SumAggregateInputType(Reference),
    AvgAggregateInputType(Reference),
    MaxAggregateInputType(Reference),
    MinAggregateInputType(Reference),
    CreateInput(Reference),
    CreateInputWithout(Reference, String),
    CreateNestedOneInput(Reference),
    CreateNestedOneInputWithout(Reference, String),
    CreateNestedManyInput(Reference),
    CreateNestedManyInputWithout(Reference, String),
    UpdateInput(Reference),
    UpdateInputWithout(Reference, String),
    UpdateNestedOneInput(Reference),
    UpdateNestedOneInputWithout(Reference, String),
    UpdateNestedManyInput(Reference),
    UpdateNestedManyInputWithout(Reference, String),
    ConnectOrCreateInput(Reference),
    ConnectOrCreateInputWithout(Reference, String),
    UpdateWithWhereUniqueInput(Reference),
    UpdateWithWhereUniqueInputWithout(Reference, String),
    UpsertWithWhereUniqueInput(Reference),
    UpsertWithWhereUniqueInputWithout(Reference, String),
    UpdateManyWithWhereInput(Reference),
    UpdateManyWithWhereInputWithout(Reference, String),
    Select(Reference),
    Include(Reference),
    OrderByInput(Reference),
    Result(Reference),
    CountAggregateResult(Reference),
    SumAggregateResult(Reference),
    AvgAggregateResult(Reference),
    MinAggregateResult(Reference),
    MaxAggregateResult(Reference),
    AggregateResult(Reference),
    GroupByResult(Reference),
}

impl Display for SynthesizedShapeReference {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SynthesizedShapeReference::Args(re) => f.write_str(&format!("Args<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::FindManyArgs(re) => f.write_str(&format!("FindManyArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::FindFirstArgs(re) => f.write_str(&format!("FindFirstArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::FindUniqueArgs(re) => f.write_str(&format!("FindUniqueArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CreateArgs(re) => f.write_str(&format!("CreateArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateArgs(re) => f.write_str(&format!("UpdateArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpsertArgs(re) => f.write_str(&format!("UpsertArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CopyArgs(re) => f.write_str(&format!("CopyArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::DeleteArgs(re) => f.write_str(&format!("DeleteArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CreateManyArgs(re) => f.write_str(&format!("CreateManyArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateManyArgs(re) => f.write_str(&format!("UpdateManyArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CopyManyArgs(re) => f.write_str(&format!("CopyManyArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::DeleteManyArgs(re) => f.write_str(&format!("DeleteManyArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CountArgs(re) => f.write_str(&format!("CountArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::AggregateArgs(re) => f.write_str(&format!("AggregateArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::GroupByArgs(re) => f.write_str(&format!("GroupByArgs<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::RelationFilter(re) => f.write_str(&format!("RelationFilter<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::ListRelationFilter(re) => f.write_str(&format!("ListRelationFilter<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::WhereInput(re) => f.write_str(&format!("WhereInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::WhereUniqueInput(re) => f.write_str(&format!("WhereUniqueInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::ScalarFieldEnum(re) => f.write_str(&format!("ScalarFieldEnum<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::ScalarWhereWithAggregatesInput(re) => f.write_str(&format!("ScalarWhereWithAggregatesInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CountAggregateInputType(re) => f.write_str(&format!("CountAggregateInputType<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::SumAggregateInputType(re) => f.write_str(&format!("SumAggregateInputType<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::AvgAggregateInputType(re) => f.write_str(&format!("AvgAggregateInputType<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::MaxAggregateInputType(re) => f.write_str(&format!("MaxAggregateInputType<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::MinAggregateInputType(re) => f.write_str(&format!("MinAggregateInputType<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CreateInput(re) => f.write_str(&format!("CreateInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CreateInputWithout(re, r) => f.write_str(&format!("CreateInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::CreateNestedOneInput(re) => f.write_str(&format!("CreateNestedOneInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CreateNestedOneInputWithout(re, r) => f.write_str(&format!("CreateNestedOneInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::CreateNestedManyInput(re) => f.write_str(&format!("CreateNestedManyInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CreateNestedManyInputWithout(re, r) => f.write_str(&format!("CreateNestedManyInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::UpdateInput(re) => f.write_str(&format!("UpdateInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateInputWithout(re, r) => f.write_str(&format!("UpdateInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::UpdateNestedOneInput(re) => f.write_str(&format!("UpdateNestedOneInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateNestedOneInputWithout(re, r) => f.write_str(&format!("UpdateNestedOneInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::UpdateNestedManyInput(re) => f.write_str(&format!("UpdateNestedManyInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateNestedManyInputWithout(re, r) => f.write_str(&format!("UpdateNestedManyInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::ConnectOrCreateInput(re) => f.write_str(&format!("ConnectOrCreateInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::ConnectOrCreateInputWithout(re, r) => f.write_str(&format!("ConnectOrCreateInputWithout<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::UpdateWithWhereUniqueInput(re) => f.write_str(&format!("UpdateWithWhereUniqueInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateWithWhereUniqueInputWithout(re, r) => f.write_str(&format!("UpdateWithWhereUniqueInput<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::UpsertWithWhereUniqueInput(re) => f.write_str(&format!("UpsertWithWhereUniqueInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpsertWithWhereUniqueInputWithout(re, r) => f.write_str(&format!("UpsertWithWhereUniqueInput<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::UpdateManyWithWhereInput(re) => f.write_str(&format!("UpdateManyWithWhereInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::UpdateManyWithWhereInputWithout(re, r) => f.write_str(&format!("UpdateManyWithWhereInput<{}, .{}>", re.string_path().join("."), r)),
            SynthesizedShapeReference::Select(re) => f.write_str(&format!("Select<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::Include(re) => f.write_str(&format!("Include<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::OrderByInput(re) => f.write_str(&format!("OrderByInput<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::Result(re) => f.write_str(&format!("Result<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::CountAggregateResult(re) => f.write_str(&format!("CountAggregateResult<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::SumAggregateResult(re) => f.write_str(&format!("SumAggregateResult<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::AvgAggregateResult(re) => f.write_str(&format!("AvgAggregateResult<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::MinAggregateResult(re) => f.write_str(&format!("MinAggregateResult<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::MaxAggregateResult(re) => f.write_str(&format!("MaxAggregateResult<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::AggregateResult(re) => f.write_str(&format!("AggregateResult<{}>", re.string_path().join("."))),
            SynthesizedShapeReference::GroupByResult(re) => f.write_str(&format!("GroupByResult<{}>", re.string_path().join("."))),
        }
    }
}