use anchor_lang::prelude::*;

declare_id!("CPydZL5gyh3Tk5Zj28tnJrF1XbVefPgzpQu5VKjhEDRF");

#[program]
pub mod tienda_zapatos {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let zapatos: Vec<Zapato> = Vec::new();

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            zapatos,
        });

        Ok(())
    }

    pub fn agregar_zapato(context: Context<NuevoZapato>, nombre: String, precio: u16) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let zapato = Zapato {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.tienda.zapatos.push(zapato);

        Ok(())
    }

    pub fn eliminar_zapato(context: Context<NuevoZapato>, nombre: String) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let zapatos = &mut context.accounts.tienda.zapatos;

        for i in 0..zapatos.len() {

            if zapatos[i].nombre == nombre {

                zapatos.remove(i);

                msg!("Zapato {} eliminado!", nombre);

                return Ok(());
            }
        }

        Err(Errores::ZapatoNoExiste.into())
    }

    pub fn ver_zapatos(context: Context<NuevoZapato>) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de zapatos: {:#?}", context.accounts.tienda.zapatos);

        Ok(())
    }

    pub fn alternar_disponibilidad(context: Context<NuevoZapato>, nombre: String) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let zapatos = &mut context.accounts.tienda.zapatos;

        for i in 0..zapatos.len() {

            let estado = zapatos[i].disponible;

            if zapatos[i].nombre == nombre {

                let nuevo_estado = !estado;

                zapatos[i].disponible = nuevo_estado;

                msg!("El zapato {} ahora tiene disponibilidad: {}", nombre, nuevo_estado);

                return Ok(());
            }
        }

        Err(Errores::ZapatoNoExiste.into())
    }

    pub fn total_zapatos(context: Context<NuevoZapato>) -> Result<()> {

        let total = context.accounts.tienda.zapatos.len();

        msg!("La tienda tiene {} zapatos registrados", total);

        Ok(())
    }
}

#[error_code]
pub enum Errores {

    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("El zapato no existe")]
    ZapatoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    zapatos: Vec<Zapato>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Zapato {

    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoZapato<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
