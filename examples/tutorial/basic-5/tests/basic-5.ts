import * as mainstay from "@nx-pkg/mainstay";
import { Basic5 } from "../target/types/basic_5";

describe("basic-5", () => {
  const provider = mainstay.MainstayProvider.local();

  // Configure the client to use the local cluster.
  mainstay.setProvider(provider);

  const program = mainstay.workspace.Basic5 as mainstay.Program<Basic5>;
  const user = provider.wallet.publicKey;

  let [actionState] = mainstay.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("action-state"), user.toBuffer()],
    program.programId
  );

  it("basic-5: Robot actions!", async () => {
    // Create instruction: set up the Solana accounts to be used
    const createInstruction = await program.methods
      .create()
      .accounts({
        actionState,
        user,
        systemProgram: mainstay.web3.SystemProgram.programId,
      })
      .instruction();
    // Walk instruction: Invoke the Robot to walk
    const walkInstruction = await program.methods
      .walk()
      .accounts({
        actionState,
        user,
      })
      .instruction();
    // Run instruction: Invoke the Robot to run
    const runInstruction = await program.methods
      .run()
      .accounts({
        actionState,
        user,
      })
      .instruction();
    // Jump instruction: Invoke the Robot to jump
    const jumpInstruction = await program.methods
      .jump()
      .accounts({
        actionState,
        user,
      })
      .instruction();
    // Reset instruction: Reset actions of the Robot
    const resetInstruction = await program.methods
      .reset()
      .accounts({
        actionState,
        user,
      })
      .instruction();

    // Array of instructions
    const instructions: mainstay.web3.TransactionInstruction[] = [
      createInstruction,
      walkInstruction,
      runInstruction,
      jumpInstruction,
      resetInstruction,
    ];

    await createAndSendV0Tx(instructions);
  });

  async function createAndSendV0Tx(
    txInstructions: mainstay.web3.TransactionInstruction[]
  ) {
    // Step 1 - Fetch the latest blockhash
    let latestBlockhash = await provider.connection.getLatestBlockhash(
      "confirmed"
    );
    console.log(
      "   ✅ - Fetched latest blockhash. Last Valid Height:",
      latestBlockhash.lastValidBlockHeight
    );

    // Step 2 - Generate Transaction Message
    const messageV0 = new mainstay.web3.TransactionMessage({
      payerKey: user,
      recentBlockhash: latestBlockhash.blockhash,
      instructions: txInstructions,
    }).compileToV0Message();
    console.log("   ✅ - Compiled Transaction Message");
    const transaction = new mainstay.web3.VersionedTransaction(messageV0);

    // Step 3 - Sign your transaction with the required `Signers`
    provider.wallet.signTransaction(transaction);
    console.log("   ✅ - Transaction Signed");

    // Step 4 - Send our v0 transaction to the cluster
    const txid = await provider.connection.sendTransaction(transaction, {
      maxRetries: 5,
    });
    console.log("   ✅ - Transaction sent to network");

    // Step 5 - Confirm Transaction
    const confirmation = await provider.connection.confirmTransaction({
      signature: txid,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    if (confirmation.value.err) {
      throw new Error(
        `   ❌ - Transaction not confirmed.\nReason: ${confirmation.value.err}`
      );
    }

    console.log("🎉 Transaction Successfully Confirmed!");
    let result = await program.account.actionState.fetch(actionState);
    console.log("Robot action state details: ", result);
  }
});
