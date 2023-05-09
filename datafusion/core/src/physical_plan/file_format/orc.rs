//! Execution plan for reading ORC files


use std::any::Any;
use std::fmt::Formatter;
use std::sync::Arc;
use arrow_schema::SchemaRef;
use datafusion_common::Statistics;
use datafusion_execution::TaskContext;
use datafusion_physical_expr::{EquivalenceProperties, OrderingEquivalenceProperties, PhysicalSortExpr, PhysicalSortRequirement};
use crate::physical_plan::{DisplayFormatType, Distribution, ExecutionPlan, Partitioning, SendableRecordBatchStream};
use crate::physical_plan::file_format::{FileMeta, FileOpener, FileOpenFuture};
use crate::physical_plan::metrics::MetricsSet;

#[derive(Debug, Clone)]
pub struct OrcExec {}

impl ExecutionPlan for OrcExec {
    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn schema(&self) -> SchemaRef {
        todo!()
    }

    fn output_partitioning(&self) -> Partitioning {
        todo!()
    }

    fn unbounded_output(&self, _children: &[bool]) -> datafusion_common::Result<bool> {
        todo!()
    }

    fn output_ordering(&self) -> Option<&[PhysicalSortExpr]> {
        todo!()
    }

    fn required_input_distribution(&self) -> Vec<Distribution> {
        todo!()
    }

    fn required_input_ordering(&self) -> Vec<Option<Vec<PhysicalSortRequirement>>> {
        todo!()
    }

    fn maintains_input_order(&self) -> Vec<bool> {
        todo!()
    }

    fn benefits_from_input_partitioning(&self) -> bool {
        todo!()
    }

    fn equivalence_properties(&self) -> EquivalenceProperties {
        todo!()
    }

    fn ordering_equivalence_properties(&self) -> OrderingEquivalenceProperties {
        todo!()
    }

    fn children(&self) -> Vec<Arc<dyn ExecutionPlan>> {
        todo!()
    }

    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> datafusion_common::Result<Arc<dyn ExecutionPlan>> {
        todo!()
    }

    fn execute(&self, partition: usize, context: Arc<TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream> {
        todo!()
    }

    fn metrics(&self) -> Option<MetricsSet> {
        todo!()
    }

    fn fmt_as(&self, _t: DisplayFormatType, f: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn statistics(&self) -> Statistics {
        todo!()
    }
}

struct OrcOpener {}

impl FileOpener for OrcOpener {
    fn open(&self, file_meta: FileMeta) -> datafusion_common::Result<FileOpenFuture> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
