# Crab-IoT 🦀

[![Build Status](https://travis-ci.org/arturocs/crab-iot.svg?branch=master)](https://travis-ci.org/arturocs/crab-iot)

Crab-IoT es un hub de dispositivos IoT que provee una API REST para acceder fácilmente a ellos. A diferencia de las soluciones populares de IoT, el  objetivo de este proyecto es que el usuario tenga total control de sus datos. Para lograr esto se recomienda usar una instalación privada de Crab-IoT, ya que es la única forma de tener una certeza absoluta de que tu privacidad está a salvo. Usando un lenguaje como Rust conseguimos que el servicio de Crab-IoT consuma pocos recursos y sea robusto, haciendo así que sea fácil de instalar en un VPS barato o en un dispositivo local de bajo consumo. 



#### Arquitectura

La arquitectura de Crab-IoT es una arquitectura de eventos, ya que cada dispositivo IoT enviará datos al servicio central dependiendo de los cambios de estado que sufra. Por ejemplo, supongamos el caso de un interruptor conectado:

El interruptor puede ser encendido y apagado manualmente o mediante un mensaje. Si el estado es cambiado de forma manual, entonces el interruptor debe avisar a Crab-IoT de que el estado ha sido cambiado, para que el estado aparezca correctamente cuando sea consultado mediante la API REST.



#### Herramientas

He elegido Rust y su ecosistema porque es un lenguaje moderno y rápido con el que estoy familiarizado.  Para compilar este proyecto es necesario instalar cargo y la versión 1.47 de rust.

#### Instrucciones

* Para comprobar que el proyecto compila correctamente basta ejecutar `make check`.
* Para correr los test hay que ejecutar `make test`.
* Para correr los microbenchmarks test hay que ejecutar `make benchmark`.



#### Módulos creados

* [device](https://github.com/arturocs/crab-iot/blob/master/device/src/lib.rs)

  * [rdevice](https://github.com/arturocs/crab-iot/blob/master/device/src/rdevice.rs)
  * [rwdevice](https://github.com/arturocs/crab-iot/blob/master/device/src/rwdevice.rs)
  * [device_list](https://github.com/arturocs/crab-iot/blob/master/device/src/device_list.rs)
  * [plugin](https://github.com/arturocs/crab-iot/blob/master/device/src/plugin.rs)
  * [error](https://github.com/arturocs/crab-iot/blob/master/device/src/error.rs)

* [tests](https://github.com/arturocs/crab-iot/blob/master/tests/lib.rs)

* [benchs](https://github.com/arturocs/crab-iot/blob/master/bench/src/bench.rs)
* [fake_plugin](https://github.com/arturocs/crab-iot/tree/master/fake_plugin)
  

#### Dependencias

Para la base de datos he optado por usar [rusqlite](https://github.com/rusqlite/rusqlite), una interfaz en Rust para sqlite. A pesar de que sqlite no escala bien con el numero de peticiones, su consumo de recursos es bastante limitado. Además, ya que Crab-IoT está orientado a instalaciones privadas, no se espera que  el número de usuarios simultáneos sea muy alto.

Para la API REST he optado por la librería [warp](https://github.com/seanmonstar/warp), una librería bastante rápida y moderna que permite implementar API REST con relativa facilidad.



[Documentacion de los objetivos](https://github.com/arturocs/proyecto-CC/blob/master/docs/configuracion.md)

[Roadmap](https://github.com/arturocs/crab-iot/blob/master/docs/roadmap.md)

[Justificación de las herramientas elegidas](https://github.com/arturocs/crab-iot/blob/master/docs/justificacion_herramientas.md)