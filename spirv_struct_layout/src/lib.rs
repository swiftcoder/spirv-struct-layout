pub use spirv_struct_layout_derive::*;

pub trait CheckSpirvStruct {
    /// Check this struct against the named instance in the given spirv
    fn check(name: &str, spirv: Vec<u32>);
}
