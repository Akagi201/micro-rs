# micro-rs

[![Rust Check & Build](https://github.com/Akagi201/micro-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Akagi201/micro-rs/actions/workflows/ci.yml)

Micro services in Rust

## OpenAPI

- [ ] [Axum](https://github.com/tokio-rs/axum) - web framework
- [ ] [utoipa](https://github.com/juhaku/utoipa) - auto generated openapi documentation

## GRPC

- [ ] [tonic](https://github.com/hyperium/tonic) - grpc framework

## OpenRPC

TODO

## Common Libs

- [ ] [shadow-rs](https://github.com/baoyachi/shadow-rs) - build time information
- [ ] [serde_with](https://github.com/jonasbb/serde_with) - serde helpers
- [ ] [tokio](https://tokio.rs/) - async runtime
- [ ] [time](https://github.com/time-rs/time) - time handling
- [ ] [tracing](https://github.com/tokio-rs/tracing) - tracing and log
- [ ] [SeaOrm](https://github.com/SeaQL/sea-orm) - db orm
- [ ] [tabled](https://github.com/zhiburt/tabled) - pretty print
- [ ] [shadow-rs](https://github.com/baoyachi/shadow-rs) - build info
- [ ] [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - system cpu/mem info
- [ ] [sentry](https://github.com/getsentry/sentry-rust) - sentry profiling
- [ ] [flamegraph](https://github.com/flamegraph-rs/flamegraph) - flame graph
- [ ] [num_traits](https://docs.rs/num-traits/latest/num_traits/) - num traits for generic math
- [ ] [dashmap](https://docs.rs/dashmap/latest/dashmap/) - concurrent hashmap
- [ ] [once_cell](https://github.com/matklad/once_cell) - lazy static without macros
- [ ] [charming](https://github.com/yuankunzhang/charming) - visualization
- [ ] [auto_impl](https://github.com/auto-impl-rs/auto_impl) - Automatically implement traits for common smart pointers
- [ ] [bytes](https://docs.rs/bytes/latest/bytes/)
- [ ] [async-trait](https://github.com/dtolnay/async-trait)
- [ ] [cacache](https://github.com/zkat/cacache-rs) - disc cache
- [ ] [r2d2](https://github.com/sfackler/r2d2) - db generic connection pool
- [ ] [redis](https://github.com/redis-rs/redis-rs) - redis client
- [ ] [async-compat](https://github.com/smol-rs/async-compat) - Compatibility adapter between tokio and futures
- [ ] [autometrics](https://github.com/autometrics-dev/autometrics-rs) - metrics and tracing
- [ ] [lazy_format](https://docs.rs/lazy_format) - lazy formatting
- [ ] [convi](https://github.com/dpc/convi) - safe conversion
- [ ] [tap](https://docs.rs/tap/latest/tap/) - position pipeline
- [ ] [resiter](https://docs.rs/resiter/latest/resiter/) - iterator helper
- [ ] [pretty-assertions](https://github.com/rust-pretty-assertions/rust-pretty-assertions) - pretty assertions
- [ ] [trybuild](https://github.com/dtolnay/trybuild) - try build in tests
- [ ] [insta](https://github.com/mitsuhiko/insta) - snapshot testing
- [ ] [criterion](https://github.com/bheisler/criterion.rs) - benchmark testing
- [ ] [divan](https://github.com/nvzqz/divan) - benchmark testing
- [ ] [arbitrary](https://github.com/rust-fuzz/arbitrary) - construct arbitrary instances of a type
- [ ] [assert_fs](https://github.com/assert-rs/assert_fs) - Filesystem fixtures and assertions for testing
- [ ] [no-panics](https://github.com/dtolnay/no-panic) - prove no panics in build phase
- [ ] [educe](https://github.com/magiclen/educe) - procedural macros to help you implement Rust build-int traits quickly.
- [ ] [validator](https://github.com/Keats/validator) - struct field validation
- [ ] [tap](https://github.com/myrrlyn/tap) - tapping values in method chains
- [ ] [lazy-static.rs](https://github.com/rust-lang-nursery/lazy-static.rs) - macro for defining lazy evaluated static variables
- [ ] [joinery](https://github.com/Lucretiel/joinery) - join iterables with a separator
- [ ] [chrono](https://github.com/chronotope/chrono) - time handling
- [ ] [time](https://github.com/time-rs/time) - time handling
- [ ] [pin-project](https://github.com/taiki-e/pin-project) - pin projection
- [ ] [soa-derive](https://github.com/lumol-org/soa-derive) - Transform AOS to SOA (Struct of Arrays)
- [ ] [derive_more](https://github.com/JelteF/derive_more) - derive traits helper
- [ ] [rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder) - generate struct builder
- [ ] [fakeit](https://github.com/PumpkinSeed/fakeit) - gake data generator
- [ ] [proptest](https://github.com/proptest-rs/proptest) - Hypothesis-like property testing
- [ ] [trybuild](https://github.com/dtolnay/trybuild) - Test that certain code variants do not compile.
- [ ] [embassy](https://github.com/embassy-rs/embassy) - async next-generation framework for embedded applications
- [ ] [postcard](https://github.com/jamesmunns/postcard) - A serde flavor that's tailored for targets have less ram than our dev machines have cache.
- [ ] [slotmap](https://github.com/orlp/slotmap) - map with typed, unique keys
- [ ] [mockall](https://github.com/asomers/mockall) - mock object
- [ ] [strum](https://github.com/Peternator7/strum) - enum strings and such
- [ ] [inherent](https://github.com/dtolnay/inherent) - make trait methods callable without the trait in scope
- [ ] [rust-smallvec](https://github.com/servo/rust-smallvec) - Small vector
- [ ] [smallstr](https://github.com/murarth/smallstr) - small str
- [ ] [compact_str](https://github.com/ParkMyCar/compact_str) - small str on the stack
- [ ] [bstr](https://github.com/BurntSushi/bstr) - byte string
- [ ] [itertools](https://github.com/rust-itertools/itertools) - iterator helpers
- [ ] [camino](https://github.com/camino-rs/camino) - Like Rust's std::path::Path, but UTF-8
- [ ] [bytemuck](https://github.com/Lokathor/bytemuck) - Cast bytes to types safely
- [ ] [nutype](https://github.com/greyblake/nutype) - Rust newtype with guarantees
- [ ] [job_scheduler](https://github.com/BlackDex/job_scheduler) - Job scheduler
- [ ] [clokwerk](https://github.com/mdsherry/clokwerk) - Job scheduler
- [ ] [delay-timer](https://github.com/BinChengZhao/delay-timer) - Job scheduler based on time wheel algorithm

## Static Analysis

- [ ] [lockbud](https://github.com/BurtonQin/lockbud) - dead lock detection
- [ ] <https://github.com/RalfJung/cargo-careful> - extra checks

## Tools

- [taplo](https://github.com/tamasfe/taplo) - toml toolkit
- [shuttle-rs](https://docs.shuttle.rs/) - devops
- [squawk](https://github.com/sbdchd/squawk) - postgres sql linter
- [cross](https://github.com/cross-rs/cross) - cross build tool
- [kani](https://github.com/model-checking/kani) - unsafe rust model checker
- [cargo-cleanall](https://github.com/LeSnake04/cargo-cleanall) - clean all cargo build artifacts
- [cargo-clean-all](https://github.com/dnlmlr/cargo-clean-all) - another tool for clean all cargo build artifacts
- [kondo](https://github.com/tbillington/kondo) - clean up node_modules, target
- [nextest](https://github.com/nextest-rs/nextest) - next-generation test runner
- [cargo-release](https://github.com/crate-ci/cargo-release) - release tool
- [git-cliff](https://github.com/orhun/git-cliff) - changelog generator
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) - cargo deny
- [mles-rs](https://github.com/jq-rs/mles-rs) - message queue

## Infras

- [quickwit](https://github.com/quickwit-oss/quickwit) - tracing

## Nice Docs

- [concurrency-programming-via-rust](https://github.com/smallnest/concurrency-programming-via-rust)
- [axum demo tutor](https://github.com/joelparkerhenderson/demo-rust-axum)
- [Lifetime Kata](https://tfpk.github.io/lifetimekata/)
- [Macro kata](https://tfpk.github.io/macrokata/)
- [proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
- [modern rust](https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Rust ecosystem](https://blessed.rs/crates)

## Nice videos

- [crust of rust](https://www.youtube.com/@jonhoo/videos)

## Nice Books

- [Rust Book Abridged](https://jasonwalton.ca/rust-book-abridged/) - condensed version of The Rust Programming Language
- [Rust on Nails](https://rust-on-nails.com/) - Full stack Rust book
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html) - The Dark Arts of Advanced and Unsafe Rust Programming
- [Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html) - The Little Book of Rust Macros
- [Google Comprehensive Rust](https://google.github.io/comprehensive-rust/) - Google Comprehensive Rust
- [Rust Atomics and Locks](https://marabos.nl/atomics/) - Rust Atomics and Locks
- [Writing Interpreters in Rust: a Guide](https://rust-hosted-langs.github.io/book/introduction.html) - Writing Interpreters in Rust: a Guide
- [Easy Rust](https://dhghomon.github.io/easy_rust/Chapter_1.html) - Easy Rust and some best practice.
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/title-page.html) - The Rust Performance Book
- <https://github.com/m-ou-se/rust-atomics-and-locks>
- <https://marabos.nl/atomics/>
- <https://github.com/rustcc/Rust_Atomics_and_Locks>
- <https://quinedot.github.io/rust-learning/lifetime-intuition.html>
- <https://bheisler.github.io/criterion.rs/book/index.html>
- <https://github.com/skerkour/black-hat-rust>
- <https://github.com/wiseaidev/dark-web-rust>
