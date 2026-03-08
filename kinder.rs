//ELIZABETH ROJAS PEREZ
//PROGRAMA QUE DA DE ALTA A LOS NIÑOS A UN KINDER SONRISAS
//CONTINE UN CRUD BASICO, ESPERO QUE CONTENGA LO NECESARIO PARA OBTENER LA CERTIFICACIÓN


use anchor_lang::prelude::*;

declare_id!("FNjRAE4EVkijUZBBP1Svfm3nBp9JKjTrKq7CkLVu3WN");
//EL PROGRAMA DA DE ALTA A UN MAÑIMO DE 50 ESTUDIANTES
pub const MAX_ESTUDIANTES: usize = 50;

#[program]
pub mod kinder {
    use super::*;
//FUNCIÓN QUE CREA EL NOMBRE DE KINDER
    pub fn crear_kinder(ctx: Context<CrearKinder>, nombre: String) -> Result<()> {

        let kinder = &mut ctx.accounts.kinder;
        kinder.owner = ctx.accounts.owner.key();
        kinder.nombre = nombre;
        kinder.estudiantes = Vec::new();

        Ok(())
    }
//AGREGAR ESTUDIANTES
    pub fn agregar_estudiante(
        ctx: Context<EditarKinder>,
        nombre: String,
        edad: u8,
    ) -> Result<()> {

        let kinder = &mut ctx.accounts.kinder;

        require!(
            kinder.estudiantes.len() < MAX_ESTUDIANTES,
            ErrorCodigo::ListaLlena
        );

        let estudiante = Estudiante {
            nombre,
            edad,
        };

        kinder.estudiantes.push(estudiante);

        Ok(())
    }
    //MODIFICAR DATOS DEL ALUMNO
    pub fn modificar_estudiante(
    ctx: Context<EditarKinder>,
    indice: u16,
    nombre: String,
    edad: u8,
) -> Result<()> {

    let kinder = &mut ctx.accounts.kinder;

    require!(
        (indice as usize) < kinder.estudiantes.len(),
        ErrorCodigo::IndiceInvalido
    );

    let estudiante = &mut kinder.estudiantes[indice as usize];

    estudiante.nombre = nombre;
    estudiante.edad = edad;

    Ok(())
}
//ELIMINA A UN ESTUDIANTE QUE SE DA DE BAJA
    pub fn eliminar_estudiante(
        ctx: Context<EditarKinder>,
        indice: u16,
    ) -> Result<()> {

        let kinder = &mut ctx.accounts.kinder;

        require!(
            (indice as usize) < kinder.estudiantes.len(),
            ErrorCodigo::IndiceInvalido
        );

        kinder.estudiantes.remove(indice as usize);

        Ok(())
    }
//MUESTRA LA LISTA DE ESTUDIANTES DADOS DE ALTA
    pub fn ver_estudiantes(ctx: Context<VerKinder>) -> Result<()> {

        let kinder = &ctx.accounts.kinder;

        msg!("Nombre del Kinder: {}", kinder.nombre);
        msg!("Total estudiantes: {}", kinder.estudiantes.len());

        for (i, e) in kinder.estudiantes.iter().enumerate() {
            msg!("Estudiante {}:", i);
            msg!("Nombre: {}", e.nombre);
            msg!("Edad: {}", e.edad);
        }

        Ok(())
    }
}
//------------CUENTA--------
#[account]
pub struct Kinder {
    pub owner: Pubkey,
    pub nombre: String,
    pub estudiantes: Vec<Estudiante>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Estudiante {
    pub nombre: String,
    pub edad: u8,
}
//------------CONTEXTO---------
#[derive(Accounts)]
pub struct CrearKinder<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + 6000,
        seeds = [b"kinder", owner.key().as_ref()],
        bump
    )]
    pub kinder: Account<'info, Kinder>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditarKinder<'info> {

    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"kinder", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub kinder: Account<'info, Kinder>,
}

#[derive(Accounts)]
pub struct VerKinder<'info> {

    #[account(
        seeds = [b"kinder", kinder.owner.as_ref()],
        bump
    )]
    pub kinder: Account<'info, Kinder>,
}

#[error_code]
pub enum ErrorCodigo {

    #[msg("La lista de estudiantes está llena")]
    ListaLlena,

    #[msg("Indice fuera de rango")]
    IndiceInvalido,
}
