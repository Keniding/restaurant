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
