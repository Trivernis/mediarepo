use std::sync::Arc;

use typemap_rev::TypeMapKey;

use crate::dao::repo::Repo;

pub struct RepoKey;

impl TypeMapKey for RepoKey {
    type Value = Arc<Repo>;
}
