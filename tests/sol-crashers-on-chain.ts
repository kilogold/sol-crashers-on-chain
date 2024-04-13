import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolCrashersOnChain } from "../target/types/sol_crashers_on_chain";
import { getAccount, getAssociatedTokenAddressSync } from "@solana/spl-token"
import { assert } from "chai";

describe("sol-crashers-on-chain", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  provider.opts.commitment = "confirmed";
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCrashersOnChain as Program<SolCrashersOnChain>;
  const developerKP = (program.provider as anchor.AnchorProvider).wallet; //me, my filesys wallet.
  
  const [mintPK] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  const developer_tokenAccount = getAssociatedTokenAddressSync(
    mintPK,
    developerKP.publicKey
  );

  it("Is initialized!", async () => {

    const tx = await program.methods
    .initialize()
    .accounts({
      mint: mintPK,
      payer: developerKP.publicKey,
    })
    .rpc();

    console.log("Transaction signature: %s", tx);
    console.log("Program ID:\t%s", program.programId.toBase58());
    console.log("Mint PK:\t%s", mintPK.toBase58());
    console.log("ATA PK:\t\t%s", developer_tokenAccount.toBase58());
  });

  it("Minted gold", async () => {
    const tx = await program.methods
    .printCurrency(new anchor.BN(300), {gold: {}})
    .accounts({
      mint: mintPK,
      payer: developerKP.publicKey,
      dstAta: developer_tokenAccount,
    })
    .rpc();

    const new_amount = Number((await getAccount(program.provider.connection, developer_tokenAccount)).amount);
    assert.isAbove(new_amount, 0, "Amount should be greater than 0");

  });
});
