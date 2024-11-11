#!/bin/bash -eu

installed_path="$HOME/.local/bin/dt"

if [[ ! -e $installed_path ]]; then
	echo "dt is not installed at $installed_path"
	exit 2
fi

version=$(dt --version)
sudo rm $installed_path && echo -e "$version successfully uninstalled"
