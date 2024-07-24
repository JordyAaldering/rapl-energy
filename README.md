# Rapl Energy

Small library for getting the CPU energy consumption from RAPL and friends.

This project is very much still a work in progress, and is mainly intended for internal use.
However I share it here should it be useful to anyone.

## RAPL permissions

Reading RAPL requires elevated permissions.

I suggest adding a new `rapl` group.

```bash
sudo addgroup rapl
sudo usermod -aG rapl $(whoami)
```

And then adding the necessary entries to `/etc/sysfs.conf`.
(Requires `sysfsutils` to be installed.)

```bash
mode class/powercap/intel-rapl:0/energy_uj = 0440
owner class/powercap/intel-rapl:0/energy_uj = root:rapl
```

```bash
mode class/powercap/intel-rapl:0:0/energy_uj = 0440
owner class/powercap/intel-rapl:0:0/energy_uj = root:rapl
```

## MSR permissions

Reading model-specific registers (MSR) requires elevated permissions.

```bash
sudo apt install msr-tools
```

You might need to run modprobe as well.

```bash
modprobe msr
```

One can then print the accumulated energy value as follows.
(Where `-a` prints all CPUs, and `-u` prints the value as an unsigned decimal.)

```bash
sudo rdmsr 0xC001029A -au
```

It seems that the executable must be run with sudo though.
I still need to figure out if perhaps this can be done instead with a group.

```bash
sudo ./target/debug/examples/amd
```

## MSR group

It might be useful to create a group for MSR.
Sadly this does not resolve the above issue however.

```bash
sudo groupadd msr
sudo chgrp msr /dev/cpu/*/msr
sudo chmod g+rw /dev/cpu/*/msr
sudo usermod -aG msr $(whoami)
newgrp msr
```
