# TP1: Criptografía Práctica y Blockchain

TP1 de la materia Criptografía Práctica y Blockchain de la Facultad de Ingeniería de la Universidad de Buenos Aires.

## Autor del trabajo práctico
| Nombre y Apellido | Padron | Mail |
|-------------------|--------|------|
| Federico Jose Pacheco | 104541 | fpacheco@fi.uba.ar |

## Ejecucion de tests

Se implementó en el lenguaje Rust. Para ejecutar los tests se debe ejecutar el siguiente comando:

```
cargo test
```

## Item 1

> Implementar un tipo de dato para un elemento de cuerpo finito, junto con sus operaciones aritméticas fundamentales (adición, sustracción, multiplicación y división).


Se implementó un tipo de dato [**Finite Body**](src/finite_body/finite_body.rs) que cumple lo solicitado. Se hizo uso de diferentes traits de Rust para la implementación de las diversas operaciones.

* _**Suma**_: La implementación se encuentra en el archivo [src/finite_body/add.rs](src/finite_body/add.rs)
* _**Resta**_: La implementación se encuentra en el archivo [src/finite_body/sub.rs](src/finite_body/sub.rs)
* _**Multiplicación**_: La implementación se encuentra en el archivo [src/finite_body/mul.rs](src/finite_body/mul.rs)
* _**División**_: La implementación se encuentra en el archivo [src/finite_body/div.rs](src/finite_body/div.rs)

Este TDA se crea teniendo en cuenta el valor del módulo, el cual se debe especificar al momento de crear una instancia del mismo; y el valor del elemento, el cual también se lo debe especificar al momento de crear. Hay diversos tests en cada uno de estos archivos que prueban el funcionamiento de las operaciones aritméticas solicitadas.

Adicionalmente se implementaron diferentes traits que se encuentran en [src/finite_body/others.rs](src/finite_body/others.rs) para poder hacer uso de diferentes operadores en los tests, en las operaciones aritméticas del cuerpo finito y además también para implementar los puntos de una curva elíptica (por ejemplo era necesario que se implemente la potencia o la Negación de un elemento del cuerpo finito teniendo en cuenta el valor del módulo p).

Para la división, hay que buscar el inverso multiplicativo modular. Para hallar el inverso se utiliza el algoritmo con el Pequeño Teorema de Fermat. Este algoritmo se encuentra en [src/finite_body/finite_body.rs:inverse()](src/finite_body/finite_body.rs). Además está comentado el código donde se hace uso también con el Algoritmo de Euclides Extendido. 


## Item 2

> Implementar un tipo de dato para puntos de una curva elíptica, junto con las operaciones de grupo (suma de puntos distintos y duplicación de puntos), utilizando la forma de Weierstrass. Hacer pruebas con la curva $y^2 = x^3 -3x - 3$ con $p=1021$, determinando la cantidad de puntos que tiene la curva. Usando $P=(379,1011)$, obtener $kP$ , siendo $k=655$. 

Se implementó un tipo de dato [**Point Elliptic Curve**](src/elliptic_curve/point_elliptic_curve.rs) que cumple lo solicitado. En mencionado TDA está el trait **add** implementado que contempla la suma de puntos distintos y la duplicación de puntos. También se hace implementó el trait **mul** escalar para poder hacer la multiplicación de un punto por un escalar. Y el trait **neg** para poder hacer la negación de un punto de la curva. También hay otros traits extras implementados que son de utilidad para los tests.

El cálculo de la cantidad de puntos de la curva se encuentra en un test titulado "test_09_ejercicio2()" en el módulo de tests del mismo archivo del TDA. Además en ese test se muestra el cálculo de $kP$. Se puede ejecutar específicamente este ejercicio de esta forma:

```
cargo test --package tp1 --lib -- elliptic_curve::point_elliptic_curve::test_points_eliptic_curve::test_09_ejercicio2 --exact --nocapture 
```

También en el mismo archivo TDA hay diferentes tests que prueben el funcionamiento de las operaciones de grupo solicitadas.


## Item 3

> Implementar un esquema básico de acuerdo de clave de Diffie-Hellman usando curvas elípticas. Usar la curva con $p=43$, $y^2=x^3+6$ y como generador $G_1=(13,15)$. ¿Qué sucede si se emplea el punto $G_2=(9,2)$?

La resolución de este ítem se encuentra en el módulo de tests del archivo [**Agreement**](src/agreement.rs). En el mismo se implementó el esquema de acuerdo de clave de Diffie-Hellman donde se hace uso del TDA de Puntos de Curvas Elípticas implementado en el ítem anterior. 

En este archivo se implementa la estructura de *Agreement* que será el acuerdo que establece Alice y Bob, que incluirá los parámetros de la curva elíptica, el valor p de módulo para el cuerpo finito, y el generador. 

Este acuerdo simula los datos públicos que estarán en la internet que cualquiera podrá ver. En el test test_01_ejercicio3_g1 se simula Diffie-Helmann con G1. Se establece el acuerdo, ALICE Y BOB calculan internamente su clave privada que será un valor dentro del orden del grupo de G (en principio debe ser un valor dentro del módulo p del cuerpo finito). Luego se calcula la clave pública con esta clave privada y el generador G. El resultado de esta clave pública es la que se enviará por internet y todo atacante podrá conocer, como también el acuerdo.

Cuando ALICE y BOB reciben las claves públicas que calcularon y enviaron respectivamente: cada uno usando su clave privada va a generar la clave compartida y obteniendo el secreto en común. En los tests se puede ver el assert_eq! que confirma que ambos obtienen el mismo secreto en común.

Esto se hace tanto para el generador $G_1$ y $G_2$. La principal diferencia de uso de estos generadores es que $G_1$ es un generador donde el orden del grupo es primo pero con un valor muy chico, al ejecutar el test_01_ejercicio3_g1 se puede ver el output que la cantidad de puntos del grupo $G_1$ es igual a 13. En cambio con $G_2$ al ejecutar test_02_ejercicio3_g2 se puede observar que el orden del grupo no es primo y tiene un valor de 39, siendo más alto que $G_1$. 

Para ambos casos, son susceptibles a ataques puesto que $G_1$ tiene un orden primo, pero muy chico. Con $G_2$ tiene un orden mayor, pero no es primo. Si nos basamos en la cantidad de puntos del grupo, $G_1$ seria mas rapido de romper resolviendo el problema del logaritmo discreto.

Para ejecutar el caso con $G_1$:

```
cargo test --package tp1 --lib -- agreement::tests::test_01_ejercicio3_g1 --exact --nocapture 
```

Para ejecutar el caso con $G_2$:

```
cargo test --package tp1 --lib -- agreement::tests::test_02_ejercicio3_g2 --exact --nocapture 
```

## Item 4

> Considerar la curva $y^2=x^3+905x+100$ definida sobre el cuerpo primo de orden $1021$ y el punto generador $(1006, 416)$. Desarrollar alguna estrategia que permita resolver el problema del logaritmo discreto $kP=(612, 827)$.

La resolución de este ítem se encuentra en el módulo de tests del archivo [**Discrete Logarithm Problem**](src/elliptic_curve/discrete_logarithm_problem.rs). Se implementaron dos algoritmos, uno por fuerza bruta y otro por Baby Step Giant Step. Ahí se calcula el valor de k, como también se calcula el valor de kP para verificar que sea igual al punto dado. Adicionalmente se muestra el tiempo de ejecución de cada algoritmo donde se puede ver que el algoritmo de Baby Step Giant Step es mucho más rápido que el de fuerza bruta. Aun así es un problema que para curvas elípticas con un orden de grupo muy grande, no es posible resolverlo en un tiempo razonable.

Para ejecutar este ejercicio:

```
cargo test --package tp1 --lib -- elliptic_curve::discrete_logarithm_problem::test_algorithms_dlp --nocapture 
```

## Referencias y Recursos utilizados

* Programming Bitcoin: Learn How to Program Bitcoin from Scratch by Jimmy Song
* http://www.christelbach.com/ECCalculator.aspx
* https://es.planetcalc.com/3311/
* https://andrea.corbellini.name/2015/06/08/elliptic-curve-cryptography-breaking-security-and-a-comparison-with-rsa/