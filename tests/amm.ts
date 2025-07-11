import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { Keypair, PublicKey } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { BN } from "bn.js";

describe("amm", () => {

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.amm as Program<Amm>;
  let provider, wallet, seed, config, lptoken, minta, mintb, userx, usery, userlp, vaulta, vaultb;

  beforeEach(async () => {
    provider = anchor.getProvider();
    wallet = provider.wallet;
    
    seed = new anchor.BN(11);
    [config] = PublicKey.findProgramAddressSync([Buffer.from("config"), seed.toArrayLike(Buffer, "le", 8)], program.programId);
    [lptoken] = PublicKey.findProgramAddressSync([Buffer.from("lp"), config.toBuffer()], program.programId);
    
    minta = new PublicKey("735dt5HekUQQq3qi9NLttRegp6koRAA5nX6Uv3LHMXJS");
    mintb = new PublicKey("AuzCK8jdZQ9Dvud9DnFbZ8KuUeuhqZJ2BDoAwzGEmEWd");
    
    userx = getAssociatedTokenAddressSync(minta, wallet.payer.publicKey, false);
    usery = getAssociatedTokenAddressSync(mintb, wallet.payer.publicKey, false);
    userlp = getAssociatedTokenAddressSync(lptoken, wallet.payer.publicKey, false);
    
    vaulta = getAssociatedTokenAddressSync(minta, config, true, TOKEN_PROGRAM_ID);
    vaultb = getAssociatedTokenAddressSync(mintb, config, true, TOKEN_PROGRAM_ID);
  });

  it("Initialize pool", async () => {
    const fee = 2;
    
    const tx = await program.methods.initialize(seed, fee, wallet.payer.publicKey).accountsStrict({
      signer: wallet.payer.publicKey,
      mintx: minta,
      minty: mintb,
      lpToken: lptoken,
      vaultX: vaulta,
      vaultY: vaultb,
      config: config,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
    
    console.log("Initialize tx:", tx);
  });

  it("Deposit liquidity", async () => {
    const amount = new anchor.BN(2);
    const max_x = new anchor.BN(6);
    const max_y = new anchor.BN(6);
    
    const tx = await program.methods.deposit(amount, max_x, max_y).accountsStrict({
      signer: wallet.payer.publicKey,
      mintx: minta,
      minty: mintb,
      lpToken: lptoken,
      vaultX: vaulta,
      vaultY: vaultb,
      userX: userx,
      userY: usery,
      userLp: userlp,
      config: config,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
    
    console.log("Deposit tx:", tx);
  });

  it("Swap tokens", async () => {
    const amount = new anchor.BN(2);
    const min_receive = new anchor.BN(1);
    
    const tx = await program.methods.swap(amount, true, min_receive).accountsStrict({
      signer: wallet.payer.publicKey,
      mintx: minta,
      minty: mintb,
      lpToken: lptoken,
      vaultX: vaulta,
      vaultY: vaultb,
      userX: userx,
      userY: usery,
      userLp: userlp,
      config: config,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
    
    console.log("Swap tx:", tx);
  });

  it("Withdraw liquidity", async () => {
    const amount = new anchor.BN(2);
    const min_x = new anchor.BN(1);
    const min_y = new anchor.BN(1);
    
    const tx = await program.methods.withdraw(amount, min_x, min_y).accountsStrict({
      signer: wallet.payer.publicKey,
      mintx: minta,
      minty: mintb,
      lpToken: lptoken,
      vaultX: vaulta,
      vaultY: vaultb,
      userX: userx,
      userY: usery,
      userLp: userlp,
      config: config,
      systemProgram: SYSTEM_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();
    
    console.log("Withdraw tx:", tx);
  });
});
