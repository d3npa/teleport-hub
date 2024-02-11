# teleport-hub

teleport-hub provides a front-end over HTTP to change exit nodes when using a vpn concentrator via pf-tables and rdomain. 

the logic is implemented through pf(4), rdomain(4). this front-end is designed to be run on a server configured with teleporter-factory (another repo of mine)
# setup

```
$ cargo build --release
# cp target/release/teleport-hub /usr/bin/teleport-hub
# cp config.toml /var/teleport-hub/config.toml
# cp teleporthubd /etc/rc.d/teleporthubd
# rcctl enable teleporthubd
# rcctl start teleporthubd
```

you'll probably want some exit defs too. using teleport-factory it might look something like this:

```
$ cd teleporter-factory
$ ls -d exits
exits
$ cargo run -- --tar
...
[*] saved to './teleporters.tar'
# tar xf teleporters.tar -C /
etc/
etc/teleport-rs
etc/teleport-rs/exits.toml
etc/pf
etc/pf/teleport_hub.conf
etc/hostname.wg53
etc/hostname.wg52
etc/hostname.wg51
# sh /etc/netstart wg51 wg52 wg53
# nvi /etc/pf.conf # add 'include "/etc/pf/teleport_hub.conf"' where needed
# pfctl -nf /etc/pf.conf && pfctl -f /etc/pf.conf
# rcctl restart teleporthubd
```

