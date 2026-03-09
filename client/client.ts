// Client

console.log("Mi dirección:", pg.wallet.publicKey.toString());

// obtener balance
const balance = await pg.connection.getBalance(pg.wallet.publicKey);

// mostrar balance en SOL
console.log(`Mi balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
