use crate::traits::{StableVTableTrait, StablePointer, StablePointerCast};
use crate::ptr::StableNonNull;
use core::ptr::NonNull;

use ::alloc::boxed::Box as RustBox;
use core::ops::{Deref, DerefMut};

#[repr(transparent)]
pub struct Box<Trait: StableVTableTrait + ?Sized>{
    ptr: StableNonNull<Trait>
}

impl<Trait: StableVTableTrait + StablePointerCast<StableNonNull<Trait>> + ?Sized> From<RustBox<Trait>> for Box<Trait>{
    fn from(t: RustBox<Trait>) -> Self {
        Box{ptr: NonNull::from(RustBox::leak(t)).into()}
    }
}

impl<Trait: StableVTableTrait + ?Sized> Drop for Box<Trait>{
    fn drop(&mut self) {
        unsafe{
            self.ptr.drop_in_place();
            self.ptr.dealloc();
        }
    }
}

impl<Trait: StableVTableTrait + StablePointerCast<StableNonNull<Trait>> + ?Sized> Deref for Box<Trait>{
    type Target = Trait;

    fn deref(&self) -> &Self::Target {
        <Trait as StablePointerCast<StableNonNull<Trait>>>::from_stable_ref(unsafe{self.ptr.deref()})
    }
}

impl<Trait: StableVTableTrait + StablePointerCast<StableNonNull<Trait>> + ?Sized> DerefMut for Box<Trait>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        <Trait as StablePointerCast<StableNonNull<Trait>>>::from_stable_mut(unsafe{self.ptr.deref_mut()})
    }
}