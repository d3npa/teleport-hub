# teleport-hub

teleport-hub provides a front-end over HTTP to change exit nodes when using a vpn concentrator via pf-tables and rdomain. 

the logic is implemented through pf(4), rdomain(4). this front-end is designed to be run on a server configured with teleporter-factory (another repo of mine)
# installation

```
$ cargo build --release
# cp target/release/teleport-hub /usr/bin/teleport-hub
# cp config.toml /var/teleport-hub/config.toml
# cp teleporthubd /etc/rc.d/teleporthubd
# rcctl enable teleporthubd
# rcctl start teleporthubd
```

# configuration

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
etc/teleport-hub
etc/teleport-hub/exits.toml
etc/pf
etc/pf/teleport_hub.conf
etc/hostname.wg53
etc/hostname.wg52
etc/hostname.wg51
var/
var/teleport-hub
var/teleport-hub/tables
# sh /etc/netstart wg51 wg52 wg53
# nvi /etc/pf.conf # add 'include "/etc/pf/teleport_hub.conf"' where needed
# pfctl -nf /etc/pf.conf && pfctl -f /etc/pf.conf
# rcctl restart teleporthubd
```

here is an example configuration if you'd prefer not to use teleporter-factory:

```
# /etc/teleport-hub/exits.toml
exits = [
	{ pf_id = 'exit1', display_name = 'SomeVPN (Tōkyō)' },
	{ pf_id = 'exit2', display_name = 'SomeVPN (Sapporo)' },
	{ pf_id = 'exit3', display_name = 'SomeOtherVPN (Reykjavík)' },
]
```
