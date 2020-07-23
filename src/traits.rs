
///
/// Defines a type which is a valid vtable for a stable_vtable trait from rfc 2955
/// Consumers of this trait may assume implementors can be freely transmuted to the VTable trait defined in this module,
///  and that the defined invariants for the fields of the vtable are upheld.
/// Additionally, implementations for the same `Trait` may be freely transmuted between each other.
///
pub unsafe trait TraitVTable<Trait: StableVTableTrait+?Sized>: 'static{}

///
/// Defines a type which is a trait object for a stable_vtable trait as per rfc 2955
pub unsafe trait StableVTableTrait: 'static{
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
    pub dealloc: unsafe extern"C" fn(*mut ())->(),
    ///
    /// Each entry points to the implementation of each trait function which can be called on a trait object
    ///  in the declaration order in the trait.
    pub _vfns: [unsafe extern"C" fn(*mut ())->();0]
}


pub unsafe trait StablePointerLifetime<'a,Trait: StableVTableTrait + ?Sized>: 'a{
    type Reference: StableReference<'a,Trait>;
    type MutReference: StableMutable<'a,Trait>;
}

pub unsafe trait StablePointerCast<Pointer: StablePointer<Self>>: StableVTableTrait{
    unsafe fn to_stable(p: *mut Self) -> Pointer;
    fn to_stable_ref(r: &Self) -> <Pointer as StablePointerLifetime<'_,Self>>::Reference;
    fn to_stable_mut(r: &mut Self) -> <Pointer as StablePointerLifetime<'_,Self>>::MutReference;
    fn from_stable(s: Self) -> *mut Self;
    fn from_stable_ref<'a>(r: <Pointer as StablePointerLifetime<'a,Self>>::Reference) -> &'a mut Self
        where Self: 'a;
    fn from_stable_mut<'a>(r: <Pointer as StablePointerLifetime<'a,Self>>::MutReference) -> &'a mut Self
        where Self: 'a;
    fn borrow_stable_ref<'a,'b: 'a>(r: &'a <Pointer as StablePointerLifetime<'b,Self>>::Reference) -> &'a Self;
    fn borrow_stable_mut<'a,'b: 'a>(r: &'a mut<Pointer as StablePointerLifetime<'b,Self>>::MutReference) -> &'a mut Self;
}

///
/// Defines a type which is Layout Compatible with a stable-layout-pointer from rfc 2955.
/// All implementations of this trait for a particular `Trait` shall be valid to transmute between,
///  except that implementations may validly impose a NonNull requirement on both the data and vtable pointers.
/// Additionally, it shall be valid to transmute from any implementation of StableRef,
///  and to an implementation of StableRef or StableMut, provided the reference validity requirements are upheld.
pub unsafe trait StablePointer<Trait: StableVTableTrait + ?Sized>: Copy + Clone + for<'a> StablePointerLifetime<'a,Trait>{
    /// Retrieves the alignment of the value from the underlying object
    /// unsafe because there are currently no limitations on the validity of vtables for non-reference pointers
    /// This shall return the value of the vtables size field
    ///
    /// Safety
    /// --------------------
    /// The vtable shall be a derefenceable pointer
    unsafe fn size_of_val(self) -> usize;
    /// Retrieves the alignment of the value from the underlying object
    /// unsafe because there are currently no limitations on the validity of vtables for non-reference pointers
    /// This shall return the value of the vtables size field
    ///
    /// Safety
    /// --------------------
    /// The vtable shall be a derefenceable pointer
    unsafe fn align_of_val(self) -> usize;

    /// Executes the destructor operation on the value
    /// The pointed-to value may not be further used,
    ///  even if the destructor operation is trivial.
    ///
    /// Safety
    /// --------------------
    /// The vtable shall be a derefernceable pointer.
    /// The data shall be a dereferenceable pointer which is valid for writing for at least `size` from the vtable,
    ///  and shall be aligned to at least align, additionally, it shall not be accessed from an aliasing region of memory
    ///  (this constraint applies even if there is no destructor or if the destructor operation is trivial).
    ///
    /// After this call, the object pointed by `data` may not be access (but the pointer is still valid for reading).
    unsafe fn drop_in_place(self) -> ();

    /// Deallocates the pointed to value.
    /// The pointer may not be futher used,
    ///  even if the destructor operation is trivial.
    ///
    /// Safety
    /// --------------------
    /// The vtable shall be a derefernceable pointer.
    /// The data shall be a dereferenceable pointer which is valid for writing for at least `size` from the vtable,
    ///  and shall be aligned to at least align, additionally, it shall not be accessed from an aliasing region of memory
    ///  (this constraint applies even if there is no destructor or if the destructor operation is trivial).
    ///
    /// After this call, the pointer is valid for neither reading nor writing.

    unsafe fn dealloc(self) -> ();

    ///
    /// Dereferences the pointer
    /// All requirements of the equivalent reference from rust shall be upheld or the behaviour is undefined.
    /// This operation shall be equivalent to a transmute.
    ///
    /// Safety
    /// -------------------
    /// The vtable shall be a dereferenceable pointer.
    /// The data shall be a dereferenceable pointer which is valid for read for at least `size` from the vtable,
    ///  and shall be aligned to at least align, additionally, it shall not be modified for the lifetime of the reference
    unsafe fn deref<'a>(self) -> <Self as StablePointerLifetime<'a,Trait>>::Reference
        where Trait: 'a ;

    ///
    /// Dereferences the pointer
    /// All requirements of the equivalent reference from rust shall be upheld or the behaviour is undefined.
    /// This operation shall be equivalent to a transmute.
    ///
    /// Safety
    /// -------------------
    /// The vtable shall be a dereferenceable pointer.
    /// The data shall be a dereferenceable pointer which is valid for read for at least `size` from the vtable,
    ///  and shall be aligned to at least align, additionally, it shall not be accessed from any other region
    ///  for the lifetime of the reference
    unsafe fn deref_mut<'a>(self) -> <Self as StablePointerLifetime<'a,Trait>>::MutReference
        where Trait: 'a;


    /// Converts between different StablePointer types.
    ///
    /// Safety
    /// --------------
    /// The target pointer type may validly impose a non-null restriction.
    ///  If such a restriction is imposed, neither data nor the vtable may be a null pointer.
    ///
    unsafe fn into_other<P: StablePointer<Trait>>(self) -> P{
        core::mem::transmute_copy(&self)
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
pub unsafe trait StableReference<'a,Trait: StableVTableTrait +'a + ?Sized>: 'a {
    type Pointer: StablePointer<Trait>;
    ///
    /// Obtains the size of the referenced value
    fn size_of_val(&self) -> usize where Trait: 'a;
    ///
    /// Obtains the align of the referenced value
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
pub unsafe trait StableMutable<'a,Trait: StableVTableTrait +'a + ?Sized>: StableReference<'a,Trait>{}

