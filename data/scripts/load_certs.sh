#!/bin/bash

CERTS_FOLDER="../certificates/"
CA_NAME="ca-native"
SRV_NAME="server"
CERT_CHAIN_NAME="server-cert-chain"
IDENTITY_PFX="identity.pfx"

echo "ingresa pass: 1234"
step certificate create $CA_NAME $CERTS_FOLDER$CA_NAME.crt $CERTS_FOLDER$CA_NAME.key --profile root-ca #pass = 1234

# $SRV_NAME = DNS name, debe ser localhost
echo "ingresa pass: 1234"
step certificate create $SRV_NAME $CERTS_FOLDER$SRV_NAME.crt $CERTS_FOLDER$SRV_NAME.key --ca ./$CERTS_FOLDER$CA_NAME.crt --ca-key ./$CERTS_FOLDER$CA_NAME.key #pass = 1234

echo "verificacion de certificado"
#step certificate inspect ./$SRV_NAME.crt
step certificate verify ./$CERTS_FOLDER$SRV_NAME.crt --roots ./$CERTS_FOLDER$CA_NAME.crt
#step certificate lint ./$SRV_NAME.crt

echo "encadenado de certificados"
step certificate bundle ./$CERTS_FOLDER$SRV_NAME.crt ./$CERTS_FOLDER$CA_NAME.crt $CERTS_FOLDER$CERT_CHAIN_NAME.crt

echo "formateo de certificado a .DER"
step certificate format $CERTS_FOLDER$CA_NAME.crt --out $CERTS_FOLDER$CA_NAME.der

echo "Descifrando CA ..."
step certificate key $CERTS_FOLDER$CA_NAME.crt

echo "Instalando certificado en SO:"
step certificate install $CERTS_FOLDER$CA_NAME.crt

#openssl pkcs12 -export -out identity.pfx-inkey 2key.pem -in 2cert.pem -certfile 2fullchain.pem #ok
echo "formateando cadena de certificados - ingresa 1234: "
openssl pkcs12 -export -out $CERTS_FOLDER$IDENTITY_PFX -inkey $CERTS_FOLDER$SRV_NAME.key -in $CERTS_FOLDER$SRV_NAME.crt -certfile $CERTS_FOLDER$CERT_CHAIN_NAME.crt
