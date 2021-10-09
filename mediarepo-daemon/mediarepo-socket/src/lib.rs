use rmp_ipc::IPCBuilder;

mod namespaces;

pub fn get_builder(address: &str) -> IPCBuilder {
    IPCBuilder::new().address(address)
}
