const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

async function main() {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Load the IDL.
  const idl = JSON.parse(
    require("fs").readFileSync("./target/idl/whitelist_sale.json", "utf8")
  );

  // Address of the deployed program.
  const programId = new anchor.web3.PublicKey("---");

  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId);

  // The account to create.
  const saleAccount = anchor.web3.Keypair.generate();

  // The price and limit for the sale.
  const price = new anchor.BN(100);
  const limit = new anchor.BN(10);

  // Create the sale account.
  await program.rpc.initialize(price, limit, {
    accounts: {
      saleAccount: saleAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [saleAccount],
  });

  console.log("Sale account created:", saleAccount.publicKey.toString());

  // Add a user to the whitelist.
  const user = anchor.web3.Keypair.generate();
  await program.rpc.whitelist(user.publicKey, {
    accounts: {
      saleAccount: saleAccount.publicKey,
    },
  });

  console.log("User whitelisted:", user.publicKey.toString());

  // Simulate a token purchase.
  const amount = new anchor.BN(5);
  await program.rpc.buy(amount, {
    accounts: {
      saleAccount: saleAccount.publicKey,
      buyer: user.publicKey,
    },
    signers: [user],
  });

  console.log("User purchased tokens:", amount.toString());
}

console.log("Running client.");
main()
  .then(() => console.log("Success"))
  .catch((err) => console.error(err));
