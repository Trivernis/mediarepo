pub use mediarepo_database::entities::tag;
pub use mediarepo_database::entities::namespace;
use crate::dto::NamespaceDto;

#[derive(Clone, Debug)]
pub struct TagDto {
    model: tag::Model,
    namespace: Option<NamespaceDto>,
}

impl TagDto {
    pub(crate) fn new(model: tag::Model, namespace_model: Option<namespace::Model>) -> Self {
        Self {
            model,
            namespace: namespace_model.map(NamespaceDto::new)
        }
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn name(&self) -> &String {
        &self.model.name
    }

    pub fn namespace(&self) -> Option<&NamespaceDto> {
        self.namespace.as_ref()
    }
}