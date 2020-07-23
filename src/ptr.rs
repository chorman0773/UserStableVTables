use crate::traits::{StableVTableTrait, StablePointer, StablePointerLifetime, VTable, StableReference, StablePointerCast};
use std::alloc::Layout;
use crate::refs::{StableRef, StableMut};
use std::ptr::NonNull;

#[repr(C)]
pub struct StablePtr<Trait: StableVTableTrait>{
    pub data: *mut (),
    pub vtable: *const Trait::VTable
}

impl<Trait: StableVTableTrait> From<*mut Trait> for StablePtr<Trait>
    where Trait: StablePointerCast<StablePtr<Trait>>{
    fn from(ptr: *mut Trait) -> Self {
        unsafe { <Trait as StablePointerCast<StablePtr<Trait>>>::to_stable(ptr) }
    }
}

impl<Trait: StableVTableTrait> From<*const Trait> for StablePtr<Trait>
    where Trait: StablePointerCast<StablePtr<Trait>>{
    fn from(ptr: *const Trait) -> Self {
        unsafe { <Trait as StablePointerCast<StablePtr<Trait>>>::to_stable(ptr as *mut Trait) }
    }
}

#[repr(C)]
pub struct StableNonNull<Trait: StableVTableTrait>{
    pub data: NonNull<()>,
    pub vtable: NonNull<Trait::VTable>
}

impl<Trait: StableVTableTrait> From<NonNull<Trait>> for StableNonNull<Trait>
    where Trait: StablePointerCast<StableNonNull<Trait>>{
    fn from(ptr: NonNull<Trait>) -> Self {
        unsafe { <Trait as StablePointerCast<StableNonNull<Trait>>>::to_stable(ptr.as_ptr() as *mut Trait) }
    }
}

impl<Trait: StableVTableTrait> From<StableNonNull<Trait>> for StablePtr<Trait>{
    fn from(ptr: StableNonNull<Trait>) -> Self {
        unsafe{ptr.into_other()}
    }
}
unsafe impl<'a,Trait: StableVTableTrait> StablePointerLifetime<'a,Trait> for StablePtr<Trait>{
    type Reference = StableRef<'a,Trait>;
    type MutReference = StableMut<'a,Trait>;
}

unsafe impl<Trait: StableVTableTrait> StablePointer<Trait> for StablePtr<Trait>{
    unsafe fn size_of_val(self) -> usize {
        *(self.vtable as *const VTable).size
    }

    unsafe fn align_of_val(self) -> usize {
        *(self.vtable as *const VTable).align
    }

    unsafe fn drop_in_place(self) -> usize {
        if let Some(F) = *(self.vtable as *const VTable).drop_in_place{
            F(self.data)
        }
    }

    unsafe fn dealloc(self) -> usize {
        if let Some(F) = *(self.vtable as *const VTable).dealloc{
            F(self.data)
        }else{
            alloc::alloc::dealloc(self.data as *mut u8,Layout::from_size_align_unchecked(*(self.vtable as *const VTable).size,*(self.vtable as *const VTable).align))
        }
    }

    unsafe fn deref<'a>(self) -> <Self as StablePointerLifetime<'a>>::Reference
        where Trait: 'a{
        core::mem::transmute(self)
    }

    unsafe fn deref_mut<'a>(self) -> <Self as StablePointerLifetime<'a>>::MutReference
        where Trait: 'a {
        core::mem::transmute(self)
    }
}

unsafe impl<'a,Trait: StableVTableTrait> StablePointerLifetime<'a,Trait> for StableNonNull<Trait>{
    type Reference = StableRef<'a,Trait>;
    type MutReference = StableMut<'a,Trait>;
}

unsafe impl<Trait: StableVTableTrait> StablePointer<Trait> for StableNonNull<Trait>{
    unsafe fn size_of_val(self) -> usize {
        *(self.vtable.as_ptr() as *const VTable).size
    }

    unsafe fn align_of_val(self) -> usize {
        *(self.vtable.as_ptr() as *const VTable).align
    }

    unsafe fn drop_in_place(self) -> usize {
        if let Some(F) = *(self.vtable.as_ptr() as *const VTable).drop_in_place{
            F(self.data.as_ptr())
        }
    }

    unsafe fn dealloc(self) -> usize {
        if let Some(F) = *(self.vtable.as_ptr() as *const VTable).dealloc{
            F(self.data.as_ptr())
        }else{
            alloc::alloc::dealloc(self.data as *mut u8,Layout::from_size_align_unchecked(*(self.vtable as *const VTable).size,*(self.vtable as *const VTable).align))
        }
    }

    unsafe fn deref<'a>(self) -> <Self as StablePointerLifetime<'a>>::Reference
        where Trait: 'a{
        core::mem::transmute(self)
    }

    unsafe fn deref_mut<'a>(self) -> <Self as StablePointerLifetime<'a>>::MutReference
        where Trait: 'a {
        core::mem::transmute(self)
    }
}