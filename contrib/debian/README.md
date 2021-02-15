Debian packaging scripts for CESYS UDK
======================================

The supplied scripts will create three barebones packages for cesys-udk and
beastlink (runtime and development). It will download the sources and compile
them into three packages. You need `g++` and `libstdc++` as well as `wget`,
`sha1sum` and `fakeroot` available. You also need `dpkg-deb` but this should be
installed by default. To build the packages run

```
fakeroot ./make-debian-packages.sh
```

The `-dev` package is not required if you want to use the Rust library but it
is provided for completeness.

The scripts should work on Ubuntu ≥ 18.04, Debian buster or sid and
derivatives.  File an issue if they do not.

**Disclaimer**: These packages are provided in the hope they will be useful for
linux users and they are in no way endorsed or supported by CESYS.
