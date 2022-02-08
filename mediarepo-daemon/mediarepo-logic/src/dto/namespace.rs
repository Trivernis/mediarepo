use mediarepo_database::entities::namespace;

#[derive(Clone, Debug)]
pub struct NamespaceDto {
    model: namespace::Model,
}

impl NamespaceDto {
    pub(crate) fn new(model: namespace::Model) -> Self {
        Self { model }
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn name(&self) -> &String {
        &self.model.name
    }
}
