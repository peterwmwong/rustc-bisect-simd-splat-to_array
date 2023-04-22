#![feature(portable_simd)]
pub fn test() -> [i32; 2] {
    ::std::simd::i32x2::splat(1).to_array()
    // ...but the following does not cause compiler to error
    // ::std::simd::i32x2::from_array([1; 2]).to_array()
}
