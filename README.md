Mi proyecto es un programa hecho en Solana Anchor Framework que sirve para administrar una tienda de zapatos dentro de la blockchain de Solana.

La idea es que el dueño de la tienda pueda guardar y controlar los zapatos que tiene, como si fuera un pequeño inventario.

Primero se crea la tienda con la función crear_tienda, donde se guarda el nombre de la tienda y el dueño. Cuando se crea, la lista de zapatos empieza vacía.

Después el dueño puede usar agregar_zapato para registrar nuevos zapatos. Cada zapato tiene un nombre, un precio y un estado de disponibilidad, que indica si está disponible o no.

También existe la función ver_zapatos, que muestra en los logs todos los zapatos que están registrados en la tienda.

Si se necesita borrar un zapato, se usa la función eliminar_zapato, que busca el zapato por su nombre y lo elimina de la lista.

Otra función es alternar_disponibilidad, que sirve para cambiar si un zapato está disponible o no disponible.

Por último, la función total_zapatos muestra cuántos zapatos hay registrados en la tienda.

Además, el programa tiene una validación para que solo el dueño de la tienda pueda hacer cambios, para evitar que otras personas modifiquen los datos.

En resumen, el proyecto funciona como un sistema simple para administrar zapatos en una tienda usando blockchain.
