# Radix Stablecoin Vault

## Installation

The following steps assume working in a Linux environment.

See official installation guide for other operation systems:
https://docs-babylon.radixdlt.com/main/getting-started-developers/first-component/install-scrypto.html

Install build dependencies

```bash
sudo apt install clang build-essential llvm
```

### Install Rust

https://www.rust-lang.org/tools/install

If you already have Rust:

```bash
rustup update stable
```

OR install it the first time

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add WebAssembly target

```bash
rustup target add wasm32-unknown-unknown
```

Add Rust in your PATH:

```
export PATH="~/.cargo/bin:$PATH"
```

To configure your current shell and confirm Rust installation:

```bash
source $HOME/.cargo/env
rustc --version
```

### Install Scrypto

```bash
git clone https://github.com/radixdlt/radixdlt-scrypto.git
cd radixdlt-scrypto
cargo install --path ./simulator
resim -V
```

### Install VS Code Extensions

- Rust-Analyzer
- Radix Transaction Manifest Support

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension RadixPublishing.radix-transaction-manifest-support
```

### Install Repository Dependencies

```bash
scrypto build
```

This is only needed for the first time to install all Rust dependencies. For subsequent building, the above command can be omitted as running tests will automatically build everything again.

### Run Automatic Tests

Adding `-- --nocapture` will force logs to show even there is no error.

```bash
scrypto test
scrypto test -- --nocapture
```

### Run Manual Tests

1. Create a new account, and save the account address

```
resim new-account
```

2. Publish the package, and save the package address:

```
resim publish .
```

3. Call the `new` function to instantiate a component, and save the component address:

```
resim call-function <PACKAGE_ADDRESS> StableCoinVault new
```

4. Call the `mint_to_bucket` method of the component:

```
resim call-method <COMPONENT_ADDRESS> mint_to_bucket 100
```

5. Check out our balance

```
resim show <ACCOUNT_ADDRESS>
```
