# Roadmap

La planificación de este proyecto se divide en cinco milestones, que se desarrollarán en el orden aquí expuesto, a excepción del ultimo que incluye mejoras varias que se irán implementando a lo largo del desarrollo.



### 1. API Rest

Al finalizar este milestone la API rest para que cualquier frontend pueda interactuar con Crab-IoT estará lista. No obstante la funcionalidad de la API en ese momento será un tanto limitada ya que la comunicación con los dispositivos IoT aun no estará lista. [Enlace al milestone correspondiente en github](https://github.com/arturocs/crab-iot/milestone/4)

**Historias de usuario relacionadas**

* [HU - Elegir librería para la API rest](https://github.com/arturocs/crab-iot/issues/15)

* [HU - Implementacion inicial de la API rest](https://github.com/arturocs/crab-iot/issues/23)

* [HU - Enlazar dispositivo con Crab-IoT](https://github.com/arturocs/crab-iot/issues/24)

* [HU - Buscar dispositivos en red local](https://github.com/arturocs/crab-iot/issues/10)



### 2. Almacenamiento permanente

Durante este milestone se va a implementar el sistema de bases de datos, gracias al cual crab-iot tendrá almacenamiento persistente. Para poder iniciar la implementación de este milestone es necesario que ya haya implementado al menos un prototipo de la API rest. [Enlace al milestone correspondiente en github](https://github.com/arturocs/crab-iot/milestone/7)

**Historias de usuario relacionadas**

* [HU - Elegir sistema de base de datos](https://github.com/arturocs/crab-iot/issues/12)

* [HU - Implementación inicial de la base de datos](https://github.com/arturocs/crab-iot/issues/22)

* [HU - Listar dispositivos enlazados](https://github.com/arturocs/crab-iot/issues/14)




### 3. Sistema de plugins

En este milestone se va a desarrollar un sistema de plugins. Dichos plugins podrán ser descargados desde crab-iot cada vez que se vaya a añadir un dispositivo nuevo al hogar. [Enlace al milestone correspondiente en github](https://github.com/arturocs/crab-iot/milestone/5)

**Historias de usuario relacionadas**

* [HU - Sistema de plugins para dispositivos](https://github.com/arturocs/crab-iot/issues/6)           

* [HU - Consultar el estado de un dispositivo](https://github.com/arturocs/crab-iot/issues/1)             

* [HU - Cambiar estado de un dispositivo](https://github.com/arturocs/crab-iot/issues/2)

 

### 4. Automatización de tareas

Durante este milestone incluye las  se desarrollará un sistema de automatización de tareas para los dispositivos IoT, ya sea mediante un temporizador de acciones o basado en eventos. [Enlace al milestone correspondiente en github](https://github.com/arturocs/crab-iot/milestone/6)

**Historias de usuario relacionadas**

* [HU - Temporizadores para cambios de estado](https://github.com/arturocs/crab-iot/issues/4)

* [HU - Automatizar acciones](https://github.com/arturocs/crab-iot/issues/5)



---



### Mejoras secundarias

Este milestone cubre tareas que si bien no son estrictamente necesarias, mejoran la calidad del proyecto y/o la de su desarrollo. Por tanto estas mejoras serán implementadas conforme el resto milestones vayan avanzando. [Enlace al milestone correspondiente en github](https://github.com/arturocs/crab-iot/milestone/2)

**Historias de usuario relacionadas**

* [HU - Tipo de dato Error](https://github.com/arturocs/crab-iot/issues/9)

* [HU - Microbenchmarks](https://github.com/arturocs/crab-iot/issues/7)                        

* [HU - Minimizar overhead de conexión](https://github.com/arturocs/crab-iot/issues/8)

* [HU - Añadir soporte para conexión cifrada](https://github.com/arturocs/crab-iot/issues/3)

  