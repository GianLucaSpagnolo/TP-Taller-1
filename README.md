# Taller de Programacion

## Grupo "La Krupoviesa"

* Rafael Ortegano - 108313
* Martin Gonzalez Prieto - 105738
* Gian Luca Spagnolo - 108072
* Alexis Martin Ramos - 98891

## Intalaciones

Antes que nada se debe cambiar a modo ejecución los archivos .sh :

    chmod +x data/scripts/*.sh

### Instalación de dependencias

Para instalar las dependencias necesarias *step* y *libssl-dev* se debe ejecutar:

    ./data/scripts/install_reqs.sh

### Certificados

#### Instalacion de certificado raiz

Cuando los certificados ya existen (en data/certificates), *solo* se debe instalar el certificado raiz en el S.O. corriendo el comando:

    ./data/scripts/install_ca.sh    

#### Actualizacion de certificados

Para actualizar/crear los certificados, se deben borrar los certificados de la carpeta *data/certificates* y ejecutar:

    ./data/scripts/load_certs.sh

## Como usar

### Message Broker

Primero se debe levantar el message broker ejecutando:

    cargo r --bin broker

### Monitoring App

Luego se debe instanciar las sesiones de cada cliente por lo que se recomienda que primeramente inicie la aplicación de monitoreo:

    cargo r --bin monitoring_app

Cabe destacar que una vez se creen las sesiones para todos los clientes, el broker almacena dicha información y por lo tanto, luego de ello, cada sistema puede comportarse independientemente de que aplicación esté corriendo en ese momento.

### Sistema de cámaras y Software de drone

Posteriormente se pueden iniciar cualquiera de los demás sistemas:

    cargo r --bin cams_system
    cargo r --bin drone -- [config path]

El *[config path]* para el caso de software del drone determina la instancia del mismo

### Configuraciones

Se distinguen dos tipos de configuraciones: configuración de sistema y configuración de mqtt.

#### Configuraciones de sistema

Este tipo de configuraciones determina el funcionamiento de cada apliación. A excepción de los drones, cada aplicación cuenta una configuración por default, la cual se pude modificar.

* *Monitoring App*: monitoring_app/config/app_config.txt

* *Central Cams System*: central_cams_system/config/system_config.txt

Los drones tambien tienen configuraciones por default pero se pasan por parámetro, se cuenta con las siguiente configuraciones de prueba:

* drone_app/config/drone_config_1.txt
* drone_app/config/drone_config_2.txt
* drone_app/config/drone_config_3.txt

Además de las configuraciones propias de la aplicación, todas cuentan con dos paths importantes:

* *Archivo de persistencia* (.db)
* *Archivo de configuración mqtt* (mqtt_config)

#### Configuración mqtt

Como se menciona anteriormente, las aplicaciones tienen en su configuración de sistema, la dirección del archivo de configuración mqtt. Este archivo setea las características importantes a la hora de establecer una conexión con el protocolo.

##### Client

La configuración del cliente tiene como configuraciones más importantes:

* *id*: id del cliente  (ej. camssystem)
* *ip*: ip del conexión  (ej. 127.0.0.1)
* *port*: puerto de conexión    (ej. 5000)
* *log_path*: archivo para loggear el protocolo (ej. data/logs/cams_log.csv)
* *log_in_terminal*: true/false si desea o no que el logger se muestre por terminal

##### Server

La configuración del servidor tiene como configuraciones más importantes:

* *id*: id del cliente  (ej. broker)
* *ip*: ip del conexión  (ej. 127.0.0.1)
* *port*: puerto de conexión    (ej. 5000)
* *log_path*: archivo para loggear el protocolo (ej. data/logs/broker_log.csv)
* *log_in_terminal*: true/false si desea o no que el logger se muestre por terminal.
* *db_path*: archivo dónde se quiere serializar la información de las sesiones (ej. data/db/broker_sessions.db)

## Como testear

    cargo test

## Notas

Cuando se necesite agregar manualmente cámaras mediante el sistema central, se recomiendan estas posiciones para la Ciudad Autónoma de Buenos aires:

    (cmd; lat; lon )
    add;-34.581568266649754;-58.4744644927824
    add;-34.631345851866776;-58.41585822580699
    add;-34.61863371802939;-58.45012545762901
    add;-34.58153624609583;-58.42089675544147
    add;-34.608203436360505;-58.37366305468922
