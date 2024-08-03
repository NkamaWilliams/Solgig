import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Commitment,
} from "@solana/web3.js";
import { Solgig } from "../target/types/solgig";
import { BN } from "bn.js";
import { assert } from "chai";

const makerSeed = [
  168, 194, 231, 133, 141, 204, 230, 100, 1, 200, 116, 223, 99, 202, 63, 78, 73,
  194, 213, 129, 233, 144, 207, 10, 216, 80, 113, 24, 193, 191, 191, 32, 41, 69,
  23, 203, 82, 216, 218, 57, 196, 211, 81, 100, 68, 238, 150, 117, 152, 196,
  215, 253, 26, 71, 2, 183, 169, 56, 60, 224, 207, 84, 198, 226,
];
const devSeed = [
  243, 134, 232, 182, 38, 20, 221, 84, 184, 243, 148, 63, 174, 105, 39, 42, 60,
  164, 154, 188, 238, 42, 142, 55, 204, 80, 197, 171, 93, 205, 219, 35, 168, 72,
  163, 61, 183, 44, 236, 227, 199, 179, 145, 171, 70, 76, 237, 85, 10, 59, 99,
  187, 157, 143, 17, 4, 64, 119, 14, 33, 111, 249, 24, 237,
];

describe("solgig", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.Solgig as Program<Solgig>;
  const seed = 1;

  //Log
  const log = async (signature: string) => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  // Accounts
  const maker = Keypair.fromSecretKey(new Uint8Array(makerSeed));
  const developer = Keypair.fromSecretKey(new Uint8Array(devSeed));
  const jobState = PublicKey.findProgramAddressSync(
    [
      Buffer.from("job"),
      maker.publicKey.toBuffer(),
      new BN(seed).toBuffer("le", 8),
    ],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), jobState.toBuffer()],
    program.programId
  )[0];
  const systemProgram = SystemProgram.programId;

  it("Initialize", async () => {
    let tx = await program.methods
      .initialize(new BN(seed), 3)
      .accounts({
        vault,
        maker: maker.publicKey,
        jobState,
        systemProgram,
      })
      .signers([maker])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
  });
  console.log();

  it("Deposit 0.1 SOL in Vault", async () => {
    let tx = await program.methods
      .deposit(new BN(seed), new BN(0.1 * LAMPORTS_PER_SOL))
      .accounts({
        vault,
        maker: maker.publicKey,
        jobState,
        systemProgram,
      })
      .signers([maker])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
    console.log(await connection.getBalance(vault));
  });
  console.log();

  it("Assign Developer", async () => {
    let tx = await program.methods
      .assign(new BN(seed))
      .accounts({
        maker: maker.publicKey,
        developer: developer.publicKey,
        jobState,
        systemProgram,
      })
      .signers([maker])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
    console.log(`Developer: ${developer.publicKey}`);
  });
  console.log();

  it("Task 1 Submitted", async () => {
    let tx = await program.methods
      .submit(new BN(seed))
      .accounts({
        developer: developer.publicKey,
        maker: maker.publicKey,
        jobState,
        systemProgram,
      })
      .signers([developer])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
  });
  console.log();

  it("Task 1 Accepted", async () => {
    let tx = await program.methods
      .acceptSubmission(new BN(seed))
      .accounts({
        maker: maker.publicKey,
        jobState,
        systemProgram,
      })
      .signers([maker])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
  });
  console.log();

  it("Task 2 Submitted", async () => {
    let tx = await program.methods
      .submit(new BN(seed))
      .accounts({
        developer: developer.publicKey,
        maker: maker.publicKey,
        jobState,
        systemProgram,
      })
      .signers([developer])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
  });
  console.log();

  it("Task 2 Rejected", async () => {
    let tx = await program.methods
      .rejectSubmission(new BN(seed))
      .accounts({
        maker: maker.publicKey,
        jobState,
        systemProgram,
      })
      .signers([maker])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
  });
  console.log();

  it("Developer Withdraw", async () => {
    let tx = await program.methods
      .withdraw(new BN(seed))
      .accounts({
        maker: maker.publicKey,
        jobState,
        systemProgram,
        developer: developer.publicKey,
        vault,
      })
      .signers([developer])
      .rpc();

    console.log(`Your transaction signature: ${tx}`);
    console.log(await program.account.job.fetch(jobState));
    console.log(`Vault balance: ${await connection.getBalance(vault)}`);
    console.log(
      `Developer balance: ${await connection.getBalance(developer.publicKey)}`
    );
  });
  console.log();

  it("Job Cancelled", async () => {
    const tx = await program.methods
      .cancel(new BN(seed))
      .accounts({ maker: maker.publicKey, jobState, vault, systemProgram })
      .signers([maker])
      .rpc();
    console.log(`Your transaction signature: ${tx}`);
    console.log(
      `Maker balance: ${await connection.getBalance(maker.publicKey)}`
    );
  });
  console.log();
});
