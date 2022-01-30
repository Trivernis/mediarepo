pub use mediarepo_database::entities::namespace;
pub use mediarepo_database::entities::tag;

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
            namespace: namespace_model.map(NamespaceDto::new),
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

    /// Returns the normalized name of the tag (namespace:tag)
    pub fn normalized_name(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}:{}", namespace.name(), self.name())
        } else {
            self.name().to_owned()
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddTagDto {
    pub namespace: Option<String>,
    pub name: String,
}

impl AddTagDto {
    pub fn from_tuple(tuple: (Option<String>, String)) -> Self {
        let (namespace, name) = tuple;
        Self { namespace, name }
    }

    /// Returns the normalized name of the tag (namespace:tag)
    pub fn normalized_name(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}:{}", namespace, &self.name)
        } else {
            self.name.to_owned()
        }
    }
}
