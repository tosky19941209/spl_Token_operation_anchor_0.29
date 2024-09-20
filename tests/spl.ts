import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Spl } from "../target/types/spl";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

describe("spl", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.Spl as Program<Spl>;

  const mintToken = anchor.web3.Keypair.generate()
  const associateTokenProgram = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
  const tokenAccount = anchor.utils.token.associatedAddress({
    mint: mintToken.publicKey,
    owner: provider.publicKey
  })

  const tx = anchor.web3.PublicKey.findProgramAddressSync(
    [
      provider.publicKey.toBuffer(),
      TOKEN_PROGRAM_ID.toBuffer(),
      mintToken.publicKey.toBuffer(),
    ],
    associateTokenProgram
  )[0]

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create Token", async () => {
    await program.methods.createToken(9, new anchor.BN(10 ** 9 * 100))
      .accounts({
        mintToken: mintToken.publicKey,
        tokenAccount: tokenAccount,
        associateTokenProgram: associateTokenProgram
      })
  })
});


//werwersdf