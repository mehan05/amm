import * as anchor from "@coral-xyz/anchor";
import {
    Account,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
const provider = anchor.AnchorProvider.env();
export const airdrop = async (to: anchor.web3.PublicKey, amount: number) => {
  try {
    const tx = await provider.connection.requestAirdrop(
      to,
      amount * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx, "confirmed");
  } catch (error) {
    console.log("form airdrop", error);
  }
};

export const mint_account = async (payer: anchor.web3.Keypair) => {
  try {
    const mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      6
    );

    return mint;
  } catch (error) {
    console.log("from mint", error);
  }
};

export const create_pda = async (
  programId: anchor.web3.PublicKey,
  seeds: anchor.BN,
  maker: anchor.web3.Keypair
) => {
  try {
    const [config] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"), seeds.toBuffer()],
      programId
    );

    return config;
  } catch (error) {
    console.log("from ", error);
  }
};

export const create_associated_token_account = async (
  mint_acc: anchor.web3.PublicKey,
  user: anchor.web3.Keypair
) => {
  try {
    const vault = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint_acc,
      user.publicKey
    );

    console.log("vault", vault);

    return vault;
  } catch (error) {
    console.log(error);
  }
};

export const mint_tokens = async (
  payer: anchor.web3.Keypair,
  mint_acc: anchor.web3.PublicKey,
  token_account: Account,
  amount: number
) => {
  console.log("Minting Tokens..");
  try {
    const tx = await mintToChecked(
      provider.connection,
      payer,
      mint_acc,
      token_account.address,
      payer.publicKey,
      amount * anchor.web3.LAMPORTS_PER_SOL,
      6
    );

    console.log(
      "Tokens are minted to ",
      token_account.address,
      "\nHash:",
      tx.toString()
    );
  } catch (error) {
    console.log("Error in minting tokens", error);
  }
};
