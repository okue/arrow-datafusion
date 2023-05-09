//! ORC format abstractions

use std::any::Any;
use std::sync::Arc;

use arrow2::io::orc::format as orc_format;
use arrow2::io::orc::read as orc_read;
use arrow_schema::SchemaRef;
use async_trait::async_trait;
use object_store::{ObjectMeta, ObjectStore};

use datafusion_common::Statistics;
use datafusion_physical_expr::PhysicalExpr;

use crate::datasource::file_format::FileFormat;
use crate::execution::context::SessionState;
use crate::physical_plan::ExecutionPlan;
use crate::physical_plan::file_format::FileScanConfig;

#[derive(Debug)]
pub struct OrcFormat {}

#[async_trait]
impl FileFormat for OrcFormat {
    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    async fn infer_schema(&self, state: &SessionState, store: &Arc<dyn ObjectStore>, objects: &[ObjectMeta]) -> datafusion_common::Result<SchemaRef> {
        todo!()
    }

    async fn infer_stats(&self, state: &SessionState, store: &Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &ObjectMeta) -> datafusion_common::Result<Statistics> {
        todo!()
    }

    async fn create_physical_plan(&self, state: &SessionState, conf: FileScanConfig, filters: Option<&Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<Arc<dyn ExecutionPlan>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        println!("Hello world.")
    }
}
