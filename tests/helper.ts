import * as anchor from "@coral-xyz/anchor";
import {
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

export const mint_account = async (payer: anchor.web3.Keypair,owner: anchor.web3.PublicKey):Promise<anchor.web3.PublicKey> => {
  try {
    const mint = await createMint(
      provider.connection,
      payer,
      owner,
      payer.publicKey,
      6
    );
    // console.log("mint created",mint);
    return mint;
  } catch (error) {
    console.log("from mint", error);
  }
};



export const create_associated_token_account = async (
  mint_acc: anchor.web3.PublicKey,
  user: anchor.web3.Keypair,
  owner: anchor.web3.PublicKey
) => {
  try {
    const vault = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user,
      mint_acc,
      owner,
      true
    );


    return vault;
  } catch (error) {
    console.log(error);
  }
};

export const mint_tokens = async (
  payer: anchor.web3.Keypair,
  mint_acc: anchor.web3.PublicKey,
  token_account: any,
  amount: number
) => {

  try {
    const tx = await mintToChecked(
      provider.connection,
      payer,
      mint_acc,
      token_account,
      payer.publicKey,
      amount * anchor.web3.LAMPORTS_PER_SOL,
      6
    );


  } catch (error) {
    console.log("Error in minting tokens", error);
  }
};
