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

## RAPL permissions (Arch)

These instructions should be distribution-invariant and even work on Debian-based distributions, I think.

1. Create and edit the following file: `sudo nano /etc/udev/rules.d/99-powercap.rules`
2. Put the following into the file: `ACTION=="add", SUBSYSTEM=="powercap", KERNEL=="intel-rapl:0", RUN+="/bin/chmod 644 /sys/class/powercap/%k/energy_uj"`
3. Reload udev rules: `sudo udevadm control --reload-rules`
4. Trigger the rule: `sudo udevadm trigger --verbose --subsystem-match=powercap --action=add`
5. Check if the rule worked (you should have read permissions for the file): `ls -la /sys/class/powercap/intel-rapl:0/energy_uj`

The reason we're doing it this way is because permission changes applied to /sys/ get reset on reboot, so this is a way to make the permission change persistent.
