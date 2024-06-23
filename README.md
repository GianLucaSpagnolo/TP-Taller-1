# Taller de Programacion

## Grupo
* 
* 
* 
* 

## Instalacion de dependencias
Para instalar las dependencias necesarias *step* y *libssl-dev* se debe ejecutar:

    cd data/scripts
    ./install_reqs.sh

### Certificados

#### Instalacion de certificado raiz
Cuando los certificados ya existen (en data/certificates), *solo* se debe instalar el certificado raiz en el S.O. corriendo el comando:

    cd data/scripts
    ./install_ca.sh    

#### Actualizacion de certificados
Para actualizar/crear los certificados, se deben borrar los certificados de la carpeta *data/certificates* y ejecutar:

    cd data/scripts
    ./load_certs.sh

## Como usar

    cargo r --bin broker
    cargo r --bin monitoring_app
    cargo r --bin cams_system

## Como testear

    cargo test