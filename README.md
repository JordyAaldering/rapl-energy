# RAPL Energy

Reading CPU energy consumption and controlling CPU power limits through RAPL.

Reading RAPL requires elevated permissions.

## RAPL permissions

* Create a new `rapl` group.

```bash
sudo groupadd rapl
sudo usermod -aG rapl $USER
```

* Create a new file `sudo nano /etc/udev/rules.d/70-intel-rapl.rules` and add the following rule:

```bash
ACTION=="add", SUBSYSTEM=="powercap", KERNEL=="intel-rapl:*", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/energy_uj", \
  RUN+="/usr/bin/chmod g+r /sys/%p/energy_uj"
```

* Reload udev rules: `sudo udevadm control --reload-rules`
* Trigger the rule: `sudo udevadm trigger --verbose --subsystem-match=powercap`
* Check if you have read permissions: `ls -l /sys/class/powercap/intel-rapl:*/energy_uj`

## RAPL permissions (alternative method using sysfn)

Create a new `rapl` group.

```bash
sudo addgroup rapl
sudo usermod -aG rapl $USER
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
