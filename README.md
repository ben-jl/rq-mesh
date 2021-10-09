# RQ-Mesh

## Set-up

1. Install Rust if necessary

    ```bash
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

2. Update Rust

    ```bash
    rustup update
    ```

3. Install gcc if necessary

    ```bash
    apk add gcc # Alpine Linux
    ```

4. Install sqlite3 if necessary

    ```bash
    apk add sqlite # Alpine Linux
    ```

5. Install nightly toolchain

    ```bash
    rustup install nightly
    ```

6. Install libc libraries

    ```bash
    apk add libc-dev=0.7.1-r0 # Alpine Linux
    ```

7. Export necessary files for linker to path

    ```bash
    PATH=$PATH:/root/.rustup/toolchains/nightly-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained

    export PATH
    ```

8. Install cargo-watch

    ```bash
    cargo install cargo-watch
    ```
