use std::fmt::Display;

use arrow::datatypes::SchemaRef;

// Logical Plan tree
#[derive(Debug, Clone)]
pub enum LogicalPlan {
    Scan(Scan),
    Projection(Projection),
    Filter(Filter),
    Aggregate(Aggregate),
    Sort(Sort),
    Limit(Limit),
    Join(Join),
    Distinct(Distinct)
}

impl LogicalPlan {
    pub fn schema(&self) -> SchemaRef {
        match self {
            LogicalPlan::Scan(plan) => plan.schema(),
            LogicalPlan::Projection(plan) => plan.schema(),
            LogicalPlan::Filter(plan) => plan.schema(),
            LogicalPlan::Aggregate(plan) => plan.schema(),
            LogicalPlan::Sort(plan) => plan.schema(),
            LogicalPlan::Limit(plan) => plan.schema(),
            LogicalPlan::Join(plan) => plan.schema(),
            LogicalPlan::Distinct(plan) => plan.schema(),
        }
    }

    pub fn children(&self) -> Vec<&LogicalPlan> {
        match self {
            LogicalPlan::Scan(plan) => plan.children(),
            LogicalPlan::Projection(plan) => plan.children(),
            LogicalPlan::Filter(plan) => plan.children(),
            LogicalPlan::Aggregate(plan) => plan.children(),
            LogicalPlan::Sort(plan) => plan.children(),
            LogicalPlan::Limit(plan) => plan.children(),
            LogicalPlan::Join(plan) => plan.children(),
            LogicalPlan::Distinct(plan) => plan.children(),
        }
    }
}

impl Display for LogicalPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_plan(self, f, 0)
    }
}

pub fn format_plan(
    input: &LogicalPlan,
    f: &mut std::fmt::Formatter<'_>,
    indent: usize,
) -> std::fmt::Result {
    for _ in 0..indent {
        write!(f, "\t")?;
    }

    match input {
        LogicalPlan::Scan(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Projection(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Filter(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Aggregate(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Sort(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Limit(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Join(plan) => write!(f, "{}", plan)?,
        LogicalPlan::Distinct(plan) => write!(f, "{}", plan)?,
    }
    writeln!(f)?;

    for child in input.children() {
        format_plan(child, f, indent + 1)?;
    }

    Ok(())
}