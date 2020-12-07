# Este dockerfile está basado en https://github.com/arturocs/crab-iot/blob/master/Dockerfile, 
# Usar Ubuntu 18.04 LTS como imagen base, ya que su tamaño es inferior a la de Ubuntu 20.04, aun
# tiene muchos años de soporte y para este caso particular usar la ultima LTS no aporta ninguna ventaja
FROM ubuntu:18.04

# Primero se fijan las variables de entorno necesarias para rustup
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.48.0

# 1. Instalar las dependencias de rust y los paquetes necesarios para descargar rustup.
# 2. Descargar rustup para GNU/Linux x86_64, darle permisos de ejecución y ejecutarlo
#    con los argumentos necesarios para instalar la versión 1.48.0 de rustc, rust-std y
#    cargo para GNU/Linux x86_64. Además en lugar de añadir los ejecutables al path, se 
#    utilizarán los directorios especificados mediante las variables de entorno
# 3. Eliminar el instalador de rustup y darle permisos de escritura a todos los usuarios
#    en los directorios de rustup u cargo
# 4. Eliminar paquetes innecesarios y sus dependencias
# 5. Eliminar datos de los paquetes, ya que ocupan bastante y no son necesarios para la imagen
# 6. Crear un usuario sin privilegios

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    wget \
    make \
    ; \
    wget "https://static.rust-lang.org/rustup/archive/1.22.1/x86_64-unknown-linux-gnu/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host x86_64-unknown-linux-gnu; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    apt-get remove -y --auto-remove \
    wget ca-certificates\
    ; \
    rm -rf /var/lib/apt/lists/*; \
    useradd crabiot; 


# Fijar el directorio de trabajo en donde se va a montar el repositorio
WORKDIR /app/test

# Cambiar al usuario sin privilegios
# USER crabiot

# Cuando se inicie el contenedor ejecutamos los siguientes pasos:
# 1. Cambiamos la propiedad de /app/test al usuario sin privilegios
# 2. Damos permisos de escritura lectura y ejecución sobre la carpeta /app/test
# 3. Ejcutamos make test cuando
#CMD chown crabiot /app/test &&  chmod 777 /app/test && make test
CMD chown crabiot /app/test &&  chmod 777 /app/test && make test


