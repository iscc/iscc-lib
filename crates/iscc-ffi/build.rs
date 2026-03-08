//! Build script for iscc-ffi — generates C# P/Invoke bindings via csbindgen.

fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("iscc_ffi")
        .csharp_class_name("NativeMethods")
        .csharp_namespace("Iscc.Lib")
        .generate_csharp_file("../../packages/dotnet/Iscc.Lib/NativeMethods.g.cs")
        .unwrap();
}
