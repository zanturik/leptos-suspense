ab testing reveal panic from tokio-runtime-worker while trying to get signal or some context. This is relatively lightweight example, more operations with context increase the failure rate.

Command:
``` ab -n 1000 -c 150 -l http://127.0.0.1:3038/ ```

Error examples:

``` thread 'tokio-runtime-worker' panicked at .cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_stores-0.1.0-rc2/src/store_field.rs:103:29:
At /home/hill/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_stores-0.1.0-rc2/src/store_field.rs:103:29, you tried to access a reactive value which was defined at /home/hill/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_stores-0.1.0-rc2/src/lib.rs:449:25, but it has already been disposed. ```

``` thread 'tokio-runtime-worker' panicked at .cargo/registry/src/index.crates.io-6f17d22bba15001f/reactive_graph-0.1.0-rc2/src/traits.rs:387:39:
At src/app.rs:97:33, you tried to access a reactive value which was defined at src/app.rs:34:17, but it has already been disposed. ```

