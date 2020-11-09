 # Justificación de las herramientas para tests elegidas



**Biblioteca de aserciones:**

Tras investigar me he decantado por las aserciones incluidas en la biblioteca estándar de rust, el principal motivo ha sido que dicha biblioteca es lo suficientemente buena, así como la mas usada. Además prefiero mantener al mínimo posible las dependencias, ya que los tiempos de compilación de rust son bastante largos. No obstante durante mi investigación me he encontrado con las siguientes opciones:

* [totems](https://crates.io/crates/totems): una biblioteca ligera de aserciones, aporta algo de flexibilidad al declarar aserciones, pero lleva demasiado sin actualizar y las mejoras que provee no son significativas.
* [galvanic-assert](https://github.com/mindsbackyard/galvanic-assert) : se trata de una biblioteca mas avanzada cuyo propósito es hacer las aserciones mas legibles, aunque a mi parecer solo complica su declaración y tampoco añade mejoras a su legibilidad. Además lleva dos años sin recibir actualizaciones.
* [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions): Una biblioteca que colorea y añade legibilidad a los fallos en aserciones. Parece ser bastante útil en los casos en los que la aserción trabaja con tipos anidados. No obstante también lleva dos años sin actualizarse.

**Marco de pruebas:**

De igual manera también me he decantado por el marco de tests de rust, ya que es el mas popular y viene integrado en el ecosistema. También he considerado el sistema [galvanic-test](https://github.com/mindsbackyard/galvanic-test), pero a mi parecer solo complica la creación de tests y de la misma forma que su proyecto hermano, [galvanic-assert](https://github.com/mindsbackyard/galvanic-assert),  lleva dos años sin actualizarse.

