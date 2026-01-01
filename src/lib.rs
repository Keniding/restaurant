/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn _deliver_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        super::_deliver_order();
    }

    fn _cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String // seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant_() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    meal.seasonal_fruit = String::from("blueberries");
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("Order {:?}", order1);
    println!("Order {:?}", order2);
}

/*
Por ejemplo, escribamos una biblioteca que proporcione la funcionalidad de un restaurante. Definiremos las firmas de las funciones, pero dejaremos sus cuerpos vacíos para centrarnos en la organización del código en lugar de en la implementación del restaurante.

En la industria restaurantera, algunas partes del restaurante se denominan sala y otras, cocina. La sala es donde se encuentran los clientes; esto abarca donde los anfitriones los sientan, los camareros toman los pedidos y los pagos, y los bármanes preparan las bebidas. La cocina es donde los chefs y cocineros trabajan en la cocina, los lavaplatos limpian y los gerentes realizan las tareas administrativas.

Para estructurar nuestro crate de esta manera, podemos organizar sus funciones en módulos anidados. Crea una nueva biblioteca llamada restaurantmediante la ejecución de cargo new restaurant --lib. Luego, introduce el código del Listado 7-1 en src/lib.rs para definir algunos módulos y firmas de funciones; este código es la sección de front-of-house.

Nombre de archivo: src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
Listado 7-1 : Un front_of_housemódulo que contiene otros módulos que a su vez contienen funciones
Definimos un módulo con la modpalabra clave seguida de su nombre (en este caso, front_of_house). El cuerpo del módulo se coloca entre llaves. Dentro de los módulos, podemos colocar otros módulos, como en este caso con los módulos hostingy serving. Los módulos también pueden contener definiciones de otros elementos, como estructuras, enumeraciones, constantes, rasgos y, como en el Listado 7-1, funciones.

Al usar módulos, podemos agrupar definiciones relacionadas y explicar por qué están relacionadas. Los programadores que usan este código pueden navegar por él según los grupos en lugar de tener que leer todas las definiciones, lo que facilita encontrar las relevantes. Los programadores que añadan nuevas funcionalidades a este código sabrán dónde colocarlo para mantener el programa organizado.

Anteriormente, mencionamos que src/main.rs y src/lib.rs se denominan raíces de crate _. Su nombre se debe a que el contenido de cualquiera de estos dos archivos forma un módulo cuyo nombre se encuentra crateen la raíz de la estructura de módulos del crate, conocida como árbol de módulos .

El listado 7-2 muestra el árbol de módulos para la estructura del listado 7-1.

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
Listado 7-2 : El árbol de módulos para el código del Listado 7-1
Este árbol muestra cómo algunos módulos se anidan dentro de otros; por ejemplo, hosting se anida dentro de front_of_house. El árbol también muestra que algunos módulos son hermanos , lo que significa que están definidos en el mismo módulo; hostingy servingson hermanos definidos dentro de front_of_house. Si el módulo A está contenido dentro del módulo B, decimos que el módulo A es hijo del módulo B y que el módulo B es padre del módulo A. Observe que el árbol de módulos tiene su raíz en el módulo implícito llamado crate.

El árbol de módulos podría recordarte al árbol de directorios del sistema de archivos de tu ordenador; ¡es una comparación muy acertada! Al igual que los directorios en un sistema de archivos, se usan módulos para organizar el código. Y, al igual que los archivos en un directorio, necesitamos una forma de encontrar nuestros módulos.
 */

/*
Incorporar rutas al alcance con la usepalabra clave
Escribir las rutas para llamar a las funciones puede resultar incómodo y repetitivo. En el Listado 7-7, independientemente de si elegimos la ruta absoluta o relativa a la add_to_waitlistfunción, cada vez que queríamos llamarla add_to_waitlist teníamos que especificar también front_of_house"y hosting". Afortunadamente, hay una forma de simplificar este proceso: podemos crear un acceso directo a una ruta con la use palabra clave una vez y luego usar el nombre más corto en el resto del ámbito.

En el Listado 7-11, llevamos el crate::front_of_house::hostingmódulo al ámbito de la eat_at_restaurantfunción de modo que solo tenemos que especificar hosting::add_to_waitlistpara llamar a la add_to_waitlistfunción en eat_at_restaurant.

Nombre de archivo: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listado 7-11 : Incorporación de un módulo al alcance conuse
Añadir useuna ruta a un ámbito es similar a crear un enlace simbólico en el sistema de archivos. Al añadirla use crate::front_of_house::hostingen la raíz del paquete, hostingahora es un nombre válido en ese ámbito, como si el hosting módulo se hubiera definido allí. Las rutas incluidas en el ámbito use también verifican su privacidad, como cualquier otra ruta.

Tenga en cuenta que usesolo crea el acceso directo para el ámbito específico en el que useocurre. El Listado 7-12 mueve la eat_at_restaurantfunción a un nuevo módulo secundario llamado customer, que entonces tiene un ámbito diferente al de la use instrucción, por lo que el cuerpo de la función no se compilará.

Nombre de archivo: src/lib.rs
¡Este código no se compila!
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
Listado 7-12 : Una usedeclaración sólo se aplica en el ámbito en el que se encuentra.
El error del compilador muestra que el acceso directo ya no se aplica dentro del customermódulo:

$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
Tenga en cuenta que también hay una advertencia que indica que useya no se usa en su ámbito. Para solucionar este problema, mueva también el usedentro del customermódulo o haga referencia al acceso directo en el módulo principal con super::hostingdentro del customermódulo secundario.
 */
mod front_of_house_with_use {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house_with_use::hosting;

pub fn eat_at_restaurant_with_use() {
    hosting::add_to_waitlist();
}

// Uso de super o definir dentro del ámbito de mod customer para q reconozca hosting
mod customer {
    pub fn _eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

/*
useCreación de rutas idiomáticas
En el Listado 7-11, es posible que se haya preguntado por qué especificamos use crate::front_of_house::hostingy luego llamamos hosting::add_to_waitlista eat_at_restaurant, en lugar de especificar la useruta hasta la add_to_waitlistfunción para lograr el mismo resultado, como en el Listado 7-13.

Nombre de archivo: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
Listado 7-13 : Incorporar la add_to_waitlistfunción al alcance con use, lo cual no es idiomático
Aunque tanto el Listado 7-11 como el Listado 7-13 realizan la misma tarea, el Listado 7-11 es la forma idiomática de incluir una función en el ámbito con use. Incluir el módulo padre de la función en el ámbito con usesignifica que debemos especificarlo al llamarla. Especificar el módulo padre al llamarla deja claro que la función no está definida localmente, a la vez que minimiza la repetición de la ruta completa. El código del Listado 7-13 no especifica dónde add_to_waitlistse define.
 */
// No recomendado pero factible
use crate::front_of_house_with_use::hosting::add_to_waitlist;

pub fn eat_at_restaurant_with_use_() {
    add_to_waitlist();
}

/*
Por otro lado, al importar estructuras, enumeraciones y otros elementos con use, es idiomático especificar la ruta completa. El Listado 7-14 muestra la forma idiomática de incluir la HashMapestructura de la biblioteca estándar en el ámbito de un contenedor binario.

Nombre de archivo: src/main.rs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
Listado 7-14 : Traer HashMapal ámbito de aplicación de manera idiomática
No hay ninguna razón de peso detrás de este modismo: es simplemente la convención que ha surgido y la gente se ha acostumbrado a leer y escribir código Rust de esta manera.
 */
// Uso de HashMap
use std::collections::HashMap;

fn _main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

/*
La excepción a esta expresión es si traemos al ámbito dos elementos con el mismo nombre mediante usesentencias, ya que Rust no lo permite. El Listado 7-15 muestra cómo traer Resultal ámbito dos tipos con el mismo nombre, pero con diferentes módulos principales, y cómo hacer referencia a ellos.

Nombre de archivo: src/lib.rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
Listado 7-15 : Para traer dos tipos con el mismo nombre al mismo ámbito es necesario utilizar sus módulos principales.
Como puedes ver, usar los módulos principales distingue los dos Resulttipos. Si, en cambio, escribiéramos use std::fmt::Resulty use std::io::Result, tendríamos dos Resulttipos en el mismo ámbito, y Rust no sabría a cuál nos referíamos al usar Result.
 */
// Dos elementos con el mismo nombre no es idóneo
use std::fmt;
use std::io;

fn _function1() -> fmt::Result {
    Ok(())
}

fn _function2() -> io::Result<()> {
    Ok(())
}

/*
Reexportación de nombres conpub use
Al incorporar un nombre al ámbito con la usepalabra clave, este es privado para el ámbito donde lo importamos. Para que el código externo pueda referirse a él como si estuviera definido en él, podemos combinar puby use. Esta técnica se denomina reexportación porque incorporamos un elemento al ámbito, pero también lo ponemos a disposición de otros usuarios para que lo incorporen al suyo.

El listado 7-17 muestra el código del listado 7-11 con useel módulo raíz cambiado a pub use.

Nombre de archivo: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listado 7-17 : Hacer que un nombre esté disponible para que cualquier código lo use desde un nuevo ámbito conpub use
Antes de este cambio, el código externo debía llamar a la add_to_waitlist función usando la ruta restaurant::front_of_house::hosting::add_to_waitlist(), lo que también requería que el front_of_housemódulo se marcara como pub. Ahora que se pub useha reexportado el hostingmódulo desde el módulo raíz, el código externo puede usar la ruta restaurant::hosting::add_to_waitlist()en su lugar.

Reexportar es útil cuando la estructura interna de tu código difiere de cómo los programadores que lo usan conciben el dominio. Por ejemplo, en esta metáfora de un restaurante, quienes lo gestionan piensan en "servicio al cliente" y "servicio técnico". Sin embargo, los clientes que visitan un restaurante probablemente no piensen en las partes del restaurante de esa manera. Con pub use, podemos escribir nuestro código con una estructura, pero exponer una diferente. Esto facilita la organización de nuestra biblioteca tanto para los programadores que trabajan en ella como para quienes la usan. Veremos otro ejemplo de pub use y cómo afecta a la documentación de tu caja en "Exportar una API pública práctica" en el capítulo 14.
 */
// Reexportación
mod front_of_house_with_pub_use {
    pub mod hosting_with_pub_use {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house_with_pub_use::hosting_with_pub_use;

pub fn eat_at_restaurant_with_pub_use() {
    hosting_with_pub_use::add_to_waitlist();
}

/*
Uso de paquetes externos
En el Capítulo 2, programamos un proyecto de juego de adivinanzas que utilizaba un paquete externo llamado randpara obtener números aleatorios. Para usarlo randen nuestro proyecto, añadimos esta línea a Cargo.toml :

Nombre del archivo: Cargo.toml
rand = "0.8.5"
Agregarlo randcomo una dependencia en Cargo.toml le indica a Cargo que descargue el randpaquete y cualquier dependencia de crates.io y los ponga randa disposición de nuestro proyecto.

Luego, para incluir randlas definiciones en el alcance de nuestro paquete, añadimos una uselínea que comenzaba con el nombre de la caja, randy enumeramos los elementos que queríamos incluir. Recuerda que en "Generación de un número aleatorio" del capítulo 2, incluimos el Rngatributo en el alcance y llamamos a la rand::thread_rngfunción:

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
Los miembros de la comunidad Rust han puesto a disposición muchos paquetes en crates.io , y extraer cualquiera de ellos para su paquete implica estos mismos pasos: enumerarlos en el archivo Cargo.toml de su paquete y usarlos usepara traer elementos de sus cajas al alcance.

Tenga en cuenta que la stdbiblioteca estándar también es una caja externa a nuestro paquete. Dado que se incluye con el lenguaje Rust, no necesitamos cambiar Cargo.toml a include std. Sin embargo, sí necesitamos hacer referencia a ella con [nombre del paquete use] para incorporar elementos desde allí al alcance de nuestro paquete. Por ejemplo, con [nombre del paquete] HashMapusaríamos esta línea:

use std::collections::HashMap;
Esta es una ruta absoluta que comienza con std, el nombre de la caja de la biblioteca estándar.
 */
// Dependencias
use rand::Rng;

fn _main_with_rand() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("{}", secret_number.to_string());
}

/*
Uso de rutas anidadas para limpiar uselistas
Si usamos varios elementos definidos en la misma caja o módulo, listar cada elemento en su propia línea puede ocupar mucho espacio vertical en nuestros archivos. Por ejemplo, estas dos usesentencias que teníamos en el juego de adivinanzas del Listado 2-4 traen elementos del stdámbito:

Nombre de archivo: src/main.rs
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
En su lugar, podemos usar rutas anidadas para incluir los mismos elementos en el ámbito de aplicación en una sola línea. Para ello, especificamos la parte común de la ruta, seguida de dos puntos y, a continuación, una lista de las partes de las rutas que difieren entre llaves, como se muestra en el Listado 7-18.

Nombre de archivo: src/main.rs
// --snip--
use std::{cmp::Ordering, io};
// --snip--
Listado 7-18 : Especificación de una ruta anidada para traer múltiples elementos con el mismo prefijo al alcance
En programas más grandes, traer muchos elementos al alcance desde la misma caja o módulo usando rutas anidadas puede reducir usemucho la cantidad de declaraciones separadas necesarias.

Podemos usar una ruta anidada en cualquier nivel de una ruta, lo cual resulta útil al combinar dos usesentencias que comparten una subruta. Por ejemplo, el Listado 7-19 muestra dos usesentencias: una que incluye std::iodentro del ámbito y otra que incluye std::io::Writedentro del ámbito.

Nombre de archivo: src/lib.rs
use std::io;
use std::io::Write;
Listado 7-19 : Dos usedeclaraciones donde una es una subruta de la otra
La parte común de estas dos rutas es std::io, y esa es la primera ruta completa. Para fusionarlas en una sola useinstrucción, podemos usar selfen la ruta anidada, como se muestra en el Listado 7-20.

Nombre de archivo: src/lib.rs
use std::io::{self, Write};
Listado 7-20 : Combinación de las rutas del Listado 7-19 en una sola usedeclaración
Esta línea trae std::ioy std::io::Writedentro del alcance.


Importación de elementos con el operador Glob
Si queremos traer todos los elementos públicos definidos en una ruta al alcance, podemos especificar esa ruta seguida por el *operador glob:

use std::collections::*;
Esta usedeclaración incorpora todos los elementos públicos definidos en std::collectionsel ámbito actual. ¡Tenga cuidado al usar el operador glob! Glob puede dificultar la identificación de los nombres dentro del ámbito y la ubicación de un nombre en su programa. Además, si la dependencia cambia sus definiciones, lo que ha importado también cambia, lo que puede generar errores de compilación al actualizar la dependencia si esta agrega una definición con el mismo nombre que una suya en el mismo ámbito, por ejemplo.

El operador glob se utiliza a menudo durante las pruebas para llevar todo lo que se está probando al testsmódulo; hablaremos de eso en “Cómo escribir pruebas” en el Capítulo 11. El operador glob también se utiliza a veces como parte del patrón de preludio: consulte la documentación de la biblioteca estándar para obtener más información sobre ese patrón.
 */
// Rutas anidadas
// use std::{cmp::Ordering, slice};
// use std::io::{Result, Write};

/*
Separar módulos en archivos diferentes
Hasta ahora, todos los ejemplos de este capítulo definieron varios módulos en un solo archivo. Cuando los módulos sean grandes, conviene trasladar sus definiciones a un archivo aparte para facilitar la navegación por el código.

Por ejemplo, comencemos con el código del Listado 7-17, que tenía varios módulos de restaurante. Extraeremos los módulos a archivos en lugar de tenerlos todos definidos en el archivo raíz del cajón. En este caso, el archivo raíz del cajón es src/lib.rs , pero este procedimiento también funciona con cajones binarios cuyo archivo raíz es src/main.rs .

Primero, extraeremos el front_of_housemódulo a su propio archivo. Elimine el código entre llaves del front_of_housemódulo, dejando solo la mod front_of_house;declaración, para que src/lib.rs contenga el código que se muestra en el Listado 7-21. Tenga en cuenta que esto no se compilará hasta que creemos el archivo src/front_of_house.rs en el Listado 7-22.

Nombre de archivo: src/lib.rs
¡Este código no se compila!
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
Listado 7-21 : Declaración del front_of_housemódulo cuyo cuerpo estará en src/front_of_house.rs
A continuación, coloque el código entre llaves en un nuevo archivo llamado src/front_of_house.rs , como se muestra en el Listado 7-22. El compilador sabe que debe buscar en este archivo porque encontró la declaración del módulo en la raíz del paquete con el nombre front_of_house.

Nombre del archivo: src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
Listado 7-22 : Definiciones dentro del front_of_housemódulo en src/front_of_house.rs
Tenga en cuenta que solo necesita cargar un archivo mediante una moddeclaración una vez en el árbol de módulos. Una vez que el compilador sabe que el archivo forma parte del proyecto (y dónde se encuentra el código en el árbol de módulos gracias a la ubicación de la mod declaración), los demás archivos del proyecto deben hacer referencia al código del archivo cargado mediante la ruta a su declaración, como se explica en la sección "Rutas para hacer referencia a un elemento en el árbol de módulos" . En otras palabras, nomod se trata de una operación de "inclusión" que quizás haya visto en otros lenguajes de programación.

A continuación, extraeremos el hostingmódulo a su propio archivo. El proceso es ligeramente diferente, ya que hostinges un módulo hijo de front_of_house, no del módulo raíz. Colocaremos el archivo for hostingen un nuevo directorio que llevará el nombre de sus antecesores en el árbol de módulos, en este caso src/front_of_house .

Para comenzar a movernos hosting, cambiamos src/front_of_house.rs para que contenga solo la declaración del hostingmódulo:

Nombre del archivo: src/front_of_house.rs
pub mod hosting;
Luego, creamos un directorio src/front_of_house y un archivo hosting.rs para contener las definiciones realizadas en el hostingmódulo:

Nombre de archivo: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
Si, en cambio, colocamos hosting.rs en el directorio src , el compilador esperaría que el código de hosting.rs estuviera en un hostingmódulo declarado en la raíz del paquete y no como un front_of_housemódulo secundario. Las reglas del compilador sobre qué archivos revisar para el código de cada módulo hacen que los directorios y archivos coincidan más estrechamente con el árbol de módulos.

Rutas de archivo alternativas
Hasta ahora hemos cubierto las rutas de archivo más comunes que usa el compilador de Rust, pero Rust también admite un estilo más antiguo de ruta de archivo. Para un módulo llamado front_of_housedeclarado en la raíz del paquete, el compilador buscará el código del módulo en:

src/front_of_house.rs (lo que cubrimos)
src/front_of_house/mod.rs (estilo antiguo, ruta aún compatible)
Para un módulo llamado hostingque es un submódulo de front_of_house, el compilador buscará el código del módulo en:

src/front_of_house/hosting.rs (lo que cubrimos)
src/front_of_house/hosting/mod.rs (estilo antiguo, ruta aún compatible)
Si usas ambos estilos para el mismo módulo, recibirás un error de compilación. Usar ambos estilos combinados para diferentes módulos del mismo proyecto está permitido, pero podría resultar confuso para quienes navegan por el proyecto.

La principal desventaja del estilo que utiliza archivos llamados mod.rs es que su proyecto puede terminar con muchos archivos llamados mod.rs , lo que puede resultar confuso cuando los tiene abiertos en su editor al mismo tiempo.

Hemos movido el código de cada módulo a un archivo separado, y el árbol de módulos permanece igual. Las llamadas a funciones eat_at_restaurantfuncionarán sin modificaciones, aunque las definiciones se encuentren en archivos diferentes. Esta técnica permite mover módulos a nuevos archivos a medida que aumentan de tamaño.

Tenga en cuenta que la pub use crate::front_of_house::hostingdeclaración en src/lib.rs tampoco ha cambiado ni useafecta los archivos que se compilan como parte del crate. La modpalabra clave declara módulos, y Rust busca el código que contiene en un archivo con el mismo nombre.

Resumen
Rust permite dividir un paquete en varias cajas y una caja en módulos para que puedas hacer referencia a los elementos definidos en un módulo desde otro. Puedes hacerlo especificando rutas absolutas o relativas. Estas rutas se pueden incluir en el ámbito de aplicación mediante una usedeclaración, lo que permite usar una ruta más corta para múltiples usos del elemento en ese ámbito. El código del módulo es privado por defecto, pero puedes hacer públicas las definiciones añadiendo la pubpalabra clave.

En el próximo capítulo, veremos algunas estructuras de datos de recopilación en la biblioteca estándar que puedes usar en tu código perfectamente organizado.
 */
//Implementación con archivo separado
mod front_of_house_with_file;

pub use crate::front_of_house_with_file::hosting_with_file;

pub fn eat_at_restaurant_with_file() {
    hosting_with_file::add_to_waitlist();
}

// Segmentación de archivos en archivos y carpetas referenciando con el mismo nombre carpeta y archivo
mod front_of_house_with_fragmentation;
pub use crate::front_of_house_with_fragmentation::hosting_with_fragmentation;

pub fn eat_at_restaurant_with_fragmentation() {
    hosting_with_fragmentation::add_to_waitlist();
}