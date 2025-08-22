# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.

Perfecto 🚀 Te preparo un **README.md** que puedes usar directamente en tu repo de GitHub para el workshop. Está escrito como guía paso a paso para los participantes.

---

# 🚀 Workshop: Deploying Your First Stellar Smart Contract (Soroban)

Welcome to this hands-on workshop where you’ll learn how to **write, deploy, and interact** with a smart contract on the **Stellar Testnet** using **Soroban**.

---

## 🎯 Objectives

By the end of this workshop you will:

* Understand the basics of Stellar & Soroban smart contracts.
* Set up your local development environment.
* Write a simple smart contract in Rust.
* Deploy it to the Stellar testnet.
* Interact with your contract via the Soroban CLI.

---

## 🛠 Prerequisites

Make sure you have the following installed:

* [Rust](https://www.rust-lang.org/tools/install)
* Soroban CLI:

  ```bash
  cargo install --locked soroban-cli
  ```
* A testnet account (funded with [Friendbot](https://laboratory.stellar.org/#account-creator))

👉 If you can’t install locally, you can also use a **Docker container or GitHub Codespaces** (instructor will provide).

---

## 📂 Project Setup

1. **Create a new contract project**:

   ```bash
   cargo new fridays_demo --lib
   cd fridays_demo
   ```

2. **Update your `Cargo.toml`**:

   ```toml
   [dependencies]
   soroban-sdk = "21.0.0-rc1" # adjust to latest version
   ```

3. **Write the contract** (`src/lib.rs`):

   ```rust
   #![no_std]
   use soroban_sdk::{contract, contractimpl, Env, Symbol};

   #[contract]
   pub struct CounterContract;

   #[contractimpl]
   impl CounterContract {
       pub fn increment(env: Env) -> u32 {
           let key = Symbol::short("count");
           let mut count: u32 = env.storage().persistent().get(&key).unwrap_or(0);
           count += 1;
           env.storage().persistent().set(&key, &count);
           count
       }

       pub fn get(env: Env) -> u32 {
           let key = Symbol::short("count");
           env.storage().persistent().get(&key).unwrap_or(0)
       }
   }
   ```

4. **Build the contract**:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

   The compiled WASM will be inside:

   ```
   target/wasm32-unknown-unknown/release/fridays_demo.wasm
   ```

---

## 🚀 Deploy the Contract

1. **Generate a keypair**:

   ```bash
   soroban keys generate --network testnet demo
   ```

2. **Fund your account** using Friendbot:
   Open [Friendbot](https://laboratory.stellar.org/#account-creator) and paste your public key.

3. **Deploy the contract**:

   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/fridays_demo.wasm \
     --source demo \
     --network testnet
   ```

   ➝ This will output a **contract ID** (save it!).

---

## 🔄 Interact with the Contract

### Increment the counter:

```bash
soroban contract invoke \
  --id <contract_id> \
  --fn increment \
  --source demo \
  --network testnet
```

### Get the counter value:

```bash
soroban contract invoke \
  --id <contract_id> \
  --fn get \
  --network testnet
```

You should see the counter increase each time you run `increment`.

---

## ✅ Next Steps

* Extend your contract with more functions.
* Build a token or NFT contract.
* Connect it to a frontend app (React / Flutter).
* Explore [Soroban Docs](https://soroban.stellar.org/docs).

---

## 🙌 Credits

This workshop was designed to give you a **smooth first experience** with Soroban and Stellar smart contracts.

---

Would you like me to also **add some ready-made funded testnet accounts + .env example file** in the README so participants don’t get stuck waiting on Friendbot?
