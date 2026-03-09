// Importa las herramientas principales de Anchor para trabajar con programas en Solana
use anchor_lang::prelude::*;

// ID único del programa desplegado en la blockchain
declare_id!("");

// Módulo principal del programa
#[program]
pub mod tienda_zapatos {

    // Permite usar las funciones y estructuras definidas arriba
    use super::*;

    // ---------------------------------------------------
    // FUNCIÓN PARA CREAR LA TIENDA
    // ---------------------------------------------------
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        // Obtiene la clave pública del dueño de la tienda
        let owner_id = context.accounts.owner.key();

        // Muestra en los logs el ID del dueño
        msg!("Owner id: {}", owner_id);

        // Crea un vector vacío donde se guardarán los zapatos
        let zapatos: Vec<Zapato> = Vec::new();

        // Guarda los datos iniciales dentro de la cuenta tienda
        context.accounts.tienda.set_inner(Tienda {

            // Guarda el dueño de la tienda
            owner: owner_id,

            // Guarda el nombre de la tienda
            nombre,

            // Guarda la lista de zapatos vacía
            zapatos,
        });

        Ok(())
    }

    // ---------------------------------------------------
    // FUNCIÓN PARA AGREGAR UN ZAPATO
    // ---------------------------------------------------
    pub fn agregar_zapato(context: Context<NuevoZapato>, nombre: String, precio: u16) -> Result<()> {

        // Verifica que quien ejecuta la función sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Crea un nuevo zapato
        let zapato = Zapato {

            // Nombre del zapato
            nombre,

            // Precio del zapato
            precio,

            // Por defecto estará disponible
            disponible: true,
        };

        // Agrega el zapato al vector de la tienda
        context.accounts.tienda.zapatos.push(zapato);

        Ok(())
    }

    // ---------------------------------------------------
    // FUNCIÓN PARA ELIMINAR UN ZAPATO
    // ---------------------------------------------------
    pub fn eliminar_zapato(context: Context<NuevoZapato>, nombre: String) -> Result<()> {

        // Verifica que solo el dueño pueda eliminar
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene la lista de zapatos
        let zapatos = &mut context.accounts.tienda.zapatos;

        // Recorre todos los zapatos registrados
        for i in 0..zapatos.len() {

            // Si encuentra el zapato con ese nombre
            if zapatos[i].nombre == nombre {

                // Lo elimina de la lista
                zapatos.remove(i);

                // Mensaje en los logs
                msg!("Zapato {} eliminado!", nombre);

                return Ok(());
            }
        }

        // Si no encuentra el zapato lanza un error
        Err(Errores::ZapatoNoExiste.into())
    }

    // ---------------------------------------------------
    // FUNCIÓN PARA VER TODOS LOS ZAPATOS
    // ---------------------------------------------------
    pub fn ver_zapatos(context: Context<NuevoZapato>) -> Result<()> {

        // Verifica que solo el dueño pueda verlos
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Imprime la lista completa de zapatos en los logs
        msg!("Lista de zapatos: {:#?}", context.accounts.tienda.zapatos);

        Ok(())
    }

    // ---------------------------------------------------
    // FUNCIÓN PARA CAMBIAR DISPONIBILIDAD
    // ---------------------------------------------------
    pub fn alternar_disponibilidad(context: Context<NuevoZapato>, nombre: String) -> Result<()> {

        // Verifica que sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtiene la lista de zapatos
        let zapatos = &mut context.accounts.tienda.zapatos;

        // Recorre todos los zapatos
        for i in 0..zapatos.len() {

            // Guarda el estado actual
            let estado = zapatos[i].disponible;

            // Busca el zapato por nombre
            if zapatos[i].nombre == nombre {

                // Cambia el estado (true -> false o false -> true)
                let nuevo_estado = !estado;

                // Guarda el nuevo estado
                zapatos[i].disponible = nuevo_estado;

                // Muestra el cambio en los logs
                msg!("El zapato {} ahora tiene disponibilidad: {}", nombre, nuevo_estado);

                return Ok(());
            }
        }

        // Si no existe el zapato
        Err(Errores::ZapatoNoExiste.into())
    }

    // ---------------------------------------------------
    // FUNCIÓN PARA CONTAR ZAPATOS
    // ---------------------------------------------------
    pub fn total_zapatos(context: Context<NuevoZapato>) -> Result<()> {

        // Cuenta cuantos zapatos hay registrados
        let total = context.accounts.tienda.zapatos.len();

        // Muestra el total en los logs
        msg!("La tienda tiene {} zapatos registrados", total);

        Ok(())
    }
}

// ---------------------------------------------------
// ENUM DE ERRORES PERSONALIZADOS
// ---------------------------------------------------
#[error_code]
pub enum Errores {

    // Error si alguien que no es el dueño intenta modificar la tienda
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    // Error si el zapato no existe
    #[msg("El zapato no existe")]
    ZapatoNoExiste,
}

// ---------------------------------------------------
// ESTRUCTURA PRINCIPAL DE LA TIENDA
// ---------------------------------------------------
#[account]
#[derive(InitSpace)]
pub struct Tienda {

    // Clave pública del dueño de la tienda
    owner: Pubkey,

    // Nombre de la tienda
    #[max_len(60)]
    nombre: String,

    // Lista de zapatos registrados
    #[max_len(10)]
    zapatos: Vec<Zapato>,
}

// ---------------------------------------------------
// ESTRUCTURA DE CADA ZAPATO
// ---------------------------------------------------
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Zapato {

    // Nombre del zapato
    #[max_len(60)]
    nombre: String,

    // Precio del zapato
    precio: u16,

    // Estado de disponibilidad
    disponible: bool,
}

// ---------------------------------------------------
// CONTEXTO PARA CREAR LA TIENDA
// ---------------------------------------------------
#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    // Cuenta del dueño que firma la transacción
    #[account(mut)]
    pub owner: Signer<'info>,

    // Cuenta donde se almacenará la tienda
    #[account(
        init, // se crea la cuenta
        payer = owner, // el dueño paga la creación
        space = Tienda::INIT_SPACE + 8, // espacio en memoria
        seeds = [b"tienda", owner.key().as_ref()], // semilla para el PDA
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    // Programa del sistema necesario para crear cuentas
    pub system_program: Program<'info, System>,
}

// ---------------------------------------------------
// CONTEXTO PARA MODIFICAR ZAPATOS
// ---------------------------------------------------
#[derive(Accounts)]
pub struct NuevoZapato<'info> {

    // Dueño que firma la transacción
    pub owner: Signer<'info>,

    // Cuenta de la tienda que será modificada
    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
