use crate::traits::{StableVTableTrait, StablePointer, StablePointerLifetime, VTable, StablePointerCast};
use crate::refs::{StableRef, StableMut};
use core::ptr::NonNull;

/// A type-erased pointer with stable layout to a trait object
/// This pointer has the same layout as `*mut dyn Trait` for `#[stable_vtable]` traits
///  as with [[RFC 2955]](https://github.com/rust-lang/rfcs/pull/2955)
#[repr(C)]
pub struct StablePtr<Trait: StableVTableTrait + ?Sized>{
    pub data: *mut (),
    pub vtable: *const Trait::VTable
}

impl<Trait: StableVTableTrait + ?Sized> StablePtr<Trait>{
    pub fn is_null(self) ->bool{
        self.data.is_null()
    }
}

impl<Trait: StableVTableTrait + ?Sized> From<*mut Trait> for StablePtr<Trait>
    where Trait: StablePointerCast<StablePtr<Trait>>{
    fn from(ptr: *mut Trait) -> Self {
        unsafe { <Trait as StablePointerCast<StablePtr<Trait>>>::to_stable(ptr) }
    }
}

impl<Trait: StableVTableTrait + ?Sized> From<*const Trait> for StablePtr<Trait>
    where Trait: StablePointerCast<StablePtr<Trait>>{
    fn from(ptr: *const Trait) -> Self {
        unsafe { <Trait as StablePointerCast<StablePtr<Trait>>>::to_stable(ptr as *mut Trait) }
    }
}

/// A type-erased pointer with stable layout to a trait object
/// This pointer has the same layout as `NonNull<dyn Trait>` for `#[stable_vtable]` traits
///  as with [[RFC 2955]](https://github.com/rust-lang/rfcs/pull/2955).
/// Note: While `NonNull<T>` has special handling when inside `Option<T>`, no such guarantee is stably made.
///  There are test suites that check to ensure this is correct.
#[repr(C)]
pub struct StableNonNull<Trait: StableVTableTrait + ?Sized>{
    pub data: NonNull<()>,
    pub vtable: NonNull<Trait::VTable>
}

impl<Trait: StableVTableTrait + ?Sized> Copy for StablePtr<Trait>{}
impl<Trait: StableVTableTrait + ?Sized> Copy for StableNonNull<Trait>{}

impl<Trait: StableVTableTrait + ?Sized> Clone for StablePtr<Trait>{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Trait: StableVTableTrait + ?Sized> Clone for StableNonNull<Trait>{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Trait: StableVTableTrait + ?Sized> From<NonNull<Trait>> for StableNonNull<Trait>
    where Trait: StablePointerCast<StableNonNull<Trait>>{
    fn from(ptr: NonNull<Trait>) -> Self {
        unsafe { <Trait as StablePointerCast<StableNonNull<Trait>>>::to_stable(ptr.as_ptr() as *mut Trait) }
    }
}

impl<Trait: StableVTableTrait + ?Sized> From<StableNonNull<Trait>> for StablePtr<Trait>{
    fn from(ptr: StableNonNull<Trait>) -> Self {
        unsafe{ptr.into_other()}
    }
}


unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StablePointerLifetime<'a,Trait> for StablePtr<Trait>{
    type Reference = StableRef<'a,Trait>;
    type MutReference = StableMut<'a,Trait>;
}

unsafe impl<Trait: StableVTableTrait + ?Sized> StablePointer<Trait> for StablePtr<Trait>{
    unsafe fn size_of_val(self) -> usize {
        (&*self.vtable.cast::<VTable>()).size
    }

    unsafe fn align_of_val(self) -> usize {
        (&*self.vtable.cast::<VTable>()).align
    }

    unsafe fn drop_in_place(self) -> () {
        if let Some(f) = (&*self.vtable.cast::<VTable>()).drop_in_place{
            (f)(self.data)
        }
    }

    unsafe fn dealloc(self) -> () {
        ((&*self.vtable.cast::<VTable>()).dealloc)(self.data)
    }

    unsafe fn deref<'a>(self) -> <Self as StablePointerLifetime<'a,Trait>>::Reference
        where Trait: 'a{
        core::mem::transmute(self)
    }

    unsafe fn deref_mut<'a>(self) -> <Self as StablePointerLifetime<'a,Trait>>::MutReference
        where Trait: 'a {
        core::mem::transmute(self)
    }
}

unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StablePointerLifetime<'a,Trait> for StableNonNull<Trait>{
    type Reference = StableRef<'a,Trait>;
    type MutReference = StableMut<'a,Trait>;
}

unsafe impl<Trait: StableVTableTrait + ?Sized> StablePointer<Trait> for StableNonNull<Trait>{
    unsafe fn size_of_val(self) -> usize {
        (self.vtable.cast::<VTable>().as_ref()).size
    }

    unsafe fn align_of_val(self) -> usize {
        (self.vtable.cast::<VTable>().as_ref()).align
    }

    unsafe fn drop_in_place(self) -> () {
        if let Some(f) = (self.vtable.cast::<VTable>().as_ref()).drop_in_place{
            (f)(self.data.as_ptr())
        }
    }

    unsafe fn dealloc(self) -> () {
        ((self.vtable.cast::<VTable>().as_ref()).dealloc)(self.data.as_ptr())
    }

    unsafe fn deref<'a>(self) -> <Self as StablePointerLifetime<'a,Trait>>::Reference
        where Trait: 'a{
        core::mem::transmute(self)
    }

    unsafe fn deref_mut<'a>(self) -> <Self as StablePointerLifetime<'a,Trait>>::MutReference
        where Trait: 'a {
        core::mem::transmute(self)
    }
}