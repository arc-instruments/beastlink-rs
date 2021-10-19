#!/bin/bash

# Docker/Podman build script for Debian/Ubuntu packages
# Run it like this
# sudo docker run -v $(pwd):/io --rm DISTRIBUTION /io/build.sh
# DISTRIBUTION can be any valid debian or ubuntu equal or newer
# to 9 and 18.04 respectively. Some tags that have been tested
# ubuntu:focal
# ubuntu:bionic
# debian:buster-slim
# debian:bullseye-slim

if [ ! -f '/run/.containerenv' ]; then
  if [ ! -f '/.dockerenv' ]; then
    echo >&2 'This script is only meant for container environments'
    exit 1
  fi
fi

apt update
apt upgrade

apt install -y wget fakeroot libusb-1.0-0-dev build-essential

cd /io
/io/make-debian-packages.sh
