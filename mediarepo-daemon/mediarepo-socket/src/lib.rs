use rmp_ipc::IPCBuilder;

mod namespaces;
pub mod types;

pub fn get_builder(address: &str) -> IPCBuilder {
    namespaces::build_namespaces(IPCBuilder::new().address(address))
}
