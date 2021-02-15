#!/bin/bash

udkver=1.5.1
blver=1.0

if ! command -v wget &> /dev/null
then
  echo "wget is required for this script"
  exit 1
fi

if ! command -v sha1sum &> /dev/null
then
  echo "sha1sum is required for this script"
  exit 1
fi


wget "https://www.cesys.com/fileadmin/user_upload/service/FPGA/fpga%20boards%20%26%20modules/BeastLink/beastlink-${blver}-linux-free.tar.bz2" \
	-O beastlink-${blver}-linux-free.tar.bz2
sha1sum -c "beastlink-${blver}.sha1sum"

if [[ "$?" != "0" ]]; then
	echo "Downloaded file does not match checksum; aborting"
	exit 1;
fi

mkdir -p ./workarea
tar xfj beastlink-${blver}-linux-free.tar.bz2 -C workarea
tar xfj workarea/beastlink-${blver}-linux-free/driver/udk3-service-linux-${udkver}-x86_64.tar.bz2 -C workarea

pushd workarea

# UDK3

mkdir -p ./cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/data
mkdir -p ./cesys-udk-lite_${udkver}-1/usr/lib/x86_64-linux-gnu
mkdir -p ./cesys-udk-lite_${udkver}-1/lib/udev/rules.d
mkdir -p ./cesys-udk-lite_${udkver}-1/usr/share/doc/cesys-udk
mkdir -p ./cesys-udk-lite_${udkver}-1/var/log
mkdir -p ./cesys-udk-lite_${udkver}-1/DEBIAN

install -D -m644 udk3-service-linux-${udkver}-x86_64/license/CSL-1.0.txt \
	cesys-udk-lite_${udkver}-1/usr/share/doc/cesys-udk/CSL-1.0.txt
install -D -m755 udk3-service-linux-${udkver}-x86_64/cuudfdl \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/cuudfdl
install -D -m644 udk3-service-linux-${udkver}-x86_64/data/config.xml \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/data/config.xml
install -D -m644 udk3-service-linux-${udkver}-x86_64/data/service.xml \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/data/service.xml
install -D -m644 udk3-service-linux-${udkver}-x86_64/data/fx2-apo-v2.0.bix \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/data/fx2-apo-v2.0.bix
install -D -m644 udk3-service-linux-${udkver}-x86_64/data/fx2-v2.0.bix \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/data/fx2-v2.0.bix
install -D -m644 udk3-service-linux-${udkver}-x86_64/data/fx3-v1.3.img \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/data/fx3-v1.3.img
install -D -m755 udk3-service-linux-${udkver}-x86_64/libudk3-${udkver}.so \
	cesys-udk-lite_${udkver}-1/usr/lib/x86_64-linux-gnu/libudk3-${udkver}.so
install -D -m755 udk3-service-linux-${udkver}-x86_64/libudk3mod-${udkver}-libusb.so \
	cesys-udk-lite_${udkver}-1/usr/lib/x86_64-linux-gnu/libudk3mod-${udkver}-libusb.so
install -D -m644 ../cesys-udk-lite.control cesys-udk-lite_${udkver}-1/DEBIAN/control
ln -s /usr/lib/x86_64-linux-gnu/libudk3mod-${udkver}-libusb.so \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/libudk3mod-${udkver}-libusb.so
touch cesys-udk-lite_${udkver}-1/var/log/udk-service.log
ln -s /var/log/udk-service.log \
	cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/service.log

cat <<'EOF' > cesys-udk-lite_${udkver}-1/opt/cesys-udk/udk3-tools/cuudfdl.sh
#!/bin/bash
cd /opt/cesys-udk/udk3-tools && ./cuudfdl $@
EOF

cat <<'EOF' > cesys-udk-lite_${udkver}-1/lib/udev/rules.d/99-cesys-udk.rules
ACTION=="add", SUBSYSTEMS=="usb", ATTRS{idVendor}=="10f8", ATTRS{idProduct}=="c4??", MODE="660", GROUP="plugdev", RUN+="/bin/bash /opt/cesys-udk/udk3-tools/cuudfdl.sh"
ACTION=="add", SUBSYSTEMS=="usb", ATTRS{idVendor}=="10f8", ATTRS{idProduct}=="c5??", MODE="660", GROUP="plugdev", RUN+="/bin/bash /opt/cesys-udk/udk3-tools/cuudfdl.sh"
EOF


# beastlink

mkdir -p ./beastlink-free_${blver}-1/usr/lib/x86_64-linux-gnu
mkdir -p ./beastlink-free_${blver}-1/opt/cesys-udk/beastlink-tools
mkdir -p ./beastlink-free_${blver}-1/usr/share/doc/beastlink-free
mkdir -p ./beastlink-free_${blver}-1/DEBIAN

pushd beastlink-${blver}-linux-free/api/c++
g++ -fPIC -O2 -shared -I./include -L../../runtime -lbeastlink-${blver} beastlink++.cpp -o libbeastlink++.so
g++ -O2 -I./include -c beastlink++.cpp -o beastlink++.o
ar rcs libbeastlink++.a beastlink++.o
popd

install -D -m644 udk3-service-linux-${udkver}-x86_64/license/CSL-1.0.txt \
	beastlink-free_${blver}-1/usr/share/doc/beastlink-free/CSL-1.0.txt
install -D -m755 beastlink-${blver}-linux-free/runtime/libbeastlink-${blver}.so \
	beastlink-free_${blver}-1/usr/lib/x86_64-linux-gnu/libbeastlink-1.0.so
install -D -m644 beastlink-${blver}-linux-free/api/c++/libbeastlink++.a \
	beastlink-free_${blver}-1/usr/lib/x86_64-linux-gnu/libbeastlink++.a
install -D -m755 beastlink-${blver}-linux-free/api/c++/libbeastlink++.so \
	beastlink-free_${blver}-1/usr/lib/x86_64-linux-gnu/libbeastlink++.so
install -D -m644 beastlink-${blver}-linux-free/runtime/beastlink_exdes_top.bin \
	beastlink-free_${blver}-1/opt/cesys-udk/beastlink-tools/beastlink_exdes_top.bin

install -D -m644 ../beastlink-free.control beastlink-free_${blver}-1/DEBIAN/control

# beastlink-dev

mkdir -p ./beastlink-free-dev_${blver}-1/usr/include
mkdir -p ./beastlink-free-dev_${blver}-1/DEBIAN

install -D -m755 beastlink-${blver}-linux-free/api/c/include/beastlink.h \
	beastlink-free-dev_${blver}-1/usr/include/beastlink.h
install -D -m755 beastlink-${blver}-linux-free/api/c++/beastlink++.h \
	beastlink-free-dev_${blver}-1/usr/include/beastlink++.h

install -D -m644 ../beastlink-free-dev.control beastlink-free-dev_${blver}-1/DEBIAN/control

popd

dpkg-deb --build workarea/cesys-udk-lite_${udkver}-1
dpkg-deb --build workarea/beastlink-free_${blver}-1
dpkg-deb --build workarea/beastlink-free-dev_${blver}-1

mv workarea/*.deb .
