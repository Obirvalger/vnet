# vnet
It is a program to set up networks for virtual machines. Under the hood
vnet uses ip utility. To work properly the program executable should be
suid or has CAP_NET_ADMIN capability.

## Build and setup
All needed dependencies seved into vendor directory, so it can be built
in the offline mode.
```
cargo build --release --offline
```

Then copy created executable to appropriate path, e.g. ~/bin/vnet, if
~/bin is in your PATH.
```
cp target/release/vnet ~/bin/vnet
```

Finally add capability to the executable.
```
sudo setcap CAP_NET_ADMIN+p ~/bin/vnet
```

To get competions (e.g. zsh) run:
```
vnet completion zsh
```

## Run
For now it only could create or remove tap devices. To the passed name
vnet adds "vnet_" prefix to not mix with other device names and to avoid
accidental changing of the system ones.

Example of tap creation:
```
vnet tap create tap0
```
Creates tap namede vnet_tap0. Prints new name if tap created. Prints
nothing if tab with the name already present.
