use crate::dao::repo::Repo;
use std::sync::Arc;
use typemap_rev::TypeMapKey;

pub struct RepoKey;

impl TypeMapKey for RepoKey {
    type Value = Arc<Repo>;
}
