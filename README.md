# Rapl Energy

Small library for getting the CPU energy consumption from RAPL

This project is very much still a work in progress, and is mainly intended for internal use.
However I share it here should it be useful to anyone.

The library is simple enough however that it might be better to just copy-past it...
Likely there will still be many breaking changes, so be warned.

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
sudo modprobe 0xC001029A -au
```

It seems that the executable must be run with sudo though.
I still need to figure out if perhaps this can be done instead with a group.

```bash
sudo ./target/debug/examples/amd
```
