use crate::traits::StableVTableTrait;
use crate::ptr::StableNonNull;

pub struct Box<Trait: StableVTableTrait>{
    ptr: StableNonNull<Trait>
}