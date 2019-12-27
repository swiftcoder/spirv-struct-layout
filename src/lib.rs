pub trait CheckSpirvStruct {
    /// Check this struct against the named instance in the given spirv
    fn check(name: &str, spirv: Vec<u32>);
}

#[macro_export]
macro_rules! spriv_struct {
    ($type_name:ident { $($field:ident : $type:ty),+ $(,)? }) => {
        #[allow(dead_code)]
        struct $type_name {
            $($field : $type),+
        }

        impl crate::CheckSpirvStruct for $type_name {
            fn check(name: &str, spirv: Vec<u32>) {
                let spv: spirq::SpirvBinary = spirv.into();
                let entries = spv.reflect().unwrap();

                let mut rust_offset = 0;
                $(
                    {
                        let symbol = name.to_owned() + "." + stringify!($field);
                        let rust_size = std::mem::size_of::<$type>();

                        if let Some((offset, var_ty)) = entries[0].resolve_desc(spirq::sym::Sym::new(&symbol)) {
                            let spirv_offset = offset.unwrap();
                            let spirv_size = var_ty.nbyte().unwrap();

                            assert_eq!(
                                spirv_size, rust_size,
                                "field {} should be {} bytes, but was {} bytes",
                                symbol, spirv_size, rust_size
                            );
                            assert_eq!(
                                spirv_offset, rust_offset,
                                "field {} should have an offset of {} bytes, but was {} bytes",
                                symbol, spirv_offset, rust_offset
                            );
                        }

                        rust_offset += rust_size;
                    }
                );+
            }
        }
    };
}
