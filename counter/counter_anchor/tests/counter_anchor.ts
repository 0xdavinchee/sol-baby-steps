import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { CounterAnchor } from "../target/types/counter_anchor";
import { expect } from "chai";

// this airdrops sol to an address
async function airdropSol(publicKey, amount) {
  let airdropTx = await anchor
    .getProvider()
    .connection.requestAirdrop(publicKey, amount);
  await confirmTransaction(airdropTx);
}

async function confirmTransaction(tx) {
  const latestBlockHash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
}

describe("counter_anchor", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterAnchor as Program<CounterAnchor>;

  const counter = Keypair.generate();
  const blapper = Keypair.generate();
  console.log("Counter keypair", counter.publicKey.toBase58());

  it("Is initialized!", async () => {
    await airdropSol(blapper.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);

    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        counterAccount: counter.publicKey,
        signer: blapper.publicKey,
      })
      .signers([counter, blapper])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counterAccount.fetch(
      counter.publicKey
    );
    expect(account.counter.toNumber()).to.equal(0);

    // Program signer is authority case
    // const tx = await program.methods
    //   .initialize()
    //   .accounts({
    //     counterAccount: counter.publicKey,
    //     signer: provider.wallet.publicKey,
    //   })
    //   .signers([counter])
    //   .rpc();
    // console.log("Your transaction signature", tx);

    // const account = await program.account.counterAccount.fetch(
    //   counter.publicKey
    // );
    // expect(account.counter.toNumber()).to.equal(0);
  });

  it("Increment", async () => {
    const tx = await program.methods
      .increment(new anchor.BN(69))
      .accounts({
        counterAccount: counter.publicKey,
        authority: blapper.publicKey,
      })
      .signers([blapper])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counterAccount.fetch(
      counter.publicKey
    );
    expect(account.counter.toNumber()).to.equal(69);
  });

  it("Decrement", async () => {
    const tx = await program.methods
      .decrement(new anchor.BN(69))
      .accounts({
        counterAccount: counter.publicKey,
        authority: blapper.publicKey,
      })
      .signers([blapper])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counterAccount.fetch(
      counter.publicKey
    );
    expect(account.counter.toNumber()).to.equal(0);
  });

  it("Update", async () => {
    const tx = await program.methods
      .update(new anchor.BN(420))
      .accounts({
        counterAccount: counter.publicKey,
        authority: blapper.publicKey,
      })
      .signers([blapper])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counterAccount.fetch(
      counter.publicKey
    );
    expect(account.counter.toNumber()).to.equal(420);
  });

  it("Reset", async () => {
    const tx = await program.methods
      .reset()
      .accounts({
        counterAccount: counter.publicKey,
        authority: blapper.publicKey,
      })
      .signers([blapper])
      .rpc();
    console.log("Your transaction signature", tx);

    const account = await program.account.counterAccount.fetch(
      counter.publicKey
    );
    expect(account.counter.toNumber()).to.equal(0);
  });
});
