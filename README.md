# RAPL Energy

Reading CPU energy consumption and controlling CPU power limits through RAPL.

## RAPL permissions (Debian)

Reading RAPL requires elevated permissions.

I suggest adding a new `rapl` group.

```bash
sudo addgroup rapl
sudo usermod -aG rapl $(whoami)
```

Then add entries to `/etc/sysfs.conf` for your RAPL domains and subdomains.
Check the folder hierarchy in `/sys/class/powercap/intel-rapl` to determine which domains are available to your CPU.

Then, for each domain, add the following lines to `/etc/sysfs.conf` (requires `sysfsutils` to be installed).

For example, for package 0:

```bash
mode class/powercap/intel-rapl/intel-rapl:0/energy_uj = 0440
owner class/powercap/intel-rapl/intel-rapl:0/energy_uj = root:rapl
```

And for its first subdomain:

```bash
mode class/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:0/energy_uj = 0440
owner class/powercap/intel-rapl/intel-rapl:0/intel-rapl:0:0/energy_uj = root:rapl
```

Finally, restart the `sysfsutils` service.

```bash
sudo systemctl restart sysfsutils
```
