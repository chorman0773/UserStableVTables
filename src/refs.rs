use crate::traits::{StableVTableTrait, StableReference, VTable, StableMutable, StablePointerCast};
use crate::ptr::StablePtr;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::marker::PhantomData;

///
/// Safety: Must satisfy all the rules of &dyn Trait, in addition to the rules for references defined by rfc 2955
/// Note: Here the vtable has to satisfy 'a.
/// It is not yet specified the lifetime a vtable has to satisfy.
#[repr(C)]
pub struct StableRef<'a,Trait: StableVTableTrait + ?Sized>{
    data: NonNull<()>,
    vtable: &'a VTable,
    phantom: PhantomData<&'a Trait>
}

unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StableReference<'a,Trait> for StableRef<'a,Trait>{
    type Pointer = StablePtr<Trait>;

    fn size_of_val(&self) -> usize where Trait: 'a {
        self.vtable.size
    }

    fn align_of_val(&self) -> usize where Trait: 'a {
        self.vtable.align
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

///
/// Safety: Must satisfy all the rules of &dyn Trait, in addition to the rules for references defined by rfc 2955
/// Note: Here the vtable has to satisfy 'a.
/// It is not yet specified the lifetime a vtable has to satisfy.
#[repr(C)]
pub struct StableMut<'a,Trait: StableVTableTrait + ?Sized>{
    data: NonNull<()>,
    vtable: &'a VTable,
    phantom: PhantomData<&'a Trait>
}


unsafe impl<'a,Trait: StableVTableTrait + ?Sized> StableReference<'a,Trait> for StableMut<'a,Trait>{
    type Pointer = StablePtr<Trait>;

    fn size_of_val(&self) -> usize where Trait: 'a {
        self.vtable.size
    }

    fn align_of_val(&self) -> usize where Trait: 'a {
        self.vtable.align
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
