<div align="center">
  <!-- <img height="170x" src="https://pbs.twimg.com/media/FVUVaO9XEAAulvK?format=png&name=small" /> -->

  <h1>Kaptn</h1>

  <p>
    <strong>Transfer-Hook Program Framework</strong>
  </p>
</div>

Kaptn is a framework for Solana's [Sealevel](https://medium.com/solana-labs/sealevel-parallel-processing-thousands-of-smart-contracts-d814b378192) runtime that makes creating [Transfer-Hook](https://solana.com/developers/guides/token-extensions/transfer-hook) programs even easier.

- Convenience macros for abstracting the complexity of native Rust Solana programs
- Similar flow to writing [Anchor](https://github.com/coral-xyz/anchor) programs
- CLI tool for deploying, managing, and monitoring Transfer-Hook programs (TBA)

This framework is in early development and any contributions or suggestions are very welcome.

**NOTE: At this time, there have been no audits on the code. Use at your own risk.**

## Getting Started

### Prerequisites

- Rust 1.70+
- Solana CLI

### Installation

To install the Kaptn CLI, run the following command:

```bash
cargo install kaptn-cli
```

### Creating a new project

To create a new project, run the following command:

```bash
kaptn new my-project
```

This will create a new directory called `my-project` with a basic structure for a Kaptn project.

This will produce a lib.rs file that looks like this:

```rust
use kaptn_lang::prelude::*;

declare_id!("FVcFz6auGqMvAAcGyYf4RDUq9P3gAPvKCUsYfRSHFGup");
declare_mint!("Eux51z9kgh6ViFkiiCuL8nzrc8pjL8U82PGKm8Rxic6m");

#[transfer_hook]
pub fn my_project(ctx: TransferContext<MyExtraMetas>) -> ProgramResult {
    msg!("Transfer hook called!");
    Ok(())
}

#[derive(ExtraMetas)]
pub struct MyExtraMetas {}

```
Here we can see a few things:

- `declare_id!` and `declare_mint!` are macros that generate the ID and mint for your program. These refer to keypairs that were generated when the project was created.
- `#[transfer_hook]` is a macro that generates the transfer hook for your program. This is the main function that will be called when a transfer hook is executed. Inside this function you are provided a `TransferContext` which contains the necessary information to process the transfer and any other bussiness logic you want to add.
- `#[derive(ExtraMetas)]` is a macro that generates the extra metas for your program. This is where you can add your own extra metas if needed. These can be static pubkeys or generated seeds or even other extra metas from your struct.

### Building and deploying

To build and deploy your project, run the following command:

```bash
cargo build-bpf
solana program deploy target/deploy/my_project.so
```

This will build your project and deploy it to the Solana network.

### Initializing and Updating ExtraMetas

When you first deploy your program you will need to initialize and fill the PDA that will be use to update your extra metas. You can do this by running the following command:

```bash
kaptn create-extra-metas
```

When ever your change your extra metas struct in the program, you will also need to build, deploy your program, and update your on chain extra metas account. This is done by running the following command:

```bash
kaptn update-extra-metas
```
With this you should be all set up and ready to start using your Transfer-Hook program. I plan on wrapping the rest of the command such and building and deploying into the kaptn cli. 
