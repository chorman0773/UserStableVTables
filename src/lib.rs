
//! Library for interacting with pointers with *stable-vtable layout*
//!

#![no_std]

#![deny(warnings)]

extern crate static_assertions;

#[cfg(any(feature="alloc",test))]
extern crate alloc;

/// Traits used by this library to provide features
pub mod traits;
/// Raw pointer tyes, such as StablePtr and StableNonNull
pub mod ptr;
/// Reference types, which are safe to use
pub mod refs;

/// Box smart pointer
#[cfg(feature="box")]
pub mod boxed;


#[cfg(test)]
mod some_tests{
    use crate::traits::{TraitVTable, StableVTableTrait, StablePointer};
    use crate::refs::StableRef;
    use crate::ptr::{StableNonNull, StablePtr};
    pub trait WithStableVTable{
        extern"C" fn item(&self);
    }

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct __WithStableVTable_VTable{
        pub size: usize,
        pub align: usize,
        pub destroy: Option<unsafe extern"C" fn(*mut ())->()>,
        pub dealloc: unsafe extern"C" fn(*mut ()),
        pub _vfn_item: unsafe extern"C" fn(*const ())
    }

    unsafe impl TraitVTable<dyn WithStableVTable> for __WithStableVTable_VTable{}

    unsafe impl StableVTableTrait for dyn WithStableVTable{
        type VTable = __WithStableVTable_VTable;
    }

    static_assertions::assert_eq_size!(StableRef<dyn WithStableVTable>,Option<StableRef<dyn WithStableVTable>>);
    static_assertions::assert_eq_size!(StableNonNull<dyn WithStableVTable>,Option<StableNonNull<dyn WithStableVTable>>);

    #[test]
    pub fn test_ref_none_is_null(){
        let x = None::<StableRef<dyn WithStableVTable>>;
        let ptr = unsafe{core::mem::transmute::<_,StablePtr<dyn WithStableVTable>>(x)};
        assert!(ptr.is_null())
    }

    #[test]
    pub fn test_nonnull_none_is_null(){
        let x = None::<StableRef<dyn WithStableVTable>>;
        let ptr = unsafe{core::mem::transmute::<_,StablePtr<dyn WithStableVTable>>(x)};
        assert!(ptr.is_null())
    }
    struct StableVTableImpl;
    impl WithStableVTable for StableVTableImpl{
        extern"C" fn item(&self) {

        }
    }
    #[allow(non_upper_case_globals)]
    #[test]
    #[should_panic]
    pub fn test_ref_some_is_not_null(){
        let obj = StableVTableImpl;
        unsafe extern"C" fn dealloc<T: WithStableVTable>(p: *mut ()){
            alloc::alloc::dealloc(p as *mut u8,::core::alloc::Layout::new::<T>())
        }
        unsafe extern"C" fn _vfn_item<T: WithStableVTable>(p: *const ()){
            <T as WithStableVTable>::item(&*(p as *const T) )
        }
        static __WithStableVTable_STableVTableImpl__V: __WithStableVTable_VTable = __WithStableVTable_VTable{
            size: core::mem::size_of::<StableVTableImpl>(),
            align: core::mem::size_of::<StableVTableImpl>(),
            destroy: None,
            dealloc: dealloc::<StableVTableImpl>,
            _vfn_item: _vfn_item::<StableVTableImpl>
        };
        let x = Some(unsafe{StablePtr::<dyn WithStableVTable>{
            data: &obj as *const _ as *mut StableVTableImpl as *mut (),
            vtable: &__WithStableVTable_STableVTableImpl__V as *const __WithStableVTable_VTable
        }.deref()});
        let ptr = unsafe{core::mem::transmute::<_,StablePtr<dyn WithStableVTable>>(x)};
        assert!(ptr.is_null())
    }
}