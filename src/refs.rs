use crate::traits::{StableVTableTrait, IntoStablePtr, StableReference, VTable, StableMutable};
use crate::ptr::StablePtr;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

///
/// Safety: Must satisfy all the rules of &dyn Trait, in addition to the rules for references defined by rfc 2955
/// Note: Here the vtable has to satisfy 'a.
/// It is not yet specified the lifetime a vtable has to satisfy.
#[repr(C)]
pub struct StableRef<'a,Trait: StableVTableTrait>{
    data: NonNull<()>,
    vtable: &'a VTable
}

unsafe impl<'a,Trait: StableVTableTrait> StableReference<'a,Trait> for StableRef<'a,Trait>{
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
///
/// Safety: Must satisfy all the rules of &dyn Trait, in addition to the rules for references defined by rfc 2955
/// Note: Here the vtable has to satisfy 'a.
/// It is not yet specified the lifetime a vtable has to satisfy.
pub struct StableMut<'a,Trait: StableVTableTrait>{
    data: NonNull<()>,
    vtable: &'a Trait::VTable
}

unsafe impl<'a,Trait: StableVTableTrait> StableReference<'a,Trait> for StableMut<'a,Trait>{
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

unsafe impl<'a,Trait: StableVTableTrait> StableMutable<'a,Trait> for StableMut<'a,Trait>{}
