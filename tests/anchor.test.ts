// No imports needed: web3, anchor, pg and more are globally available

describe("Tienda", () => {
  it("Crear tienda", async () => {

    // Obtener wallet del usuario
    const owner = pg.wallet.publicKey;

    // Crear PDA de la tienda
    const [tiendaPda] = await web3.PublicKey.findProgramAddress(
      [
        Buffer.from("tienda"),
        owner.toBuffer(),
      ],
      pg.program.programId
    );

    // Enviar transacción para crear la tienda
    const txHash = await pg.program.methods
      .crearTienda("Zapatería Noelia")
      .accounts({
        owner: owner,
        tienda: tiendaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Usa 'solana confirm -v ${txHash}' para ver los logs`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Obtener datos de la cuenta en la blockchain
    const tienda = await pg.program.account.tienda.fetch(tiendaPda);

    console.log("Datos de la tienda en blockchain:", tienda);

    // Verificar datos
    assert(tienda.nombre === "Zapatería Noelia");
  });
});
