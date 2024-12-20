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
- [ ] [malachite-bigint](https://github.com/RustPython/malachite-bigint) - a faster drop-in replacement for num-bigint
- [ ] [dashmap](https://docs.rs/dashmap/latest/dashmap/) - concurrent hashmap, !!may dead lock <https://dev.to/acter/beware-of-the-dashmap-deadlock-lij>
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
- [ ] [pinned-init](https://github.com/Rust-for-Linux/pinned-init) - Library facilitating safe pinned initialization
- [ ] [soa-derive](https://github.com/lumol-org/soa-derive) - Transform AOS to SOA (Struct of Arrays)
- [ ] [soapy](https://github.com/tim-harding/soapy/) - another SOA macro
- [ ] [derive_more](https://github.com/JelteF/derive_more) - derive traits helper
- [ ] [rust-derive-builder](https://github.com/colin-kiegel/rust-derive-builder) - generate struct builder, builder pattern
- [ ] [bon](https://github.com/elastio/bon) - generate builder for functions and structs
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
- [ ] [tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) - tokio cron scheduler
- [ ] [cronframe](https://github.com/antcim/cronframe) - cron
- [ ] [clokwerk](https://github.com/mdsherry/clokwerk) - Job scheduler
- [ ] [delay-timer](https://github.com/BinChengZhao/delay-timer) - Job scheduler based on time wheel algorithm
- [ ] [enum_delegate](https://gitlab.com/dawn_app/enum_delegate) - easily replace dynamic dispatch with an enum.
- [ ] [auto_enums](https://github.com/taiki-e/auto_enums) - allow multiple return types by automatically generated enum
- [ ] [scopeguard](https://github.com/bluss/scopeguard) - like defer in golang
- [ ] [iceoryx2](https://github.com/eclipse-iceoryx/iceoryx2) - IPC
- [ ] [ratatui](https://github.com/ratatui-org/ratatui) - TUI
- [ ] [downcast-rs](https://github.com/marcianx/downcast-rs) - cast trait objects back to the original concrete types
- [ ] [compact_str](https://github.com/ParkMyCar/compact_str) - drop in replacement of String.
- [ ] [uint](https://github.com/recmo/uint) - Uint crate with generics
- [ ] [ryu](https://github.com/dtolnay/ryu) - float to string conversion
- [ ] [itoa](https://github.com/dtolnay/itoa) - integer to string conversion
- [ ] [winnow](https://github.com/winnow-rs/winnow) - write parser by hand.
- [ ] [parse-display](https://github.com/frozenlib/parse-display) - derive macro, Display and FromStr
- [ ] [miette](https://github.com/zkat/miette) - Fancy extension for std::error::Error with pretty, detailed diagnostic printing.
- [ ] [ariadne](https://github.com/zesterer/ariadne) - A fancy diagnostics & error reporting crate
- [ ] [nolife](https://github.com/dureuill/nolife) - construct a struct that contain reference without a lifetime.
- [ ] [abi_stable](https://github.com/rodrimati1992/abi_stable_crates/) - rust plugin system
- [ ] [RustFFT](https://github.com/ejmahler/RustFFT) - SIMD FFT
- [ ] [ferrilab](https://github.com/ferrilab/ferrilab) - bitvec related crates
- [ ] [rug](https://gitlab.com/tspiteri/rug) - integers and floating-point numbers with arbitrary precision
- [ ] [cve-rs](https://github.com/Speykious/cve-rs) - memory vulnerabilities
- [ ] [thread-manager](https://github.com/syn-chromatic/thread-manager-rs) - thread manager
- [ ] [happylock](https://crates.io/crates/happylock) - lock free mutex
- [ ] [listeners](https://github.com/GyulyVGC/listeners) - get processes listening on a TCP port
- [ ] [testresult](https://github.com/wiktor-k/testresult) - anyhow for tests
- [ ] [fastbloom](https://github.com/tomtomwombat/fastbloom/) - bloom filter
- [ ] [symbolica](https://github.com/benruijl/symbolica) - computer algebra system
- [ ] [deku](https://github.com/sharksforarms/deku) - bit-level serialization/deserialization implementations for structs and enums
- [ ] [profi](https://github.com/lyonsyonii/profi) - multi-threaded profiler
- [ ] [try-iterator](https://github.com/rodrigocfd/try-iterator) - TryIterator trait
- [ ] [notify-rust](https://github.com/hoodie/notify-rust) - desktop notification
- [ ] [strafe](https://gitlab.com/Neek-sss/strafe) - statistics
- [ ] [kameo](https://github.com/tqwewe/kameo) - actor model on tokio
- [ ] [pptr](https://github.com/ribelo/pptr)- actor model on tokio
- [ ] [xtra](https://github.com/Restioson/xtra) - actor model on any runtime
- [ ] [eyre](https://github.com/eyre-rs/eyre) - error handling, anyhow fork, better than anyhow
- [ ] [scc](https://github.com/wvwwvwwv/scalable-concurrent-containers) - concurrent containers, better than dashmap
- [ ] [wiring](https://github.com/louaykamel/wiring) - async serialization
- [ ] [nutype](https://github.com/greyblake/nutype) - proc macro that allows adding extra constraints like sanitization and validation
- [ ] [unfmt](https://github.com/mathematic-inc/unfmt) - pattern matching library that reverses the interpolation process of `format!`.
- [ ] [enum_dispatch](https://gitlab.com/antonok/enum_dispatch) - transforms your trait objects to static dispatch.
- [ ] [archspec](https://github.com/prefix-dev/archspec-rs) - detect CPU architecture
- [ ] [serde-sqlite-jsonb](https://github.com/lovasoa/serde-sqlite-jsonb) - sqlit jsonb serde
- [ ] [futures-concurrency](https://github.com/yoshuawuyts/futures-concurrency) - Structured concurrency operations for async Rust
- [ ] [shiva](https://github.com/igumnoff/shiva) - generate and parse many type of documents
- [ ] [ffmpeg-sidecar](https://github.com/nathanbabcock/ffmpeg-sidecar) - ffmpeg binary wrapper
- [ ] [medium-to-markdown](https://github.com/Harshil-Jani/medium-to-markdown) - parse medium article to markdown, design: <https://medium.com/@harshiljani2002/a-parser-in-rust-to-convert-your-medium-blogs-to-markdown-84173a6c1300>
- [ ] [declarative_enum_dispatch](https://github.com/Zettroke/declarative_enum_dispatch) - dynamic dispatch of a trait using an enum
- [ ] [rauthy](https://github.com/sebadob/rauthy) - authentication
- [ ] [balterloadtesting](https://github.com/BalterLoadTesting/balter) - load testing framework
- [ ] [faster-hex](https://github.com/nervosnetwork/faster-hex) - faster hex encoding and decoding
- [ ] [prefixes](https://github.com/synek317/prefixes) - simplest proc macros example, attribute-like macros
- [ ] [unwrap_or_log](https://github.com/ninomerlino/unwrap_or_log) - unwrap or log
- [ ] [serde-save](https://github.com/aatifsyed/serde-save) - serialization tree for serde, more information than serde-value.
- [ ] [valuable](https://github.com/tokio-rs/valuable) - object-safe value inspection
- [ ] [schemars](https://github.com/GREsau/schemars) - generate JSON Schema from rust strut
- [ ] [nunny](https://github.com/aatifsyed/nunny) - non-empty vec
- [ ] [subenum](https://github.com/paholg/subenum) - proc macro to make sub enum
- [ ] [deadline](https://github.com/niklaslong/deadline) - Requires a condition closure to return `true` before the specified duration has elapsed
- [ ] [transitive](https://github.com/bobozaur/transitive) - Transitive converions through derive macros
- [ ] [polyit-rs](https://github.com/SkyeC0re/polyit-rs) - A no-std library for manipulating polynomials
- [ ] [comrak](https://github.com/kivikakk/comrak) - markdown parser and renderer
- [ ] [arc-swap](https://github.com/vorner/arc-swap) - atomic operations on Arc.
- [ ] [rpds](https://github.com/orium/rpds) - Rust Persistent Data Structures
- [ ] [haiku-search](https://github.com/beowolx/haiku-search) - wasm fuzzy search library
- [ ] [petgraph](https://github.com/petgraph/petgraph) - Graph data structure library
- [ ] [rustwire](https://github.com/Basis-Health/rustwire) - decode protobuf message
- [ ] [iroh](https://github.com/n0-computer/iroh) - tookit to build p2p app, with QUIC
- [ ] [statrs](https://github.com/statrs-dev/statrs) - statistical scientific computing
- [ ] [fjall](https://github.com/fjall-rs/fjall) - LSM-based embeddeable key-value db in safe rust
- [ ] [redb](https://github.com/cberner/redb) - ACID, COW, B-tree embeddeable embeddeable key-value db
- [ ] [faer-rs](https://github.com/sarah-ek/faer-rs) - Linear algebra
- [ ] [nalgebra](https://github.com/dimforge/nalgebra) - Linear algebra
- [ ] [ndarray](https://github.com/rust-ndarray/ndarray) - N-dimensional array
- [ ] [glam-rs](https://github.com/bitshifter/glam-rs) - 3d math
- [ ] [glamour](https://github.com/simonask/glamour) - glam wrapper
- [ ] [varpro](https://github.com/geo-ant/varpro) - nonlinear models to data
- [ ] [impl-for](https://github.com/Bajix/impl-for) - repeating implementations with type substitutions
- [ ] [aqueducts](https://github.com/vigimite/aqueducts) - ETL pipeline
- [ ] [cacache-rs](https://github.com/zkat/cacache-rs) - concurrent async disk cache
- [ ] [portpicker-rs](https://github.com/Dentosal/portpicker-rs) - pick a free unused port
- [ ] [bool-to-bitflags](https://github.com/gnomeddev/bool-to-bitflags) - compact multiple bools into a single bitflags field
- [ ] [rerun](https://github.com/rerun-io/rerun) - visualization of multimodal data
- [ ] [russh](https://github.com/warp-tech/russh) - Rust SSH client & server library
- [ ] [cliclack](https://github.com/fadeevab/cliclack) - cli prompt, prettier
- [ ] [inquire](https://github.com/mikaelmello/inquire) - cli prompt, feature rich
- [ ] [protect-endpoints](https://github.com/DDtKey/protect-endpoints) - auth extension for web frameworks
- [ ] [may](https://github.com/Xudong-Huang/may) - stackful coroutine
- [ ] [rust-bridge](https://github.com/postgresml/postgresml/tree/master/pgml-sdks/rust-bridge) - generate bindings for python/js/c
- [ ] [unfmt](https://github.com/mathematic-inc/unfmt) - reverse process of format!
- [ ] [automerge](https://github.com/automerge/automerge) - CRDT
- [ ] [octocrab](https://github.com/XAMPPRocky/octocrab) - github api
- [ ] [tracing-bunyan-formatter](https://github.com/LukeMathWalker/tracing-bunyan-formatter) - json log
- [ ] [json-subscriber](https://github.com/mladedav/json-subscriber) - json log
- [ ] [russh](https://github.com/warp-tech/russh) - async tokio ssh
- [ ] [indexed_priority_queue](https://github.com/binary-banter/indexed_priority_queue) - indexed priority queue
- [ ] [openfhe-rs](https://github.com/fairmath/openfhe-rs) - a OpenFHE wrapper.
- [ ] [rust2go](https://github.com/ihciah/rust2go) - rust call go.
- [ ] [gerevs](https://github.com/itamarsch/gerevs) - socks5 proxy
- [ ] [tower-surf](https://github.com/quiet-camp/tower-surf) - tower CSRF middleware
- [ ] [tower-http-client](https://github.com/alekseysidorov/tower-http-client) - tower http client middleware
- [ ] [serdev](https://github.com/ohkami-rs/serdev) - serde with validation
- [ ] [backon](https://github.com/Xuanwo/backon) - retry
- [ ] [plotlars](https://github.com/alceal/plotlars) - plotly wrapper
- [ ] [dasm](https://github.com/DvvCz/dasm) - x86/x64 assembler
- [ ] [iced](https://github.com/icedland/iced) - x86/x64 assembler disassembeler
- [ ] [wrappers](https://github.com/supabase/wrappers) - development framework for Postgres Foreign Data Wrappers(FDW)
- [ ] [moka](https://github.com/moka-rs/moka) - a fast, concurrent cache library
- [ ] [rkyv](https://github.com/rkyv/rkyv) - zero-copy deserialization framework
- [ ] [rust-lexical](https://github.com/Alexhuszagh/rust-lexical) - fast numeric and string conversion
- [ ] [chili](https://github.com/dragostis/chili) - rust port of spice, a low-overhead parallelization library.
- [ ] [rustynum](https://github.com/IgorSusmelj/rustynum) - rust numpy with SIMD
- [ ] [skiplist-rust](https://github.com/monadbobo/skiplist-rust) - skiplist inspired by leveldb
- [ ] [ureq](https://github.com/algesten/ureq) - lightweight sync http client
- [ ] [pumps_rs](https://github.com/alexpusch/pumps_rs) - future stream api
- [ ] [blst](https://github.com/supranational/blst) - BLS12-381 signature library written in C and assembly.
- [ ] [tokio-graceful](https://github.com/plabayo/tokio-graceful) - Graceful shutdown for tokio project
- [ ] [stochastic-rs](https://github.com/dancixx/stochastic-rs) - high-performance simulation and analysis of stochastic processes and models in quant finance
- [ ] [flume](https://github.com/zesterer/flume) - mpmc channel
- [ ] [constantine](https://github.com/mratsim/constantine) - cryptographic primitives used in zkp
- [ ] [Integrate](https://github.com/mtantaoui/Integrate) - numerical integration of real-valued functions
- [ ] [confy](https://github.com/rust-cli/confy) - configuration management
- [ ] [jiff](https://github.com/BurntSushi/jiff) - high-level datetime library
- [ ] [askama](https://github.com/rinja-rs/askama) - type-safe, compiled Jinja-like templates
- [ ] [embive](https://github.com/embive/embive) - risc-v interpreter

## Testing

- [httpmock](https://github.com/alexliesenfeld/httpmock) - http mock 
- [mockito](https://github.com/lipanski/mockito) - http mock

## Framework

- <https://github.com/ntex-rs/ntex> - framework for networking service
- <https://github.com/emanguy/rust-rest> - rest template
- <https://github.com/plabayo/rama> - proxy framework
- <https://github.com/cloudflare/pingora> - proxy framework
- <https://github.com/albbus-stack/t5.rs> - full rust cross platform framework
- <https://github.com/r3bl-org/r3bl-open-core> - tui framework
- <https://github.com/holo-routing/holo> - micro service framework
- <https://github.com/meteroid-oss/meteroid> - grpc micro service saas product
- <https://github.com/barter-rs/barter-rs> - like ccxt, but in rust
- <https://github.com/gosub-io/gosub-engine> - browser engine in rust

## AI

- [burn](https://github.com/tracel-ai/burn) - Deep Learning Framework
- [candle](https://github.com/huggingface/candle) - Minimalist ML framework

## ZK

- [lambdaworks](https://github.com/lambdaclass/lambdaworks) - zk crypto and math

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
- [cargo-multivers](https://github.com/ronnychevalier/cargo-multivers) - build multiple versions of the same binary, each with a different CPU features set, merged into a single portable optimized binary.
- [cargo-unused-features](https://github.com/TimonPost/cargo-unused-features) - check unused features.
- [cargo-cache](https://github.com/matthiaskrgr/cargo-cache) - manage cargo cache
- [cargo-valgrind](https://github.com/jfrimmel/cargo-valgrind) - run valgrind with rust
- [cargo-dist](https://github.com/axodotdev/cargo-dist) - release your binary to github release
- [firedbg](https://firedbg.sea-ql.org/) - Time Travel Visual Debugger for Rust
- [rudric](https://github.com/mike-lloyd03/rudric) - .env secret encryption tool
- [rustviz](https://github.com/rustviz/rustviz) - visualize rust ownership and borrowing
- [hyperqueue](https://github.com/it4innovations/hyperqueue) - job scheduler
- [cargo-dylint](https://github.com/trailofbits/dylint) - dynamic lint, another clippy
- [zerocopy](https://github.com/google/zerocopy) - unsafe zero copy
- [presser](https://github.com/EmbarkStudios/presser) - unsafe copy structure data to raw buffer
- [release-plz](https://github.com/MarcoIeni/release-plz) - release github action
- [cargo-autoinherit](https://github.com/mainmatter/cargo-autoinherit) - auto manage workspace deps
- [BugStalker](https://github.com/godzie44/BugStalker/) - Rust debugger for linux
- [portal-tunneler](https://github.com/ThomasMiz/portal-tunneler) - hole punching
- [rlt](https://github.com/wfxr/rlt) - load testing tui
- [aquascope](https://github.com/cognitive-engineering-lab/aquascope) - generates interactive visualizations of Rust programs
- [watchexec](https://github.com/watchexec/watchexec) - Executes commands in response to file modifications
- [just](https://github.com/casey/just) - make alternative
- [systemfd](https://github.com/mitsuhiko/systemfd) - help with auto reloading for projects supporting systemd socket activation
- [creusot](https://github.com/creusot-rs/creusot) - a deductive verifier for Rust code
- [tokei](https://github.com/XAMPPRocky/tokei) - count your code, quickly
- [cargo-loc](https://github.com/Shnatsel/cargo-loc) - count lines of code across your entire dependency tree
- [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) - list unsafe rust code
- [mintaka](https://github.com/mwilliamson/mintaka) - Run long-running processes in parallel
- [genson-rs](https://github.com/junyu-w/genson-rs) - generate JSON schema
- [creusot](https://github.com/creusot-rs/creusot) - deductive verifier for Rust
- [get_blessed_rs](https://github.com/josueBarretogit/get_blessed_rs) - get best crates
- [BugStalker](https://github.com/godzie44/BugStalker) - rust debugger
- [gxhash](https://github.com/ogxd/gxhash) - fastest non-cryptographic hashing algorithm
- [cijail](https://github.com/staex-io/cijail) - prevent supply chain attacks
- [rsapar](https://github.com/dertin/rsapar) - struct/JSON schema validate
- [shpool](https://github.com/shell-pool/shpool) - tmux lightweight alternative
- [envelope](https://github.com/mattrighetti/envelope) - env variables manager
- [jlf](https://github.com/PoOnesNerfect/jlf) - json log viewer
- [flamelens](https://github.com/YS-L/flamelens) - terminal flamegraph viewer
- [sqruff](https://github.com/quarylabs/sqruff) - sql lint
- [trippy](https://github.com/fujiapple852/trippy) - traceroute + ping
- [tonbo](https://tonbo.io/) - Arrow and parquet embedded KV db.
- [rsql](https://github.com/theseus-rs/rsql) - db cli
- [Hawk](https://github.com/iyksh/Hawk) - network sniffer with GUI <https://iyksh.github.io/blog/sniffer/>
- [hurl](https://github.com/Orange-OpenSource/hurl) - http request with plain text
- [oryx](https://github.com/pythops/oryx) - tui for sniffer using eBPF
- [query.rs](https://query.rs/) - a search engine for rust
- [garage](https://github.com/deuxfleurs-org/garage) - S3-compatible object store
- [paradedb](https://github.com/paradedb/paradedb) - Elasticsearch alternative built on Postgres
- [BugStalker](https://github.com/godzie44/BugStalker) - rust debugger
- [rainfrog](https://github.com/achristmascarl/rainfrog) - postgres tui
- [arroyo](https://github.com/ArroyoSystems/arroyo) - Distributed stream processing engine
- [cargo-show-asm](https://github.com/pacak/cargo-show-asm) - cargo subcommand showing the assembly
- [refinery](https://github.com/rust-db/refinery) - db migration tool, can be used in Rust code or cli.
- [binsider](https://github.com/orhun/binsider) - Analyze ELF binaries
- [cargo-vet](https://github.com/mozilla/cargo-vet) - supply-chain security for Rust
- [diff.rs](https://diff.rs/) - diff between crate versions.
- [kaffe-rs](https://github.com/Schachte/kaffe-rs) - mdx to html
- [oha](https://github.com/hatoo/oha) - HTTP load generator, inspired by rakyll/hey with tui animation.
- [qcp](https://github.com/crazyscot/qcp) - like scp but with quick protocol
- [harper](https://github.com/elijah-potter/harper) - English grammer checker
- [gosub](https://github.com/gosub-io/gosub-engine) - rust browser engine
- [mise](https://github.com/jdx/mise) - dev tool
- [comtrya](https://github.com/comtrya/comtrya) - ansible alternative, much simpler
- [himalaya](https://github.com/pimalaya/himalaya) - email cli

## Rust FFI

- <https://neon-rs.dev/> - rust x node.js
- <https://trunkrs.dev/> - rust x wasm
- <https://github.com/lu-zero/cargo-c> - build and install C-compatible libraries

## Rust Apps

- <https://github.com/GyulyVGC/sniffnet>

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
- [Rust for Malware Development](https://github.com/Whitecat18/Rust-for-Malware-Development)
- [Rust in companies](https://github.com/ImplFerris/rust-in-production)
- <https://greptime.com/blogs/2024-05-07-error-rust> - error handling for big projects
- <https://github.com/tailcallhq/rust-benchmarks> - benchmark between pattern matching and dynamic dispatch
- <https://rust-dex.cc/#> - std lib explain and example code

## Nice videos

- [crust of rust](https://www.youtube.com/@jonhoo/videos)

## Nice Books

- [The Little Book of Rust Books](https://lborb.github.io/book/) - all rust books
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
- <https://rustwasm.github.io/book/introduction.html>
- <https://rust-lang.github.io/wg-async/>
- <https://rust-unofficial.github.io/patterns/>
- <https://github.com/LukeMathWalker/zero-to-production>
- <https://skanehira.github.io/writing-a-wasm-runtime-in-rust/>
- <https://toetoe55.github.io/async-rt-book/>
- <https://github.com/tugglecore/rust-tracing-primer>
- <https://rust-exercises.com/>
- <https://rust-lang.github.io/async-book/>
- <https://rust-lang.github.io/generic-associated-types-initiative/> - official GAT book
- <https://rust-for-rustaceans.com/>

## Rust design patterns

- <https://cglab.ca/~abeinges/blah/everyone-poops/>
- <https://smallcultfollowing.com/babysteps/blog/2018/04/16/rust-pattern-rooting-an-rc-handle/>
- <https://smallcultfollowing.com/babysteps/blog/2018/04/24/rust-pattern-precise-closure-capture-clauses/> is no longer needed. Closures now automatically capture only the fields from the struct that they use
- <https://smallcultfollowing.com/babysteps/blog/2018/09/02/rust-pattern-iterating-an-over-a-rc-vec-t/>
- <https://users.rust-lang.org/t/blog-post-series-rust-patterns/20080/10>
- <https://matklad.github.io/2023/11/15/push-ifs-up-and-fors-down.html>
- <https://matklad.github.io/2022/11/18/if-a-tree-falls-in-a-forest-does-it-overflow-the-stack.html>
- <https://ismailmaj.github.io/destructing-trees-safely-and-cheaply>
- <https://matklad.github.io/2022/05/29/builder-lite.html>
- <https://matklad.github.io/2022/06/11/caches-in-rust.html>
- <https://matklad.github.io/2021/07/09/inline-in-rust.html>
- <https://matklad.github.io/2018/06/04/newtype-index-pattern.html>
- <https://matklad.github.io/2018/05/24/typed-key-pattern.html>
- <https://matklad.github.io/2018/05/04/encapsulating-lifetime-of-the-field.html>
- <https://github.com/rust-unofficial/patterns/issues?q=is%3Aissue+is%3Aopen+label%3AA-pattern+label%3AC-addition>
- <https://symbolica.io/posts/control_flow_patterns/#block-breaks>
- <https://ryhl.io/blog/temporary-shared-mutation/>
- <https://symbolica.io/posts/control_flow_patterns/>
- <https://github.com/fadeevab/design-patterns-rust>
- <https://github.com/alexpusch/rust-magic-patterns>

## Rust powered projects

- <https://github.com/freedit-org/freedit> - rust forum
- <https://github.com/thewh1teagle/vibe> - Transcribe audio in every language
