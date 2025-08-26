# RAPL Energy

Reading CPU energy consumption and controlling CPU power limits through RAPL.

## RAPL permissions

Reading RAPL requires elevated permissions.

I suggest adding a new `rapl` group.

```bash
sudo addgroup rapl
sudo usermod -aG rapl $(whoami)
```

And then adding entries to `/etc/sysfs.conf` for your RAPL domains and subdomains.
Check your folder hierarchy in `/sys/class/powercap/` to determine which domains
you have available to your CPU.

Then for each domain, add the following lines to `/etc/sysfs.conf`.
(Requires `sysfsutils` to be installed.)

For example, for package 0:

```bash
mode class/powercap/intel-rapl:0/energy_uj = 0440
owner class/powercap/intel-rapl:0/energy_uj = root:rapl
```

And for its first subdomain:

```bash
mode class/powercap/intel-rapl:0:0/energy_uj = 0440
owner class/powercap/intel-rapl:0:0/energy_uj = root:rapl
```

Finally, restart the `sysfsutils` service.

```bash
sudo systemctl restart sysfsutils
```
