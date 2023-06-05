MCVE rust/issues/107358
=======================

```bash
$ uname -a
Linux archlinux 6.2.12-arch1-1 #1 SMP PREEMPT_DYNAMIC Thu, 20 Apr 2023 16:11:55 +0000 x86_64 GNU/Linux
$ cargo --version
cargo 1.70.0 (ec8a8a0ca 2023-04-25
```


```bash
$ RUSTFLAGS="-C link-args=-Wl,-zstack-size=104857600"  cargo +1.66.1-x86_64-unknown-linux-gnu test
    Finished test [unoptimized + debuginfo] target(s) in 2.34s
     Running unittests src/lib.rs (target/debug/deps/issue_107358-aa9a464a84458a7e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worker.rs (target/debug/deps/worker-024feb5faa555454)

running 1 test
test it_works_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.65s
```


```bash
$ RUSTFLAGS="-C link-args=-Wl,-zstack-size=104857600"  cargo +1.67-x86_64-unknown-linux-gnu test
    Finished test [unoptimized + debuginfo] target(s) in 9.54s
     Running unittests src/lib.rs (target/debug/deps/issue_107358-c5a815655ca58f89)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worker.rs (target/debug/deps/worker-4cde5aa7fa93ee3b)

running 1 test
test it_works_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.56s
```


```bash
$ cargo +1.69-x86_64-unknown-linux-gnu --version
cargo 1.69.0 (6e9a83356 2023-04-12)
$ RUSTFLAGS="-C link-args=-Wl,-zstack-size=104857600"  cargo +1.69-x86_64-unknown-linux-gnu test
    Finished test [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (target/debug/deps/issue_107358-81ba837bb3c047ed)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worker.rs (target/debug/deps/worker-8cce9f077cc3965e)

running 1 test

thread 'it_works_sync' has overflowed its stack
fatal runtime error: stack overflow
error: test failed, to rerun pass `--test worker`
```


```bash
$ cargo +stable --version
cargo 1.70.0 (ec8a8a0ca 2023-04-25)
$ RUSTFLAGS="-C link-args=-Wl,-zstack-size=104857600"  cargo +stable test
    Finished test [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/issue_107358-f84ad7734e1e5a2d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worker.rs (target/debug/deps/worker-dccb388c9739b30c)

running 1 test
test it_works_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.57s
```

```bash
$ cargo +nightly --version
cargo 1.72.0-nightly (b0fa79679 2023-06-03)
$ RUSTFLAGS="-C link-args=-Wl,-zstack-size=104857600"  cargo +nightly test
    Finished test [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/issue_107358-741c8da4883e6499)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worker.rs (target/debug/deps/worker-3b201a2304369ce2)

running 1 test
test it_works_sync ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.53s
```

