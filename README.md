# RAPL Energy

Reading CPU energy consumption and controlling CPU power limits through RAPL.

Reading RAPL requires elevated permissions.

## RAPL permissions

Create a new `rapl` group:
```bash
sudo groupadd rapl
sudo usermod -aG rapl $USER
```

Create a new file `/usr/local/sbin/set-rapl-permissions`:
```bash
#!/bin/sh

for d in /sys/class/powercap/intel-rapl/intel-rapl:*; do
    [ -d "$d" ] || continue

    for f in "$d/name" \
             "$d/energy_uj" \
             "$d/max_energy_range_uj" \
             "$d"/constraint_*_name \
             "$d"/constraint_*_max_power_uw \
             "$d"/constraint_*_time_window_us; do
        [ -e "$f" ] || continue
        chgrp rapl "$f"
        chmod g+r "$f"
    done

    for f in "$d"/constraint_*_power_limit_uw; do
        [ -e "$f" ] || continue
        chgrp rapl "$f"
        chmod g+rw "$f"
    done
done
```

Set its permissions:
```bash
sudo chmod 755 /usr/local/sbin/set-rapl-permissions
```

Create a new file `/etc/udev/rules.d/70-intel-rapl.rules` and add the following rule:
```bash
ACTION=="add", SUBSYSTEM=="powercap", KERNEL=="intel-rapl:*", \
  RUN+="/usr/local/sbin/set-rapl-permissions"
```

Reload the rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger --subsystem-match=powercap
```

You should now be able to read the energy counters, and adjust the power limit, if available.

You can check your permissions, and available domains, with:
```bash
find /sys/class/powercap/intel-rapl/ -type f -printf '%M %p\n'
```
