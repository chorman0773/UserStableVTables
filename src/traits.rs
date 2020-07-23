use std::ops::Deref;

///
/// Defines a type which is a valid vtable for a stable_vtable trait from rfc 2955
/// Consumers of this trait may assume implementors can be freely transmuted to the VTable trait defined in this module,
///  and that the defined invariants for the fields of the vtable are upheld.
/// Additionally, implementations for the same `Trait` may be freely transmuted between each other.
///
pub unsafe trait TraitVTable<Trait: StableVTableTrait+?Sized>{}

///
/// Defines a type which is a trait object for a stable_vtable trait as per rfc 2955
pub unsafe trait StableVTableTrait{
    type VTable: TraitVTable<Self>;
}


/// A type which is layout compatible with a vtable from rfc 2955, but may not be relied upon to uphold the invariants of such a vtable
#[repr(C)]
pub struct VTable{
    ///
    /// The value of core::mem::size_of_val for the object
    ///
    pub size: usize,
    ///
    /// The value of core::mem::align_of_val for the object
    pub align: usize,
    ///
    /// If present, points to a function which performs the destructor operation for the type
    pub drop_in_place: Option<unsafe extern"C" fn(*mut ())->()>,
    ///
    /// If present, points to a function which can safely deallocate the pointer
    pub dealloc: Option<unsafe extern"C" fn(*mut ())->()>,
    ///
    /// Each entry points to the implementation of each trait function which can be called on a trait object
    ///  in the declaration order in the trait.
    pub _vfns: [unsafe extern"C" fn(*mut ())->();0]
}


pub unsafe trait StablePointerLifetime<'a,Trait: StableVTableTrait + ?Sized>: 'a{
    type Reference: StableReference<'a,Trait>;
    type MutReference: StableMutable<'a,Trait>;
}

///
/// Defines a type which is Layout Compatible with a stable-layout-pointer from rfc 2955.
/// All implementations of this trait for a particular `Trait` shall be valid to transmute between.
/// Additionally, it shall be valid to transmute from any implementation of StableRef,
///  and to an implementation of StableRef or StableMut, provided the reference validity requirements are upheld.
pub unsafe trait StablePointer<Trait: StableVTableTrait + ?Sized>: Copy + Clone + for<'a> StablePointerLifetime<'a>{
    /// Retrieves the size of the value from
    unsafe fn size_of_val(self) -> usize;
    unsafe fn align_of_val(self) -> usize;
    unsafe fn drop_in_place(self) -> usize;
    unsafe fn dealloc(self) -> usize;
    ///
    /// Dereferences the pointer
    /// All requirements of the equivalent reference from rust shall be upheld or the behaviour is undefined.
    /// This operation shall be equivalent to a transmute.
    unsafe fn deref<'a>(self) -> <Self as StablePointerLifetime<'a>>::Reference
        where Trait: 'a ;

    unsafe fn deref_mut<'a>(self) -> <Self as StablePointerLifetime<'a>>::MutReference
        where Trait: 'a;

    fn into_other<P: StablePointer<Trait>>(self) -> P{
        unsafe{core::mem::transmute(self)}
    }
}


///
/// Defines a safe-to-use type which is Layout Compatible with a stable-layout-pointer from rfc 2955
/// It shall not be possible to safely construct a StableRef<'a,Trait> unless Trait: 'a, or any of the following is violated:
/// The VTable shall be valid to read for reading for its size and for 'a,
///  and shall not be modifiable through the reference.
/// The VTable pointer may be assumed to be readonly.
/// data shall be valid for reading for size, and well aligned to align for 'a.
///
/// Implementations may assume all of the above is true.
pub unsafe trait StableReference<'a,Trait: StableVTableTrait + ?Sized>: 'a {
    type Pointer: StablePointer<Trait>;
    fn size_of_val(&self) -> usize where Trait: 'a;
    fn align_of_val(&self) -> usize where Trait: 'a;
    /// Converts the value into a raw pointer
    /// This operation shall be equivalent to a transmute.
    fn into_raw(self)-> Self::Pointer;
}

///
/// A stable-layout mutable (unique) reference.
///
/// It shall not be possible to safely construct a StableMut<'a,Trait> unless the following is true:
/// data shall be valid for writing for size, and shall not be accessed through any other pointer that is not reborrowed from the reference
///  for 'a.
///
/// Implementations may assume all of the above is true
pub unsafe trait StableMutable<'a,Trait: StableVTableTrait + ?Sized>: StableReference<'a,Trait>{}