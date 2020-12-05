# Crab-IoT 🦀

[![Build Status](https://travis-ci.org/arturocs/crab-iot.svg?branch=master)](https://travis-ci.org/arturocs/crab-iot)

Crab-IoT es un hub de dispositivos IoT que provee una API REST para acceder fácilmente a ellos. A diferencia de las soluciones populares de IoT, el  objetivo de este proyecto es que el usuario tenga total control de sus datos. Para lograr esto se recomienda usar una instalación privada de Crab-IoT, ya que es la única forma de tener una certeza absoluta de que tu privacidad está a salvo. Usando un lenguaje como Rust conseguimos que el servicio de Crab-IoT consuma pocos recursos y sea robusto, haciendo así que sea fácil de instalar en un VPS barato o en un dispositivo local de bajo consumo. 



#### Instrucciones

* Para comprobar que el proyecto compila correctamente basta ejecutar `make check`.

* Para correr los test hay que ejecutar `make test`.

* Para correr los microbenchmarks test hay que ejecutar `make benchmark`.

  

#### Avance en el proyecto

* Se refactorizado el código para aprovechar mejor el sistema de traits de rust, #41
* Se ha diseñado un nuevo plugin que devuelve la información climática extraída de [este json](https://github.com/arturocs/crab-iot/blob/master/weather_fake_plugin/1769_es.json), #42
* Se ha comezado a diseñar un sistema de plugins alternativo mas eficiente, #43



[Documentacion de los objetivos](https://github.com/arturocs/proyecto-CC/blob/master/docs/configuracion.md)

[Roadmap](https://github.com/arturocs/crab-iot/blob/master/docs/roadmap.md)

[Justificación de las herramientas elegidas](https://github.com/arturocs/crab-iot/blob/master/docs/justificacion_herramientas.md)

[Documentación sobre docker](./docs/documentación_docker.md)