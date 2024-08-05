clean:
	sudo rm data/certificates/*.crt data/certificates/*.key data/certificates/*.crt data/certificates/*.der data/certificates/*.pfx 

reqs:
	sudo ./data/scripts/install_reqs.sh

install:
	sudo ./data/scripts/load_certs.sh
	sudo ./data/scripts/install_ca.sh