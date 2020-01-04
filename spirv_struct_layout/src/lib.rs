pub use spirv_struct_layout_derive::*;

/// Trait for structs that should have their layout checked against a SPIRV type
pub trait CheckSpirvStruct {
    /// Check this struct against the named instance in the given spirv
    fn check_spirv_layout(name: &str, spirv: Vec<u32>);
}
