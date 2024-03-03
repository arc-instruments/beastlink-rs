#!/bin/bash

# Docker/Podman build script for RHEL-based packages
# Run it like this
# sudo docker run -v $(pwd):/io --rm DISTRIBUTION /io/build.sh
# DISTRIBUTION is recommended to be one of the below
# almalinux:8-minimal
# almalinux:9-minimal

if [ ! -f '/run/.containerenv' ]; then
  if [ ! -f '/.dockerenv' ]; then
    echo >&2 'This script is only meant for container environments'
    exit 1
  fi
fi

ARCH=x86_64

microdnf install -y gcc-c++ libstdc++-devel rpmdevtools rpmlint

cd /root
rpmdev-setuptree
cp /io/cesys-udk-lite.spec /io/beastlink-free.spec rpmbuild/SPECS
spectool -g -R rpmbuild/SPECS/cesys-udk-lite.spec
spectool -g -R rpmbuild/SPECS/beastlink-free.spec

env QA_RPATHS=0x0013 rpmbuild -bb rpmbuild/SPECS/cesys-udk-lite.spec
env QA_RPATHS=0x0013 rpmbuild -bb rpmbuild/SPECS/beastlink-free.spec

cp rpmbuild/RPMS/${ARCH}/*.rpm /io
