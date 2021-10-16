use mediarepo_core::rmp_ipc::{namespace, namespace::Namespace, IPCBuilder};

pub mod files;

pub fn build_namespaces(builder: IPCBuilder) -> IPCBuilder {
    builder.add_namespace(namespace!(files::FilesNamespace))
}
