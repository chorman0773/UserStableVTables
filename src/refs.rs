use crate::traits::{StableVTableTrait, StableReference, VTable, StableMutable, StablePointerCast};
use crate::ptr::StablePtr;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::marker::PhantomData;

/// A type-erased pointer with stable layout to a trait object
/// This pointer has the same layout as `&dyn Trait` for `#[stable_vtable]` traits
///  as with [[RFC 2955]](https://github.com/rust-lang/rfcs/pull/2955).
/// Note: While `&T` has special handling when inside `Option<T>`, no such guarantee is stably made.
///  There are test suites that check to ensure this is correct.
///
/// Also Note: Specifically, this implementation imposes a restriction that the VTable reference be valid for reading for 'a.
///  It is not yet specified the required validity of the vtable
#[repr(C)]
pub struct StableRef<'a,Trait: StableVTableTrait + ?Sized>{
    data: NonNull<()>,
    vtable: NonNull<VTable>,
    phantom: PhantomData<&'a Trait>
}

unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StableReference<'a,Trait> for StableRef<'a,Trait>{
    type Pointer = StablePtr<Trait>;

    fn size_of_val(&self) -> usize where Trait: 'a {
        unsafe{self.vtable.as_ref().size}
    }

    fn align_of_val(&self) -> usize where Trait: 'a {
        unsafe{self.vtable.as_ref().align}
    }

    fn into_raw(self) -> Self::Pointer {
        unsafe{core::mem::transmute(self)}
    }
}



impl<Trait: StableVTableTrait + StablePointerCast<StablePtr<Trait>> + ?Sized> Deref for StableRef<'_,Trait>{
    type Target = Trait;

    fn deref(&self) -> &Self::Target {
        <Trait as StablePointerCast<StablePtr<Trait>>>::borrow_stable_ref(self)
    }
}

impl<'a,'b: 'a,Trait: StableVTableTrait + ?Sized> From<StableMut<'b,Trait>> for StableRef<'a,Trait>{
    fn from(v: StableMut<'b, Trait>) -> Self {
        unsafe{core::mem::transmute(v)}
    }
}

/// A type-erased pointer with stable layout to a trait object
/// This pointer has the same layout as `&dyn Trait` for `#[stable_vtable]` traits
///  as with [[RFC 2955]](https://github.com/rust-lang/rfcs/pull/2955).
/// Note: While `&mut T` has special handling when inside `Option<T>`, no such guarantee is stably made.
///  There are test suites that check to ensure this is correct.
///
/// Also Note: Specifically, this implementation imposes a restriction that the VTable reference be valid for reading for 'a.
///  It is not yet specified the required validity of the vtable
#[repr(C)]
pub struct StableMut<'a,Trait: StableVTableTrait + ?Sized>{
    data: NonNull<()>,
    vtable: NonNull<VTable>,
    phantom: PhantomData<&'a Trait>
}


unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StableReference<'a,Trait> for StableMut<'a,Trait>{
    type Pointer = StablePtr<Trait>;

    fn size_of_val(&self) -> usize where Trait: 'a {
        unsafe{self.vtable.as_ref().size}
    }

    fn align_of_val(&self) -> usize where Trait: 'a {
        unsafe{self.vtable.as_ref().align}
    }

    fn into_raw(self) -> Self::Pointer {
        unsafe{core::mem::transmute(self)}
    }
}

unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StableMutable<'a,Trait> for StableMut<'a,Trait>{}

impl<Trait: StableVTableTrait + StablePointerCast<StablePtr<Trait>> + ?Sized> Deref for StableMut<'_,Trait>{
    type Target = Trait;

    fn deref(&self) -> &Self::Target {
        <Trait as StablePointerCast<StablePtr<Trait>>>::borrow_stable_ref(unsafe{core::mem::transmute(self)})
    }
}

impl<Trait: StableVTableTrait + StablePointerCast<StablePtr<Trait>> + ?Sized> DerefMut for StableMut<'_,Trait>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        <Trait as StablePointerCast<StablePtr<Trait>>>::borrow_stable_mut(self)
    }
}
