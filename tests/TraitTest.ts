import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TraitTest } from "../target/types/trait_test";
const fs = require("fs");

describe("TraitTest", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TraitTest as Program<TraitTest>;

  // turn on all logs
  program.provider.connection.onLogs("all", ({ logs }) => {
    console.log(logs);
  });

  const user = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(
      fs
        .readFileSync("tEStzwUiictKKqgQvEMiCnguJ4PXVWYy6dHf9nombcJ.json", {
          encoding: "utf8",
          flag: "r",
        })
        .slice(1, -1)
        .split(",")
    )
  );

  it("Initialized old and new storage accounts!", async () => {
    let accounts1 = await program.methods.initialize1().pubkeys();
    let accounts2 = await program.methods.initialize2().pubkeys();
    console.log(
      accounts1.payer.toString(),
      accounts1.systemProgram.toString(),
      accounts1.storageAccount1.toString()
    );
    const tx1 = await program.methods
      .initialize1()
      .accounts(accounts1)
      .signers([user])
      .rpc();
    const tx2 = await program.methods
      .initialize2()
      .accounts(accounts2)
      .signers([user])
      .rpc();
  });

  it("Set and print name with same handler!", async () => {
    let accounts1 = await program.methods.name1("version 1").pubkeys();
    let accounts2 = await program.methods.name2("version 2").pubkeys();
    console.log(accounts1.storageAccount1.toString());
    try {
      const tx1 = await program.methods
        .name1("version 1")
        .accounts(accounts1)
        .rpc();
    } catch (err) {
      console.error(err);
    }
    const tx2 = await program.methods
      .name2("version 2")
      .accounts(accounts2)
      .rpc();
  });
});
