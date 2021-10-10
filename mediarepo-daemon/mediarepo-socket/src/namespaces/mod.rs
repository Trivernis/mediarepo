use crate::namespaces::files::FILES_NAMESPACE;
use rmp_ipc::IPCBuilder;

pub mod files;

pub fn build_namespaces(builder: IPCBuilder) -> IPCBuilder {
    let builder = files::build(builder.namespace(FILES_NAMESPACE)).build();

    builder
}
