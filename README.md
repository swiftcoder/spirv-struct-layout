# spirv-struct-layout

Attempts to ensure that a rust struct used as a uniform buffer matches the layout of the struct declared in spirv.

Usage example:
```
use spirv_struct_layout::{CheckSpirvStruct, SpirvLayout};

#[repr(C)]
#[derive(SpirvLayout)]
struct Uniforms {
    model_view: [f32; 16],
    light_dir: [f32; 3],
    // _padding: f32, // uncomment this line, and the alignment will match the spirv
    position: [f32; 4],
}


fn main() {
    let spirv = Vec::from(cast_slice(include_bytes!("simple.frag.spv")));

    Uniforms::check_spirv_layout("buf", spirv);
}
```

Which fails like so, becuase SPIR-V mandates that vec3 is aligned to 16 bytes:
```
The application panicked (crashed).
Message:  assertion failed: `(left == right)`
  left: `80`,
 right: `76`: field buf.position should have an offset of 80 bytes, but was 76 bytes
Location: spirv_struct_layout/examples/simple/main.rs:19
```
