#!/bin/bash

echo "installing native_tls req:"
sudo apt install pkg-config libssl-dev libgtk-3-dev -y

echo "installing step: "
wget https://dl.smallstep.com/cli/docs-cli-install/latest/step-cli_amd64.deb
sudo dpkg -i step-cli_amd64.deb
rm step-cli_amd64.deb


