# kinder-solana-program
Programa CRUD de estudiantes en Solana usando Anchor
/*
PROGRAMA: Sistema de gestión de estudiantes para un kínder en Solana

Descripción general:
Este programa está desarrollado en Rust utilizando el framework Anchor para la blockchain de Solana.
Su objetivo es implementar un sistema básico de gestión de estudiantes (CRUD) almacenado en una cuenta
de la blockchain.

El programa permite registrar un kínder y administrar una lista de estudiantes asociados a él.

Funcionalidades principales:
1. Crear un registro de kínder en la blockchain.
2. Agregar estudiantes al kínder.
3. Consultar la lista de estudiantes almacenados.
4. Modificar los datos de un estudiante existente.
5. Eliminar estudiantes del registro.

Estructura del programa:

1. Declaración del programa
Se define el ID del programa en la red de Solana mediante `declare_id`, el cual identifica
de manera única al contrato inteligente.

2. Módulo principal (program)
Contiene las funciones que pueden ejecutarse desde el cliente o frontend:

- crear_kinder()
  Inicializa una cuenta en la blockchain que almacenará la información del kínder y
  una lista de estudiantes.

- agregar_estudiante()
  Permite añadir un nuevo estudiante al vector de estudiantes almacenado en la cuenta.

- ver_estudiantes()
  Recorre la lista de estudiantes y muestra su información en los logs mediante `msg!`.

- modificar_estudiante()
  Permite actualizar el nombre o edad de un estudiante utilizando su índice dentro del vector.

- eliminar_estudiante()
  Elimina un estudiante del vector usando su índice.

3. Estructuras de datos

- Kinder
  Representa la cuenta principal donde se almacenan:
  - la clave pública del propietario
  - el nombre del kínder
  - la lista de estudiantes

- Estudiante
  Representa la información de cada estudiante:
  - nombre
  - edad

4. Contextos (Accounts)
Los contextos definen qué cuentas de Solana participan en cada instrucción:

- CrearKinder
  Inicializa la cuenta del kínder utilizando una PDA derivada del owner.

- EditarKinder
  Permite modificar la cuenta existente del kínder.

- VerKinder
  Permite consultar la información almacenada.

5. Manejo de errores
Se utilizan códigos de error personalizados mediante `#[error_code]`
para validar condiciones como:
- lista llena
- índice fuera de rango
*/
