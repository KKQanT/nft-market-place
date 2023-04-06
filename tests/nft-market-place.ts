import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMarketPlace } from "../target/types/nft_market_place";

describe("nft-market-place", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftMarketPlace as Program<NftMarketPlace>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
