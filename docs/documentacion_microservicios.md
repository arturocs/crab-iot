 

### 1. Justificación técnica del framework elegido para el microservicio.

En el ecosistema de rust, actualmente hay tres principales framework para servicios rest:

* [Rocket](https://crates.io/crates/rocket): Es la librería mas fácil de usar de las tres, hasta hace poco no tenía soporte para funciones asíncronas, por lo que solía ser la mas lenta de las tres. No obstante recientemente ha recortado distancias con las otras dos librerías en términos de rendimiento. La principal contra que tiene es que requiere usar rust nightly.
* [Actix-web](https://crates.io/crates/actix-web): Se trata de uno de los frameworks mas rápidos actualmente según [Tech Empower](https://www.techempower.com/benchmarks/). Está bastante bien documentado, es relativamente sencillo de usar y posee un gran conjunto de herramientas.
* [Warp](https://crates.io/crates/warp): Es la librería mas joven de las tres, se supone que fue creada para simplificar el proceso de creación de APIs sin comprometer el rendimiento, no obstante a mi parecer es la mas pesada de usar de las tres.

[En este benchmark](https://aspenuwu.me/blog/rust-web-frameworks-2021-benchmarked/) se comparan las versiones mas recientes de los tres frameworks y como se puede apreciar las diferencias en rendimiento no son significativas. Por lo que la decisión se reduce a cuestión de gustos. 

Finalmente he elegido actix-web por todas las herramientas que trae así como los abundantes tutoriales.

### 2. Diseño en general del API y las rutas.

He dividido las rutas en dispositivos de solo lectura y dispositivos de lectura/escritura. Esta decisión ha venido en parte dada por el diseño interno de crab-iot, ya que los dispositivos de solo lectura están representados con un tipo distinto a los dispositivos de lectura/escritura, por lo que no pueden ser almacenados en la misma estructura de datos sin recurrir a algún truco como [std::any](https://doc.rust-lang.org/std/any/).

Para añadir un nuevo dispositivo hay que hacer una peticion post a /rdevice o /rwdevice dependiendo de si el dispositivo a crear es de solo lectura o de lectura/escritura. De la misma forma haciendo una peticion delete a /rwdevice/interruptor, eliminaremos el dispositivo con permisos de lectura/escritura llamado interruptor. Tambien demos recibir una lista de los dispositivos de solo lectura diponibles realizado una petición get a /rdevices.

También se han añadido un par de rutas para los dispositivos falsos que creé durante los hitos anteriores. Si se realiza una peticion get a /rdevice/weather/forecast/n, el sistema devolverá la predicción del tiempo para dentro de n días, siendo el máximo n posible 7. 

Realizando una petición get a /rwdevice/fake_switch se puede ver el estado del interruptor falso, mientras que si realizamos la peticion post pertinente podremos encenderlo o apagarlo.

### 3. Configuración distribuida, logs.

Para la configuración distribuida se ha usado la librería [etcd-client](https://crates.io/crates/etcd-client), y si se arranca el programa principal, tratará de utilizar el valor asociado la clave "IP", como ip donde hostear el servicio, y el valor de "PORT" como el puerto en el que servir el microservicio. En caso de que dichas claves no estén definidas, utilizará por defecto 127.0.0.1 y 8080.

Para los logs he usado el middleware de logs de actix y el logger [simple_logger](https://crates.io/crates/simple_logger)

### 4. Tests correctos y de acuerdo con las historias de usuario.

Los tests de la api añadidos cierran las siguientes historias de usuario:

* [HU Añadir soporte para nuevos dispositivos facilmente](https://github.com/arturocs/crab-iot/issues/6)
* [HU Modificar estado de un actuador](https://github.com/arturocs/crab-iot/issues/2)
* [HU Consultar información recopilada por un dispositivo](https://github.com/arturocs/crab-iot/issues/1)       

Así como las issues:

* [Devolver información climática](https://github.com/arturocs/crab-iot/issues/42)
* [Listar dispositivos enlazados](https://github.com/arturocs/crab-iot/issues/14)

### 5. Cantidad de  trabajo invertido,  creación de una imagen Docker para desplegarlo, configuración  correcta del gestor de tareas.

Además del servicio REST, se ha diseñado un nuevo plugin mas realista así como un dispositivo falso que interactua con él mediante el protocolo CoAP, un protocolo diseñado para el intercambio de datos en el Internet de las cosas. El programa que simula ser este dispositivo se descarga la predicción climática de Internet cada hora y la pone a disposición del plugin, la idea es simular una estación meteorológica 

También se ha añadido la orden install al makefile, así como un nuevo dockerfile que compila y ejecuta crab-iot.

