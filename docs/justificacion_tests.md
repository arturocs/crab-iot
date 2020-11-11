 # Justificación de las herramientas para tests elegidas



**Biblioteca de aserciones:**

Rust incluye un sistema de aserciones TDD en su biblioteca estándar, cuyo uso está muy extendido, de hecho no he visto ningún proyecto de envergadura media o alta que no la use. No obstante tras investigar me he decantado por las aserciones de [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions), una biblioteca TDD similar a la de rust pero que colorea y añade legibilidad a los fallos en aserciones. Otras opciones con las que he encontrado son:

* [totems](https://crates.io/crates/totems): una biblioteca ligera de aserciones TDD, no se diferencia demasiado de la biblioteca estándar de rust, aunque aporta algo de flexibilidad al declarar aserciones, pero prácticamente no tiene usuarios, además las mejoras que provee no son significativas.
* [galvanic-assert](https://github.com/mindsbackyard/galvanic-assert) : se trata de una biblioteca BDD mas avanzada cuyo propósito es hacer las aserciones mas legibles, aunque a mi parecer solo complica su declaración y tampoco añade mejoras a su legibilidad. Además lleva tres años sin recibir actualizaciones.

  

**Marco de pruebas:**

Para el marco de pruebas esta vez me he decantado por el marco de tests de rust, un sistema TDD que viene integrado en el ecosistema y que además es el mas popular. No parece haber muchas alternativas, no obstante también he considerado el sistema [galvanic-test](https://github.com/mindsbackyard/galvanic-test), un marco que puede ser BDD o TDD dependiendo de como se use su proyecto hermano, [galvanic-assert](https://github.com/mindsbackyard/galvanic-assert). Comparte el mismo problema que tiene  [galvanic-assert](https://github.com/mindsbackyard/galvanic-assert), lleva dos años sin actualizarse, aunque su principal pega es que todos los test deben estar dentro de una macro, y usar demasiadas macros en rust se paga con tiempos de compilación bastante altos.



**Task manager**

Inicialmente pensaba usar cargo, el task manager oficial de rust y , pero me terminé dando cuenta de que algunas de las acciones requerían mas de una llamada a cargo, por ejemplo la compilación del plugin requería una llamada a cargo ella sola. Finalmente opté por usar GNU Make para encadenar dichas llamadas en un solo argumento de make, ya que es una herramienta sencilla de usar y que generalmente todo sistema linux ya tiene instalado.