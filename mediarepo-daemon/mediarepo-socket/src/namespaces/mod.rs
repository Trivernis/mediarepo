use mediarepo_core::bromine::prelude::AsyncStreamProtocolListener;
use mediarepo_core::bromine::{namespace, namespace::Namespace, IPCBuilder};

pub mod files;
pub mod jobs;
pub mod presets;
pub mod repo;
pub mod tags;

pub fn build_namespaces<L: AsyncStreamProtocolListener>(builder: IPCBuilder<L>) -> IPCBuilder<L> {
    builder
        .add_namespace(namespace!(files::FilesNamespace))
        .add_namespace(namespace!(tags::TagsNamespace))
        .add_namespace(namespace!(repo::RepoNamespace))
        .add_namespace(namespace!(jobs::JobsNamespace))
        .add_namespace(namespace!(presets::PresetsNamespace))
}
