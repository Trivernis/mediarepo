use mediarepo_core::rmp_ipc::prelude::AsyncStreamProtocolListener;
use mediarepo_core::rmp_ipc::{namespace, namespace::Namespace, IPCBuilder};

pub mod files;
pub mod tags;

pub fn build_namespaces<L: AsyncStreamProtocolListener>(builder: IPCBuilder<L>) -> IPCBuilder<L> {
    builder
        .add_namespace(namespace!(files::FilesNamespace))
        .add_namespace(namespace!(tags::TagsNamespace))
}
