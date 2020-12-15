## Justificación de travis CI



### Travis

Para activar Travis CI en un un repositorio de Gihub hay que darse de alta en travis-ci.com usando la cuenta de Github y una vez iniciada sesión hay basta con seleccionar el repositorio en el que se quiere activar la integración continua.

Mi archivo `.travis.yml` es el siguiente:

```yaml
language: rust
rust:
  - stable
cache: cargo
install: 
  - make build
script:
  - make test

```



Está basado en el [ejemplo de rust que travis provee](ejemplo de rust que travis provee https://docs.travis-ci.com/user/languages/rust/), pero solo compila con la versión estable de Rust para ahorrar minutos en Travis. Además activa la caché de cargo para evitar recompilaciones innecesarias de las dependencias y sobrescribe "install" para usar el gestor de tareas make



### CircleCI

CircleCI es un sistema de integración continua que pone bastante fácil el uso de imágenes de Docker para el testeo de aplicaciones, es por eso que lo he elegido para realizar la integración continua dentro de la imagen que realicé para el hito anterior. El proceso de activación de CircleCI es bastante similar al de Travis, basta con iniciar sesión en su web usando Github y a continuación activar la integración continua en el repositorio deseado. Al hacerlo la web nos proveerá con un archivo de configuración de prueba el cual podemos aceptar que se incluya automáticamente en nuestro repositorio, una vez hecho esto debemos modificar dicho archivo para activar la integración continua. En concreto mi archivo `.circlec\config.yml` es el siguiente:

```yaml
version: 2.1

jobs:
    test:
        docker:
            - image: arturocs/crab-iot:latest
        steps:
            - checkout
            - run: make test

workflows:
    test_in_docker:
        jobs:
            - test

```

Se trata de una versión recortada y adaptada de uno de los [ejemplos que hay en la web de CircleCI](https://circleci.com/docs/2.0/sample-config/). 

En mi versión primero definimos una lista de trabajos con un único trabajo llamado `test`. A continuación le comunicamos que queremos que se ejecute en docker, y especificamos la imagen que queremos que usar, en este caso la imagen creada para el hito anterior. Luego definimos una serie de pasos a ejecutar en la imagen. El paso `checkout` descarga el repositorio git en el que se tiene activada la integración continua,  y el paso `run: make test` ejecuta los tests. A continuación en la lista de workflows se define un workflow que llama al trabajo `test` definido anteriormente.



### Github Actions

Además de para la integración continua, he usado Github actions para buscar malas prácticas en mi código y para formatear el código que se vaya subiendo con cada commit. Mi archivo de acciones para este proyecto puede encontrarse [aquí](https://github.com/arturocs/crab-iot/blob/master/.github/workflows/rust.yml), y está basado en el [ejemplo de actions-rs](https://github.com/actions-rs/meta/blob/master/recipes/quickstart.md), un proyecto que provee acciones de Github para proyectos Rust.  En esta acción se definen seis trabajos:

* test_stable: ejecuta los tests sobre la última versión estable de Rust.
* test_nightly: ejecuta los tests sobre la última versión nightly de Rust.
* test_beta: ejecuta los tests sobre la última versión beta de Rust.
* test_minimal: ejecuta los tests sobre la versión mínima de Rust en la que el proyecto funciona, actualmente la 1.40
* clippy: busca malas prácticas en el código.
* fmt: formatea el código Rust de acorde con el estándar.

La mayoría de trabajos se limitan a llamar a acciones predefinidas y ejecutar el comando correspondiente, pero fmt funciona de manera distinta. Además de las acciones `actions/checkout@v2` y `actions-rs/toolchain@v1` utiliza la acción `mbrobbel/rustfmt-check@master` , esta ultima acción requiere un token que hay que activar en los ajustes de Github. Gracias a este token puede escribir en el repositorio y formatear el código, cosa que no hace el ejemplo de actions-rs que solo se limita a avisar de disconformidades con el formato estándar.

