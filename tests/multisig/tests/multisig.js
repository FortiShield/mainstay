const mainstay = require("@nx-pkg/mainstay");
const { assert } = require("chai");

describe("multisig", () => {
  // Configure the client to use the local cluster.
  mainstay.setProvider(mainstay.MainstayProvider.env());

  const program = mainstay.workspace.Multisig;

  it("Tests the multisig program", async () => {
    const multisig = mainstay.web3.Keypair.generate();
    const [multisigSigner, nonce] =
      await mainstay.web3.PublicKey.findProgramAddress(
        [multisig.publicKey.toBuffer()],
        program.programId
      );
    const multisigSize = 200; // Big enough.

    const ownerA = mainstay.web3.Keypair.generate();
    const ownerB = mainstay.web3.Keypair.generate();
    const ownerC = mainstay.web3.Keypair.generate();
    const ownerD = mainstay.web3.Keypair.generate();
    const owners = [ownerA.publicKey, ownerB.publicKey, ownerC.publicKey];

    const threshold = new mainstay.BN(2);
    await program.rpc.createMultisig(owners, threshold, nonce, {
      accounts: {
        multisig: multisig.publicKey,
        rent: mainstay.web3.SYSVAR_RENT_PUBKEY,
      },
      instructions: [
        await program.account.multisig.createInstruction(
          multisig,
          multisigSize
        ),
      ],
      signers: [multisig],
    });

    let multisigAccount = await program.account.multisig.fetch(
      multisig.publicKey
    );

    assert.strictEqual(multisigAccount.nonce, nonce);
    assert.isTrue(multisigAccount.threshold.eq(new mainstay.BN(2)));
    assert.deepEqual(multisigAccount.owners, owners);

    const pid = program.programId;
    const accounts = [
      {
        pubkey: multisig.publicKey,
        isWritable: true,
        isSigner: false,
      },
      {
        pubkey: multisigSigner,
        isWritable: false,
        isSigner: true,
      },
    ];
    const newOwners = [ownerA.publicKey, ownerB.publicKey, ownerD.publicKey];
    const data = program.coder.instruction.encode("setOwners", {
      owners: newOwners,
    });

    const transaction = mainstay.web3.Keypair.generate();
    const txSize = 1000; // Big enough, cuz I'm lazy.
    await program.rpc.createTransaction(pid, accounts, data, {
      accounts: {
        multisig: multisig.publicKey,
        transaction: transaction.publicKey,
        proposer: ownerA.publicKey,
        rent: mainstay.web3.SYSVAR_RENT_PUBKEY,
      },
      instructions: [
        await program.account.transaction.createInstruction(
          transaction,
          txSize
        ),
      ],
      signers: [transaction, ownerA],
    });

    const txAccount = await program.account.transaction.fetch(
      transaction.publicKey
    );

    assert.isTrue(txAccount.programId.equals(pid));
    assert.deepEqual(txAccount.accounts, accounts);
    assert.deepEqual(txAccount.data, data);
    assert.isTrue(txAccount.multisig.equals(multisig.publicKey));
    assert.strictEqual(txAccount.didExecute, false);

    // Other owner approves transaction.
    await program.rpc.approve({
      accounts: {
        multisig: multisig.publicKey,
        transaction: transaction.publicKey,
        owner: ownerB.publicKey,
      },
      signers: [ownerB],
    });

    // Now that we've reached the threshold, send the transaction.
    await program.rpc.executeTransaction({
      accounts: {
        multisig: multisig.publicKey,
        multisigSigner,
        transaction: transaction.publicKey,
      },
      remainingAccounts: program.instruction.setOwners
        .accounts({
          multisig: multisig.publicKey,
          multisigSigner,
        })
        // Change the signer status on the vendor signer since it's signed by the program, not the client.
        .map((meta) =>
          meta.pubkey.equals(multisigSigner)
            ? { ...meta, isSigner: false }
            : meta
        )
        .concat({
          pubkey: program.programId,
          isWritable: false,
          isSigner: false,
        }),
    });

    multisigAccount = await program.account.multisig.fetch(multisig.publicKey);

    assert.strictEqual(multisigAccount.nonce, nonce);
    assert.isTrue(multisigAccount.threshold.eq(new mainstay.BN(2)));
    assert.deepEqual(multisigAccount.owners, newOwners);
  });
});
