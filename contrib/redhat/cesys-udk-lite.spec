Name: cesys-udk-lite
Version: 1.5.1
Release: 1
Summary: CESYS UDK Libraries

License: CPL-1.0
URL: https://wwww.cesys.com
Source0: https://www.cesys.com/fileadmin/user_upload/service/FPGA/fpga%20boards%20%26%20modules/BeastLink/beastlink-1.0-linux-free.tar.bz2

Requires: libusb1 udev

%description
CESYS UDK SDK libraries (publicly available API).

%changelog
* Wed Feb 17 2021 Spyros Stathopoulos <spyros@arc-instruments.co.uk> 1.5.1-1
- Initial version

%prep
tar xvfj %{SOURCE0}
tar xvfj beastlink-1.0-linux-free/driver/udk3-service-linux-%{version}-x86_64.tar.bz2
%{__cat} <<'EOF' > 99-cesys-udk.rules
ACTION=="add", SUBSYSTEMS=="usb", ATTRS{idVendor}=="10f8", ATTRS{idProduct}=="c4??", MODE="660", GROUP="plugdev", RUN+="/bin/bash /opt/cesys-udk/udk3-tools/cuudfdl.sh"
ACTION=="add", SUBSYSTEMS=="usb", ATTRS{idVendor}=="10f8", ATTRS{idProduct}=="c5??", MODE="660", GROUP="plugdev", RUN+="/bin/bash /opt/cesys-udk/udk3-tools/cuudfdl.sh"
EOF

%{__cat} <<'EOF' > cuudfdl.sh
#!/bin/bash
cd /opt/cesys-udk/udk3-tools && ./cuudfdl $@
EOF

%build

%install
mkdir -p %{buildroot}
mkdir -p %{buildroot}/%{_libdir}
mkdir -p %{buildroot}/usr/lib/udev/rules.d
mkdir -p %{buildroot}/opt/cesys-udk/udk3-tools/data
echo '' > %{buildroot}/opt/cesys-udk/udk3-tools/service.log
install -D -m755 udk3-service-linux-%{version}-x86_64/cuudfdl %{buildroot}/opt/cesys-udk/udk3-tools/cuudfdl
install -D -m644 udk3-service-linux-%{version}-x86_64/data/config.xml %{buildroot}/opt/cesys-udk/udk3-tools/data/config.xml
install -D -m644 udk3-service-linux-%{version}-x86_64/data/service.xml %{buildroot}/opt/cesys-udk/udk3-tools/data/service.xml
install -D -m644 udk3-service-linux-%{version}-x86_64/data/fx2-apo-v2.0.bix %{buildroot}/opt/cesys-udk/udk3-tools/data/fx2-apo-v2.0.bix
install -D -m644 udk3-service-linux-%{version}-x86_64/data/fx2-v2.0.bix %{buildroot}/opt/cesys-udk/udk3-tools/data/fx2-v2.0.bix
install -D -m644 udk3-service-linux-%{version}-x86_64/data/fx3-v1.3.img %{buildroot}/opt/cesys-udk/udk3-tools/data/fx3-v1.3.img
install -D -m755 udk3-service-linux-%{version}-x86_64/libudk3-%{version}.so %{buildroot}/%{_libdir}/libudk3-%{version}.so
install -D -m755 udk3-service-linux-%{version}-x86_64/libudk3mod-%{version}-libusb.so %{buildroot}/%{_libdir}/libudk3mod-%{version}-libusb.so
install -D -m644 99-cesys-udk.rules %{buildroot}/usr/lib/udev/rules.d/99-cesys-udk.rules
install -D -m755 cuudfdl.sh %{buildroot}/opt/cesys-udk/udk3-tools/cuudfdl.sh
ln -s %{_libdir}/libudk3mod-%{version}-libusb.so %{buildroot}/opt/cesys-udk/udk3-tools/libudk3mod-%{version}-libusb.so

%clean
rm -rf %{buildroot}
rm -rf %{_builddir}/beastlink-1.0-linux-free
rm -rf %{_builddir}/udk3-service-linux-%{version}-x86_64

%files
/opt/cesys-udk/udk3-tools/cuudfdl
/opt/cesys-udk/udk3-tools/cuudfdl.sh
/opt/cesys-udk/udk3-tools/data/
/opt/cesys-udk/udk3-tools/data/config.xml
/opt/cesys-udk/udk3-tools/data/fx2-apo-v2.0.bix
/opt/cesys-udk/udk3-tools/data/fx2-v2.0.bix
/opt/cesys-udk/udk3-tools/data/fx3-v1.3.img
/opt/cesys-udk/udk3-tools/data/service.xml
/opt/cesys-udk/udk3-tools/libudk3mod-1.5.1-libusb.so
/opt/cesys-udk/udk3-tools/service.log
%{_libdir}/libudk3-1.5.1.so
%{_libdir}/libudk3mod-1.5.1-libusb.so
/usr/lib/udev/rules.d/99-cesys-udk.rules

