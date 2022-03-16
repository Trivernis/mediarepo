use mediarepo_core::trait_bound_typemap::TypeMapKey;
use std::sync::Arc;

use crate::dao::repo::Repo;

pub struct RepoKey;

impl TypeMapKey for RepoKey {
    type Value = Arc<Repo>;
}
