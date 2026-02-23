# RAPL Energy

Reading CPU energy consumption and controlling CPU power limits through RAPL.

Reading RAPL requires elevated permissions.

## RAPL permissions

* Create a new `rapl` group.

```bash
sudo groupadd rapl
sudo usermod -aG rapl $USER
```

* Create a new file `sudo nano /etc/udev/rules.d/70-intel-rapl.rules` and add the following rule.

```bash
ACTION=="add", SUBSYSTEM=="powercap", KERNEL=="intel-rapl:*", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/energy_uj", \
  RUN+="/usr/bin/chmod g+r /sys/%p/energy_uj"
```

Reboot, and check if you can read `cat /sys/class/powercap/intel-rapl:*/energy_uj`
