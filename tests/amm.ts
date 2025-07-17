import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  airdrop,
  create_associated_token_account,
  mint_account,
  mint_tokens,
} from "./helper";
import { assert, expect } from "chai";

const provider = anchor.AnchorProvider.env();

let initializer: anchor.web3.Keypair;
let mint_x: anchor.web3.PublicKey;
let mint_y: anchor.web3.PublicKey;
let config: anchor.web3.PublicKey;
let lp_mint: anchor.web3.PublicKey;
let vault_x: any;
let vault_y: any;
let user_x: any;
let user_y: any;
let user_lp: any;
describe("amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const seed = 7;
  const fee = 300;
  const program = anchor.workspace.amm as Program<Amm>;

  before("setting up accounts", async () => {
    initializer = anchor.web3.Keypair.generate();
    await airdrop(initializer.publicKey, 1000);

    mint_x = await mint_account(initializer, initializer.publicKey);
    mint_y = await mint_account(initializer, initializer.publicKey);
    config = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"), new anchor.BN(seed).toBuffer()],
      program.programId
    )[0];

    lp_mint = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_mint"), config.toBuffer()],
      program.programId
    )[0];

    console.log("config", config);
    console.log("lp_mint", lp_mint);

    user_x = (
      await create_associated_token_account(
        mint_x,
        initializer,
        initializer.publicKey
      )
    ).address;
    user_y = (
      await create_associated_token_account(
        mint_y,
        initializer,
        initializer.publicKey
      )
    ).address;

    vault_x = await getAssociatedTokenAddress(mint_x, config, true);
    vault_y = await getAssociatedTokenAddress(mint_y, config, true);

    await mint_tokens(initializer, mint_x, user_x, 1000);
    await mint_tokens(initializer, mint_y, user_y, 1000);

    // console.log("initializer", initializer);
    // console.log("mint_x", mint_x);
    // console.log("mint_y", mint_y);
    // console.log("config", config);
    // console.log("lp_mint", lp_mint);
    // console.log("vault_x", vault_x);
    // console.log("vault_y", vault_y);
    // console.log("user_x", user_x);
    // console.log("user_y", user_y);
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(seed, initializer.publicKey, fee)
      .accountsStrict({
        initializer: initializer.publicKey,
        mintX: mint_x,
        mintY: mint_y,
        config: config,
        lpMint: lp_mint,
        vaultX: vault_x,
        vaultY: vault_y,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([initializer])
      .rpc();
    user_lp = await getAssociatedTokenAddress(lp_mint, initializer.publicKey);

    const ammPda = await program.account.config.fetch(config);

    assert.notEqual(ammPda, null);
  });

  it("deposit", async () => {
    try {
      const amount = new anchor.BN(500);
      const max_amount_x = new anchor.BN(600);
      const max_amount_y = new anchor.BN(600);

      const tx = await program.methods
        .deposit(amount, max_amount_x, max_amount_y)
        .accountsStrict({
          initializer: initializer.publicKey,
          mintX: mint_x,
          mintY: mint_y,
          config,
          lpMint: lp_mint,
          vaultX: vault_x,
          vaultY: vault_y,
          userX: user_x,
          userY: user_y,
          userLp: user_lp,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([initializer])
        .rpc();

      const vault_x_account = await getAccount(
        provider.connection,
        vault_x.address
      );

      const vaultXBalance = vault_x_account.amount;
      const vaultYBalance = vault_x_account.amount;

      expect(vaultXBalance).to.equal(max_amount_x);
      expect(vaultYBalance).to.equal(max_amount_y);

      console.log("vaultXBalance", vaultXBalance);
      console.log("vaultYBalance", vaultYBalance);
    } catch (error) {
      console.log(error);
    }
  });

  it("withdraw", async () => {
    try {
      const amount = new anchor.BN(500);
      const min_amount_x = new anchor.BN(400);
      const min_amount_y = new anchor.BN(400);

      const vault_x_info = await getAccount(
        provider.connection,
        vault_x.address
      );
      const vault_y_info = await getAccount(
        provider.connection,
        vault_y.address
      );

      const initial_x_balance = vault_x_info.amount;
      const initial_y_balance = vault_y_info.amount;
      const tx = await program.methods
        .withdraw(amount, min_amount_x, min_amount_y)
        .accountsStrict({
          initializer: initializer.publicKey,
          mintX: mint_x,
          mintY: mint_y,
          config,
          lpMint: lp_mint,
          vaultX: vault_x,
          vaultY: vault_y,
          userX: user_x,
          userY: user_y,
          userLp: user_lp,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([initializer])
        .rpc();

      const final_x_balance = vault_x_info.amount;
      const final_y_balance = vault_y_info.amount;

      expect(final_x_balance).to.not.equal(initial_x_balance);
      expect(final_y_balance).to.not.equal(initial_y_balance);
    } catch (error) {
      console.log(error);
    }
  });

  it("swap", async () => {
    try {
      let is_x = true;
      let min_y = new anchor.BN(250);
      let amount = new anchor.BN(300);

      const vault_x_info = await getAccount(
        provider.connection,
        vault_x.address
      );
      const vault_y_info = await getAccount(
        provider.connection,
        vault_y.address
      );

      let initial_x_balance = vault_x_info.amount;
      let initial_y_balance = vault_y_info.amount;

      let tx = await program.methods
        .swap(amount, is_x, min_y)
        .accountsStrict({
          initializer: initializer.publicKey,
          mintX: mint_x,
          mintY: mint_y,
          config,
          lpMint: lp_mint,
          vaultX: vault_x.address,
          vaultY: vault_y.address,
          userX: user_x.address,
          userY: user_y.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([initializer])
        .rpc();

      let final_x_balance = vault_x_info.amount;
      let final_y_balance = vault_y_info.amount;

      expect(Number(initial_x_balance) - Number(amount)).to.equal(
        final_x_balance
      );
      expect(Number(initial_y_balance) + Number(min_y)).to.equal(
        final_y_balance
      );
    } catch (error) {
      console.log(error);
    }
  });
});
