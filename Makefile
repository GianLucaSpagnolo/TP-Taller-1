# Certificados

# Iniciación de una máquina nueva (servidor o client)
install:
	chmod +x data/scripts/*.sh
	./data/scripts/install_reqs.sh

# Inicialización de certificados en el servidor
init_server:
	chmod +x data/scripts/*.sh
	./data/scripts/load_certs.sh

# Actualización de certificados en el servidor
update_server:
	chmod +x data/scripts/*.sh
	rm -f data/certificates/*.crt data/certificates/*.key data/certificates/*.der data/certificates/*.pfx 
	./data/scripts/load_certs.sh

# Instalación de certificados en el cliente
init_client:
	chmod +x data/scripts/*.sh
	./data/scripts/install_ca.sh

# Limpieza de certificados
clean_certs:
	rm -f data/certificates/*.crt data/certificates/*.key data/certificates/*.der data/certificates/*.pfx

# Limpieza de bases de datos de las aplicaciones
clean:
	rm -f data/db/*.db
	rm -rfv data/camera_videos/*
	touch data/camera_videos/temp
	rmdir -v data/camera_videos/*

clean_db:
	rm -f data/db/*.db

clean_vides:
	rm -rfv data/camera_videos/*
	touch data/camera_videos/temp
	rmdir -v data/camera_videos/*
