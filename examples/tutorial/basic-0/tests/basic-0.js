const mainstay = require("@nx-pkg/mainstay");

describe("basic-0", () => {
  // Configure the client to use the local cluster.
  mainstay.setProvider(mainstay.MainstayProvider.local());

  it("Uses the workspace to invoke the initialize instruction", async () => {
    // #region code
    // Read the deployed program from the workspace.
    const program = mainstay.workspace.Basic0;

    // Execute the RPC.
    await program.methods.initialize().rpc();
    // #endregion code
  });
});
