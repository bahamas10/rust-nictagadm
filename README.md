really basic rust nictagadm

probably doesn't work or do anything useful

```
$ cargo run -- tags.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/nictagadm tags.txt`
NAME	MACADDRESS	LINK	TYPE
external_nic	00:1b:21:c1:46:d5	-	normal
sdc_underlay_nic	00:1b:21:c1:46:d5	-	normal
internal_nic	00:1b:21:c1:46:d5	-	normal
admin_nic	00:1b:21:c1:46:d4	-	normal
```

```
$ cargo run -- usb-datadyne.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/nictagadm usb-datadyne.txt`
NAME	MACADDRESS	LINK	TYPE
admin_nic	00:11:22:00:11:22	-	normal
```

```
$ cargo run -- usb-portal.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/nictagadm usb-portal.txt`
NAME	MACADDRESS	LINK	TYPE
external_nic	aa:bb:cc:dd:ee:ff	-	normal
admin_nic	00:11:22:33:44:55	-	normal
```
