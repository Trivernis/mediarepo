use std::any::{Any, TypeId};
use std::vec::IntoIter;
use typemap_rev::TypeMapKey;

pub trait CloneAny: Any + Send + Sync {
    fn clone_any(&self) -> Box<dyn CloneAny>;
}

impl<T: Any + Clone + Send + Sync> CloneAny for T {
    fn clone_any(&self) -> Box<dyn CloneAny> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneAny> {
    fn clone(&self) -> Self {
        (**self).clone_any()
    }
}

#[derive(Default, Clone)]
pub struct TypeList(Vec<(TypeId, Box<dyn CloneAny>)>);

impl TypeList {
    pub fn add<T: TypeMapKey<Value = C>, C: CloneAny>(&mut self, value: T::Value) {
        self.0.push((TypeId::of::<T>(), Box::new(value)))
    }
}

impl IntoIterator for TypeList {
    type Item = (TypeId, Box<dyn Any + Send + Sync>);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(t, v)| {
                (t, unsafe {
                    // SAFETY: CloneAny requires types to be Any + Send + Sync (+ Clone)
                    std::mem::transmute::<Box<dyn CloneAny>, Box<dyn Any + Send + Sync>>(v)
                })
            })
            .collect::<Vec<(TypeId, Box<dyn Any + Send + Sync>)>>()
            .into_iter()
    }
}
