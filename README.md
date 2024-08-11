# Taller de Programación

## Grupo "La Krupoviesa"

* Rafael Ortegano - **108313**
* Martin Gonzalez Prieto - **105738**
* Gian Luca Spagnolo - **108072**
* Alexis Martin Ramos - **98891**

---

## Instalaciones

Todos estos comandos se deben realizar en el directorio root del proyecto.

Antes que nada se debe cambiar a modo ejecución los archivos **.sh**:

    make chmod

### Instalación de dependencias

Para instalar las dependencias necesarias *step* y *libssl-dev* se debe ejecutar:

    make reqs

### Certificados

#### Actualizacion de certificados

Para **actualizar/crear** los certificados, se deben borrar los certificados de la carpeta *data/certificates* y ejecutar:

    make load

#### Instalacion de certificado raiz

Cuando los certificados ya existen (en data/certificates), **solo** se debe instalar el certificado raiz en el S.O. corriendo el comando:

    make install

### Eliminación de certificados

Asimismo, se cuenta con el siguiente comando, el cual elimina automaticamente todos los certificados cargados previamente (en caso de querer iniciarlos denuevo), para una mayor conveniencia:

    make clean

---

## Como usar

### Comandos importantes

Para compilar el proyecto

    cargo build --verbose

Para ejecutar el `cargo clippy`

    cargo linter

Para ejecutar todos los tests

    cargo test --verbose

### Message Broker

Primero se debe levantar el message broker ejecutando:

    cargo broker

### Monitoring App

Luego se debe instanciar las sesiones de cada cliente por lo que se recomienda que primeramente inicie la aplicación de monitoreo:

    cargo monitoring_app

Cabe destacar que una vez se creen las sesiones para todos los clientes, el broker almacena dicha información y por lo tanto, luego de ello, cada sistema puede comportarse independientemente de que aplicación esté corriendo en ese momento.

### Sistema de cámaras

Posteriormente se pueden iniciar el sistema de camaras con el siguiente comando:

    cargo cams_system

### Software de drones

Por último, se puede iniciar instancias de un dron correspondiente, con sus respectivos archivos de configuración y sus correspondientes caracteristicas de la siguiente forma:

    cargo droneN

Siendo N el número de una instancia de drone correspondiente. Por ejemplo:

    cargo drone1

Cabe destacar que, en este ejemplo, se esta ejecutando la instancia del **drone1**, pero existen 7 diferentes instancias de drones hasta **drone7**, las cuales se pueden correr en paralelo.

### Limpieza de los archivos de base de datos

En caso que se haya ejecutado completamente el proyecto, y se desee eliminar tanto los archivos de persistencia correspondientes a las aplicaciones y al broker (en caso de que se quiera iniciar el proyecto de cero), o se busque borrar aquellas carpetas temporales que corresponden a cada camara, donde se colocan los incidentes potenciales que recibe en el directorio *camera_videos* (en caso de querer iniciar nuevamente de cero el sistema de camaras) se cuenta con el siguiente comando:

    make reset

---

### Configuraciones

Se distinguen dos tipos de configuraciones: configuración de sistema y configuración de mqtt.

#### Configuraciones de sistema

Este tipo de configuraciones determina el funcionamiento de cada apliación. A excepción de los drones, cada aplicación cuenta una configuración por default, la cual se pude modificar y al momento de ejecutar con el comando `cargo run` no hace falta introducir.

* *Monitoring App*: monitoring_app/config/app_config.txt
* *Central Cams System*: central_cams_system/config/system_config.txt

Los drones tambien tienen configuraciones por default pero, en caso de ejecutar el comando `cargo run` se deben pasan por parámetro para determinar la instancia de drone. Se cuenta con las siguiente configuraciones de prueba:

* drone_app/config/drone_config_1.txt
* drone_app/config/drone_config_2.txt
* drone_app/config/drone_config_3.txt
* drone_app/config/drone_config_4.txt
* drone_app/config/drone_config_5.txt
* drone_app/config/drone_config_6.txt
* drone_app/config/drone_config_7.txt

Ejemplo de comando de inicio de las aplicaciones manualmente:

    cargo run --bin monitoring_app
    cargo run --bin drone drone_app/config/drone_config_1.txt

Además de las configuraciones propias de la aplicación, todas cuentan con dos archivos importantes:

* **.db**: Archivo de persistencia
* **mqtt_config**: Archivo de configuración de protocolo MQTT

#### Configuración MQTT

Como se menciona anteriormente, las aplicaciones tienen en su configuración de sistema, la dirección del archivo de configuración mqtt. Este archivo setea las características importantes a la hora de establecer una conexión con el protocolo.

##### Client

La configuración del cliente tiene como configuraciones más importantes:

* *id*: id del cliente  (ej. camssystem)
* *ip*: ip del conexión  (ej. 127.0.0.1)
* *port*: puerto de conexión    (ej. 5000)
* *log_path*: archivo para loggear el protocolo (ej. data/logs/cams_log.csv)
* *log_in_terminal*: true/false si desea o no que el logger se muestre por terminal

##### Server

La configuración del broker tiene como configuraciones más importantes:

* *id*: id del cliente  (ej. broker)
* *ip*: ip del conexión  (ej. 127.0.0.1)
* *port*: puerto de conexión    (ej. 5000)
* *log_path*: archivo para loggear el protocolo (ej. data/logs/broker_log.csv)
* *log_in_terminal*: true/false si desea o no que el logger se muestre por terminal.
* *db_path*: archivo dónde se quiere serializar la información de las sesiones (ej. data/db/broker_sessions.db)

Ademas, el broker dispone de un archivo *broker_auth_data.txt* el cual posee un registro de aquellas aplicaciones (identificadas por ID) que tienen permitido conectarse al servidor.

## Cámaras recomendadas

Cuando se necesite agregar manualmente cámaras mediante el sistema central, se recomiendan estas posiciones para la Ciudad Autónoma de Buenos aires:

    (cmd; lat; lon)
    add;-34.581568266649754;-58.4744644927824
    add;-34.631345851866776;-58.41585822580699
    add;-34.61863371802939;-58.45012545762901
    add;-34.58153624609583;-58.42089675544147
    add;-34.608203436360505;-58.37366305468922
    add;-34.568241129718864;-58.44865694819334
    add;-34.61727675184148;-58.51268664179685
