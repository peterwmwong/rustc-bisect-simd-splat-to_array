Cargo project reproducing compiler error using [`Simd::splat`](https://doc.rust-lang.org/std/simd/struct.Simd.html#method.splat)/[`Simd::to_array`](https://doc.rust-lang.org/std/simd/struct.Simd.html#method.to_array) in release mode.

```rs
#![feature(portable_simd)]
pub fn test() -> [i32; 2] {
    ::std::simd::i32x2::splat(1).to_array()
    // ...but the following does not cause compiler to error
    // ::std::simd::i32x2::from_array([1; 2]).to_array()
}
```

```
cargo b --release
   Compiling simd-rustc-bisect-simd-splat-to_array v0.1.0 (/Users/pwong/projects/cargo-bisect-rustc/rustc-bisect-simd-splat-to_array)
error: could not compile `simd-rustc-bisect-simd-splat-to_array` (lib)

Caused by:
  process didn't exit successfully: `rustc --crate-name simd_rustc_bisect_simd_splat_to_array --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=241 --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C metadata=38baaff510ce154a -C extra-filename=-38baaff510ce154a --out-dir /Users/pwong/projects/cargo-bisect-rustc/rustc-bisect-simd-splat-to_array/target/release/deps -L dependency=/Users/pwong/projects/cargo-bisect-rustc/rustc-bisect-simd-splat-to_array/target/release/deps` (signal: 5, SIGTRAP: trace/breakpoint trap)
```

As of 2023-04-22, reproducible on [Rust Playground - Nightly/Release](https://play.rust-lang.org/?version=nightly&mode=release&edition=2021&gist=75f304c7759fbebb55bc7bee5dc37a5e)

Cargo Bisect Rustc points to this [commit - Permit MIR inlining without #[inline]](https://github.com/rust-lang/rust/commit/5546cb64f6fbba70529582bbe58a40ba4a8ed9fc)

    cargo bisect-rustc --start=2023-04-17 --end=2023-04-19 -- build --release

<details>
<summary>
Bisect Output
</summary>

    cargo bisect-rustc --start=2023-04-17 --end=2023-04-19 -- build --release
    checking the start range to find a passing nightly
    installing nightly-2023-04-17
    rust-std-nightly-aarch64-apple-darwin: 25.62 MB / 25.62 MB [================================================================================================================================================================] 100.00 % 9.98 MB/s testing...
    RESULT: nightly-2023-04-17, ===> No
    uninstalling nightly-2023-04-17

    checking the end range to verify it does not pass
    installing nightly-2023-04-19
    rust-std-nightly-aarch64-apple-darwin: 26.08 MB / 26.08 MB [===============================================================================================================================================================] 100.00 % 10.61 MB/s testing...
    RESULT: nightly-2023-04-19, ===> Yes
    uninstalling nightly-2023-04-19

    1 versions remaining to test after this (roughly 1 steps)
    installing nightly-2023-04-18
    rust-std-nightly-aarch64-apple-darwin: 26.02 MB / 26.02 MB [===============================================================================================================================================================] 100.00 % 10.64 MB/s testing...
    RESULT: nightly-2023-04-18, ===> Yes
    uninstalling nightly-2023-04-18

    searched toolchains nightly-2023-04-17 through nightly-2023-04-19


    ********************************************************************************
    Regression in nightly-2023-04-18
    ********************************************************************************

    fetching https://static.rust-lang.org/dist/2023-04-17/channel-rust-nightly-git-commit-hash.txt
    nightly manifest 2023-04-17: 40 B / 40 B [================================================================================================================================================================================] 100.00 % 876.98 KB/s converted 2023-04-17 to d0f204e4d750b62f9d6c2593405e828757126832
    fetching https://static.rust-lang.org/dist/2023-04-18/channel-rust-nightly-git-commit-hash.txt
    nightly manifest 2023-04-18: 40 B / 40 B [================================================================================================================================================================================] 100.00 % 602.51 KB/s converted 2023-04-18 to 7908a1d65496b88626e4b7c193c81d777005d6f3
    looking for regression commit between 2023-04-17 and 2023-04-18
    opening existing repository at "/Users/pwong/projects/rust"
    Found origin remote under name `origin`
    refreshing repository at "/Users/pwong/projects/rust"
    fetching (via local git) commits from d0f204e4d750b62f9d6c2593405e828757126832 to 7908a1d65496b88626e4b7c193c81d777005d6f3
    opening existing repository at "/Users/pwong/projects/rust"
    Found origin remote under name `origin`
    refreshing repository at "/Users/pwong/projects/rust"
    looking up first commit
    looking up second commit
    checking that commits are by bors and thus have ci artifacts...
    finding bors merge commits
    found 9 bors merge commits in the specified range
    commit[0] 2023-04-16: Auto merge of #109133 - weihanglo:make-cargo-a-workspace, r=ehuss
    commit[1] 2023-04-17: Auto merge of #109061 - saethlin:leak-backtraces, r=oli-obk
    commit[2] 2023-04-17: Auto merge of #109247 - saethlin:inline-without-inline, r=oli-obk
    commit[3] 2023-04-17: Auto merge of #109588 - Nilstrieb:dropless-expr, r=compiler-errors
    commit[4] 2023-04-17: Auto merge of #110440 - matthiaskrgr:rollup-eit19vi, r=matthiaskrgr
    commit[5] 2023-04-17: Auto merge of #110367 - saethlin:no-truncations, r=oli-obk
    commit[6] 2023-04-17: Auto merge of #110458 - matthiaskrgr:rollup-1xcxmgc, r=matthiaskrgr
    commit[7] 2023-04-17: Auto merge of #110343 - saethlin:encode-initmask, r=lqd
    commit[8] 2023-04-17: Auto merge of #110243 - WaffleLapkin:bless_tagged_pointers🙏, r=Nilstrieb
    validated commits found, specifying toolchains

    checking the start range to verify it passes
    installing d0f204e4d750b62f9d6c2593405e828757126832
    rust-std-nightly-aarch64-apple-darwin: 26.65 MB / 26.65 MB [===============================================================================================================================================================] 100.00 % 10.52 MB/s testing...
    RESULT: d0f204e4d750b62f9d6c2593405e828757126832, ===> No
    uninstalling d0f204e4d750b62f9d6c2593405e828757126832

    checking the end range to verify it does not pass
    installing 7908a1d65496b88626e4b7c193c81d777005d6f3
    rust-std-nightly-aarch64-apple-darwin: 27.06 MB / 27.06 MB [===============================================================================================================================================================] 100.00 % 10.69 MB/s testing...
    RESULT: 7908a1d65496b88626e4b7c193c81d777005d6f3, ===> Yes
    uninstalling 7908a1d65496b88626e4b7c193c81d777005d6f3

    4 versions remaining to test after this (roughly 2 steps)
    installing bdb32bd4bbcabb0d32a04a0b45e6a8ceaa5e54d6
    rust-std-nightly-aarch64-apple-darwin: 27.06 MB / 27.06 MB [===============================================================================================================================================================] 100.00 % 10.47 MB/s testing...
    RESULT: bdb32bd4bbcabb0d32a04a0b45e6a8ceaa5e54d6, ===> Yes
    uninstalling bdb32bd4bbcabb0d32a04a0b45e6a8ceaa5e54d6

    2 versions remaining to test after this (roughly 1 steps)
    installing 5546cb64f6fbba70529582bbe58a40ba4a8ed9fc
    rust-std-nightly-aarch64-apple-darwin: 27.06 MB / 27.06 MB [===============================================================================================================================================================] 100.00 % 10.47 MB/s testing...
    RESULT: 5546cb64f6fbba70529582bbe58a40ba4a8ed9fc, ===> Yes
    uninstalling 5546cb64f6fbba70529582bbe58a40ba4a8ed9fc

    1 versions remaining to test after this (roughly 1 steps)
    installing 23eb90ffa77943153d203c3d184c182490d758e7
    rust-std-nightly-aarch64-apple-darwin: 26.61 MB / 26.61 MB [===============================================================================================================================================================] 100.00 % 10.66 MB/s testing...
    RESULT: 23eb90ffa77943153d203c3d184c182490d758e7, ===> No
    uninstalling 23eb90ffa77943153d203c3d184c182490d758e7

    searched toolchains d0f204e4d750b62f9d6c2593405e828757126832 through 7908a1d65496b88626e4b7c193c81d777005d6f3


    ********************************************************************************
    Regression in 5546cb64f6fbba70529582bbe58a40ba4a8ed9fc
    ********************************************************************************

    Attempting to search unrolled perf builds
    ERROR: couldn't find perf build comment
    ==================================================================================
    = Please file this regression report on the rust-lang/rust GitHub repository     =
    =        New issue: https://github.com/rust-lang/rust/issues/new                 =
    =     Known issues: https://github.com/rust-lang/rust/issues                     =
    = Copy and paste the text below into the issue report thread.  Thanks!           =
    ==================================================================================

    searched nightlies: from nightly-2023-04-17 to nightly-2023-04-19
    regressed nightly: nightly-2023-04-18
    searched commit range: https://github.com/rust-lang/rust/compare/d0f204e4d750b62f9d6c2593405e828757126832...7908a1d65496b88626e4b7c193c81d777005d6f3
    regressed commit: https://github.com/rust-lang/rust/commit/5546cb64f6fbba70529582bbe58a40ba4a8ed9fc
</details>