beastlink-rs
============

Rust binding for the CESYS Beastlink library. Requires Rust ≥ 1.40.

This library provides Rust bindings for the CESYS beastlink FPGA library used to
interface with the [EFM03](https://www.cesys.com/en/our-products/fpga-boards/efm-03.html) board.

### Disclaimer
**Please note**: This library links against the free version of the beastlink
library which implies that it **can only be used with EFM03 board**. If you
want to interface custom designs based on the same FPGA chip you will need an
explicit license agreement from CESYS. The authors of `beastlink-rs` will not
be held liable for improper use of this library.

###  Prerequisites
To build this you need to have the beastlink shared libraries available for
[Windows](https://www.cesys.com/fileadmin/user_upload/service/FPGA/fpga%20boards%20%26%20modules/BeastLink/beastlink-1.0-windows-free.zip)
(x86 or x86_64) or
[Linux](https://www.cesys.com/fileadmin/user_upload/service/FPGA/fpga%20boards%20%26%20modules/BeastLink/beastlink-1.0-linux-free.tar.bz2)
(x86_64 only). These are found under  the `runtime` folder on either of these
distributions.  If you are building on Windows you will also need the stub
`lib` files for compiler to properly find the exported symbols. These can be
found under `api/c++/lib` on the Windows one. All of them must be accessible
from the compiler (so they must be in `%PATH%` on Windows or `$LIBRARY_PATH` on
Linux).

### Connecting to an EFM03 board

Beastlink alone is not enough to get you connected to an EFM03 board. You will
also to have the UDK USB drivers. These can be found under the `driver` folder.
On Windows installing `udk3usb-drivers-windows` should be enough. On Linux the
UDK libraries (`libudk3` and `libudk3mod-*-libusb`), firmware (under `data`) as
well as the firmware flashing utility, `cuudfdl`, all found in the
`udk3-service-linux` archive must be installed. The firmware flashing utility
must be placed alongside the `data` folder for it to work properly. If you want
to automatically upload the device firmware when you connect the board you will
also need some udev rules to do that. For instance...

```
ACTION=="add", SUBSYSTEMS=="usb", ATTRS{idVendor}=="10f8", ATTRS{idProduct}=="c4??", MODE="660", GROUP="plugdev", RUN+="/bin/bash /path/to/cuudfdl.sh"
ACTION=="add", SUBSYSTEMS=="usb", ATTRS{idVendor}=="10f8", ATTRS{idProduct}=="c5??", MODE="660", GROUP="plugdev", RUN+="/bin/bash /path/to/cuudfdl.sh"
```

`cuudfdl.sh` is a wrapper script that runs the `cuudfdl` executable with the
appropriate arguments and must be placed alongside the main `cuudfdl`
executable and the `data` folder containing the firmware as outlined above. The
contents are simply

```
#!/bin/bash
cd /path/to/data_and_cuudfld && ./cuudfdl $@
```

If you are using archlinux `PKGBUILD`s are available under the
`contrib/archlinux` folder of this repository. If you are on a debian-based
distribution you can build packages for your distribution using the scripts
provided in `contrib/debian`.

### Examples
You can check the tests for simple usecases for this library. In general
beastlink only provices a handful of operations to interface with the EFM03,
namely read and write from/to block memory, read and write from/to registers,
load a bitstream on the board and some firmware information.

**Device enumeration**

```rust
use beastlink;

const VID: u16 = 0x10f8;
const PID: u16 = 0xc583;

fn main() {
        match beastlink::enumerate(VID, PID) {
            Ok(count) => {
                println!("Enumeration result: {}", count);
            },
            Err(err) => {
                panic!("Enumeration failed with {}", err)
            }
        }
}
```

**Open devices**

```rust
use beastlink;

const VID: u16 = 0x10f8;
const PID: u16 = 0xc583;

fn main() {
    let count = match beastlink::enumerate(VID, PID) {
        Ok(count) => {
            println!("Enumerated {} devices", count);
            count
        },
        Err(err) => {
            panic!("Enumeration failed with {}", err)
        }
    };

    // Try to open a device
    for enum_id in 0..count {
        println!("Trying device {}", enum_id);
        let dev = match beastlink::Device::open(enum_id) {
            Ok(dev) => dev,
            Err(err) => {
                panic!("Error while opening device: {}", err)
            }
        };

        println!("Opened device {:?}", dev);

        match dev.close() {
            Err(err) => {
                panic!("Error while closing device {:?}: {}", dev, err)
            },
            _ => {}
        }
    }

}
```
