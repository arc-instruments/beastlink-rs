Name: beastlink-free
Version: 1.0
Release: 1
Summary: Beastlink EFM03 library

License: CPL-1.0
URL: https://wwww.cesys.com
Source0: https://www.cesys.com/fileadmin/user_upload/service/FPGA/fpga%20boards%20%26%20modules/BeastLink/beastlink-1.0-linux-free.tar.bz2

BuildRequires: gcc-c++ libstdc++-devel
Requires: libusb1 udev cesys-udk-lite

%description
CESYS Beastlink library for interfacing with EFM03 boards

%changelog
* Wed Feb 17 2021 Spyros Stathopoulos <spyros@arc-instruments.co.uk> 1.5.1-1
- Initial version

%prep
tar xvfj %{SOURCE0}

%build
cd %{_builddir}/beastlink-%{version}-linux-free/api/c++
g++ -std=c++11 -fPIC -O2 -shared -I. -I./include -L../../runtime -lbeastlink-%{version} beastlink++.cpp -o libbeastlink++.so
g++ -std=c++11 -O2 -I. -I./include -c beastlink++.cpp -o beastlink++.o
ar rcs libbeastlink++.a beastlink++.o
cd %{_builddir}

%install
mkdir -p %{buildroot}
mkdir -p %{buildroot}/opt/cesys-udk/beastlink-tools
mkdir -p %{buildroot}/%{_libdir}
install -D -m755 beastlink-%{version}-linux-free/runtime/libbeastlink-%{version}.so %{buildroot}/%{_libdir}/libbeastlink-%{version}.so
install -D -m755 beastlink-%{version}-linux-free/api/c++/libbeastlink++.so %{buildroot}/%{_libdir}/libbeastlink++.so
install -D -m644 beastlink-%{version}-linux-free/api/c++/libbeastlink++.a %{buildroot}/%{_libdir}/libbeastlink++.a
install -D -m644 beastlink-%{version}-linux-free/runtime/beastlink_exdes_top.bin %{buildroot}/opt/cesys-udk/beastlink-tools/beastlink_exdes_top.bin

%clean
rm -rf %{buildroot}
rm -rf %{_builddir}/beastlink-1.0-linux-free

%files
%{_libdir}/libbeastlink++.a
%{_libdir}/libbeastlink++.so
%{_libdir}/libbeastlink-1.0.so
/opt/cesys-udk/beastlink-tools/beastlink_exdes_top.bin
