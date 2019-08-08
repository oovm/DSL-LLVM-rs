use std::any::{Any as StdAny, TypeId};
use std::fmt;

pub trait Any: StdAny {
    fn type_id(&self) -> TypeId;
    fn box_clone(&self) -> Box<dyn Any>;
}

impl<T> Any for T
where
    T: Clone + StdAny + ?Sized,
{
    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    #[inline]
    fn box_clone(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl dyn Any {
    #[inline]
    fn box_clone(&self) -> Box<dyn Any> {
        Any::box_clone(self)
    }

    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        let t = TypeId::of::<T>();
        let boxed = <dyn Any as Any>::type_id(self);

        t == boxed
    }

    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Any as *const T)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}

impl Clone for Box<dyn Any> {
    fn clone(&self) -> Self {
        Any::box_clone(self.as_ref() as &dyn Any)
    }
}

impl fmt::Debug for dyn Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Any")
    }
}
