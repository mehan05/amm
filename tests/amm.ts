import { Program } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import {
  Account,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  airdrop,
  create_associated_token_account,
  create_pda,
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
let vault_x:Account
let vault_y:Account
let user_x: Account;
let user_y: Account;
let user_lp: Account;
describe("amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const seed = 7;
  const fee = 300;
  const program = anchor.workspace.amm as Program<Amm>;

  before(async () => {
    initializer = anchor.web3.Keypair.generate();
    user_x = await create_associated_token_account(mint_x, initializer);
    user_y = await create_associated_token_account(mint_y, initializer);
    await airdrop(initializer.publicKey, 1000);
    await mint_tokens(initializer, mint_x, user_x, 1000);
    await mint_tokens(initializer, mint_y, user_y, 1000);
    mint_x = await mint_account(initializer);
    mint_y = await mint_account(initializer);
    lp_mint = await mint_account(initializer);
    config = await create_pda(
      program.programId,
      new anchor.BN(seed),
      initializer
    );
    vault_x = await create_associated_token_account(mint_x, initializer);
    vault_y = await create_associated_token_account(mint_y, initializer);
    user_lp = await create_associated_token_account(
      lp_mint,
      initializer
    );

    
  });

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(seed, initializer.publicKey, fee)
      .accountsStrict({
        initializer: initializer.publicKey,
        mintX: mint_x,
        mintY: mint_y,
        config,
        lpMint: lp_mint,
        vaultX: vault_x.address,
        vaultY: vault_y.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([initializer])
      .rpc();
    console.log("Your transaction signature", tx);

    const ammPda = await program.account.config.fetch(config);

    assert.notEqual(ammPda, null);
  });

  it("deposit", async () => {
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
        vaultX: vault_x.address,
        vaultY: vault_y.address,
        userX: user_x.address,
        userY: user_y.address,
        userLp: user_lp.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([initializer])
      .rpc();

    console.log("deposit transaction signature", tx);

    const vault_x_account = await getAccount(provider.connection, vault_x.address);

    const vaultXBalance = vault_x_account.amount;
    const vaultYBalance = vault_x_account.amount;

    expect(vaultXBalance).to.equal(max_amount_x);
    expect(vaultYBalance).to.equal(max_amount_y);

    console.log("vaultXBalance", vaultXBalance);
    console.log("vaultYBalance", vaultYBalance);
  });

  it("withdraw", async () => {
    const amount = new anchor.BN(500);
    const min_amount_x = new anchor.BN(400);
    const min_amount_y = new anchor.BN(400);

    const vault_x_info = await getAccount(provider.connection, vault_x.address);
    const vault_y_info = await getAccount(provider.connection, vault_y.address);

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
        vaultX: vault_x.address,
        vaultY: vault_y.address,
        userX: user_x.address,
        userY: user_y.address,
        userLp: user_lp.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([initializer])
      .rpc();

    console.log("withdraw transaction signature", tx);

    const final_x_balance = vault_x_info.amount;
    const final_y_balance = vault_y_info.amount;

    expect(final_x_balance).to.not.equal(initial_x_balance);
    expect(final_y_balance).to.not.equal(initial_y_balance);
  });

  it("swap", async () => {
    let is_x = true;
    let min_y = new anchor.BN(250);
    let amount = new anchor.BN(300);

    const vault_x_info = await getAccount(provider.connection, vault_x.address);
    const vault_y_info = await getAccount(provider.connection, vault_y.address);

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

    console.log("swap transaction signature", tx);

    let final_x_balance = vault_x_info.amount;
    let final_y_balance = vault_y_info.amount;

    expect(Number(initial_x_balance) - Number(amount)).to.equal(
      final_x_balance
    );
    expect(Number(initial_y_balance) + Number(min_y)).to.equal(final_y_balance);
  });
});
