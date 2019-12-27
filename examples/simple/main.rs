use spirv_struct_layout::{spriv_struct, CheckSpirvStruct};

// This is obnoxious. We need an include_words! macro to keep things aligned nicely
fn cast_slice(v: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u32,
            v.len() / std::mem::size_of::<u32>(),
        )
    }
}

spriv_struct!(Uniforms {
    model_view: [f32; 16],
    light_dir: [f32; 3],
    _padding: f32, // comment this line, and the alignment will no longer match the spirv
    position: [f32; 4],
});

fn main() {
    color_backtrace::install();

    let spirv = Vec::from(cast_slice(include_bytes!("simple.frag.spv")));

    Uniforms::check("buf", spirv);

    println!("I guess the struct is laid out correctly.");
}
