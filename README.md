# RAPL Energy

Reading CPU energy consumption and controlling CPU power limits through RAPL.

Reading RAPL requires elevated permissions.

## RAPL permissions

Create a new `rapl` group:
```bash
sudo groupadd rapl
sudo usermod -aG rapl $USER
```

Create a new file `/etc/udev/rules.d/70-intel-rapl.rules` and add the following rule:
```bash
ACTION=="add", SUBSYSTEM=="powercap", KERNEL=="intel-rapl:*", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/name", \
  RUN+="/usr/bin/chmod g+r  /sys/%p/name", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/max_energy_range_uj", \
  RUN+="/usr/bin/chmod g+r  /sys/%p/max_energy_range_uj", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/energy_uj", \
  RUN+="/usr/bin/chmod g+r  /sys/%p/energy_uj"
```

You can find available domains and their permissions with:
```bash
find /sys/class/powercap/intel-rapl/ -type f -printf '%M %p\n'
```

If you also need access to the power limiting capabilities, add:
```bash
  RUN+="/usr/bin/chgrp rapl /sys/%p/constraint_*_name", \
  RUN+="/usr/bin/chmod g+r  /sys/%p/constraint_*_name", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/constraint_*_max_power_uw", \
  RUN+="/usr/bin/chmod g+r  /sys/%p/constraint_*_max_power_uw", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/constraint_*_power_limit_uw", \
  RUN+="/usr/bin/chmod g+rw /sys/%p/constraint_*_power_limit_uw", \
  RUN+="/usr/bin/chgrp rapl /sys/%p/constraint_*_time_window_us", \
  RUN+="/usr/bin/chmod g+r  /sys/%p/constraint_*_time_window_us"
```

Reboot, and check if you can read `cat /sys/class/powercap/intel-rapl:*/energy_uj`.
