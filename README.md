# spirv-struct-layout

Attempts to ensure that a rust struct used as a uniform buffer matches the layout of the struct declared in spirv.

Usage example:
```
use spirv_struct_layout::{spriv_struct, CheckSpirvStruct};

spriv_struct!(Uniforms {
    model_view: [f32; 16],
    light_dir: [f32; 3],
    _padding: f32, // comment this line, and the alignment will no longer match the spirv
    position: [f32; 4],
});

fn main() {
    let spirv = Vec::from(cast_slice(include_bytes!("simple.frag.spv")));

    Uniforms::check("buf", spirv);
}
```