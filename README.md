# Rapl Energy

Small library for getting the CPU energy consumption from RAPL

This project is very much still a work in progress, and is mainly intended for internal use.
However I share it here should it be useful to anyone.

The library is simple enough however that it might be better to just copy-past it...
Likely there will still be many breaking changes, so be warned.

## RAPL permissions

Reading RAPL requires elevated permissions.

I suggest adding a new `rapl` group.

```
sudo addgroup rapl
sudo usermod -aG rapl $(whoami)
```

And then adding the necessary entries to `/etc/sysfs.conf`.
(Requires `sysfsutils` to be installed.)

```
mode class/powercap/intel-rapl:0/energy_uj = 0440
owner class/powercap/intel-rapl:0/energy_uj = root:rapl
```

```
mode class/powercap/intel-rapl:0:0/energy_uj = 0440
owner class/powercap/intel-rapl:0:0/energy_uj = root:rapl
```
