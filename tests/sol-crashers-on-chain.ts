import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolCrashersOnChain } from "../target/types/sol_crashers_on_chain";
import { getAccount, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { assert } from "chai";

describe("sol-crashers-on-chain", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  provider.opts.commitment = "confirmed";
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCrashersOnChain as Program<SolCrashersOnChain>;
  const developerKP = (program.provider as anchor.AnchorProvider).wallet; //me, my filesys wallet.
  
  const [mintGoldPK] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), Buffer.from("gold")],
    program.programId
  );
  const [mintGemsPK] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), Buffer.from("gems")],
    program.programId
  );

  const [developer_gemsATA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("gems"), developerKP.publicKey.toBuffer()],
    program.programId
  );
  
  const [developer_goldATA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("gold"), developerKP.publicKey.toBuffer()],
    program.programId
  );



  it("Is initialized!", async () => {

    const tx = await program.methods
    .initialize()
    .accounts({
      mintGold: mintGoldPK,
      mintGems: mintGemsPK,
      payer: developerKP.publicKey,
    })
    .rpc();

    console.log("Transaction signature: %s", tx);
    console.log("Program ID:\t%s", program.programId.toBase58());
    console.log("Payer:\t\t%s", developerKP.publicKey);
    console.log("Mint Gold PK:\t%s", mintGoldPK.toBase58());
    console.log("Mint Gems PK:\t%s", mintGemsPK.toBase58());
    console.log("ATA Gold PK:\t%s", developer_goldATA.toBase58());
    console.log("ATA Gems PK:\t%s", developer_gemsATA.toBase58());
  });

  it("Minted gold", async () => {
    const tx = await program.methods
    .printGold(new anchor.BN(299))
    .accounts({
      mint: mintGoldPK,
      payer: developerKP.publicKey,
      dstAta: developer_goldATA,
    })
    .rpc();

    const new_amount = Number((await getAccount(program.provider.connection, developer_goldATA)).amount);
    assert.isAbove(new_amount, 0, "Amount should be greater than 0");
  });

  it("Minted two gems", async () => {
    const tx = await program.methods
    .printGems(new anchor.BN(2))
    .accounts({
      mint: mintGemsPK,
      payer: developerKP.publicKey,
      dstAta: developer_gemsATA,
    })
    .rpc().catch((err) => {
      console.log(err);
    }
    );

    const new_amount = Number((await getAccount(program.provider.connection, developer_gemsATA)).amount);
    assert.equal(new_amount, 2, "Amount should be greater than 0");
  });

  it("Minted one more gem", async () => {
    const tx = await program.methods
    .printGems(new anchor.BN(1))
    .accounts({
      mint: mintGemsPK,
      payer: developerKP.publicKey,
      dstAta: developer_gemsATA,
    })
    .rpc().catch((err) => {
      console.log(err);
    }
    );

    const new_amount = Number((await getAccount(program.provider.connection, developer_gemsATA)).amount);
    assert.equal(new_amount, 3, "Amount should be greater than 0");
  });

  it("Burned gold", async () => {
    const tx = await program.methods
    .burnGold(new anchor.BN(1))
    .accounts({
      mint: mintGoldPK,
      payer: developerKP.publicKey,
      dstAta: developer_goldATA,
    })
    .rpc().catch((err) => {
      console.log(err);
    }
    );

    const new_amount = Number((await getAccount(program.provider.connection, developer_goldATA)).amount);
    assert.equal(new_amount, 298, "Amount should be greater than 0");
  });

  it("Burned gems", async () => {
    const tx = await program.methods
    .burnGems(new anchor.BN(2))
    .accounts({
      mint: mintGemsPK,
      payer: developerKP.publicKey,
      dstAta: developer_gemsATA,
    })
    .rpc().catch((err) => {
      console.log(err);
    }
    );

    const new_amount = Number((await getAccount(program.provider.connection, developer_gemsATA)).amount);
    assert.equal(new_amount, 1, "Amount should be greater than 0");
  });
});
