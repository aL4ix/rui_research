use std::any::{Any, TypeId};

pub trait Downcast: 'static {
    fn is<T: 'static + ?Sized>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }
    fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        unsafe { (*self).is::<T>().then(|| &*<*const _>::cast(self)) }
    }
    ///
    /// Use it like (**downcastable).downcast_mut::<ConcreteType>();
    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        unsafe { (*self).is::<T>().then(|| &mut *<*mut _>::cast(self)) }
    }
}

impl<T: ?Sized + 'static> Downcast for T {}
