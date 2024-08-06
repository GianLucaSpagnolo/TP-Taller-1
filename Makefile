# Certificados
chmod:
	chmod +x data/scripts/*.sh

reqs:
	./data/scripts/install_reqs.sh

load:
	./data/scripts/load_certs.sh

install:
	./data/scripts/install_ca.sh

clean:
	rm data/certificates/*.crt data/certificates/*.key data/certificates/*.der data/certificates/*.pfx 
