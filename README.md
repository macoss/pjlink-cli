# PJLink CLI

This is a command line interface client for the PJLink protocol that is written in Rust.  PJLink is a network control protocol that has been incorporated into projectors and displays over the last few years.  You can find the protocol specification [here](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=1&cad=rja&uact=8&ved=2ahUKEwj6s-zOkODcAhWEG3wKHbagAloQFjAAegQIABAC&url=https%3A%2F%2Fpjlink.jbmia.or.jp%2Fenglish%2Fdata%2F5-1_PJLink_eng_20131210.pdf&usg=AOvVaw3eWuyry5fcVR1_R-jxrK7J). This utility currently supports both authenticated and open connections and returns the an unparsed response from the device

Testing has been done with Panasonic and Sanyo projectors.

## Usage

```
pjlink-cli [host] [command] [password]
```

## Examples

Check to see if the device is powered on:
  ```
pjlink-cli 192.168.1.1 "POWR ?" password

Responses:
%1POWR=0 # If Off
%1POWR=1 # If On
%1POWR=ERRA # Authencation Error
```

Power on a projector:
```
pjlink-cli 192.168.1.1 "POWR 1" password

Responses:
%1POWR=OK # Successful
%1POWR=ERR2 # Invalid Parameter
%1POWR=ERR3 # Unavailable at this time
%1POWR=ERR4 # Projector/Display failure
```

## Binaries
[PJLink CLI for Mac](https://github.com/macoss/pjlink-cli/releases/download/v0.1.0/pjlink-cli_MacOS.zip)

[PJLink CLI for Windows 64 bit](https://github.com/macoss/pjlink-cli/releases/download/v0.1.0/pjlink-cli_Windows-64bit.zip)

Binaries for other operating system will be coming soon.

## License

Licensed under

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)


### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be licensed as above, without any additional terms or
conditions.
