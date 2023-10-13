# dst-ping

Checks if a Don't Starve Together game server is alive and reports it's approximate latency.  
May be used as Docker health check.

Implemented using [RakNet's](https://github.com/facebookarchive/RakNet) unconnected ping feature.

## Usage

```
dst-ping <ADDRESS> <PORT>

Arguments:
  <ADDRESS>  IP address of the DST server
  <PORT>     Game port of the DST server

Options:
  -h, --help  Print help
```

Example Output:

```
Host:        192.168.178.103:11000
Server GUID: 8F 84 C1 30 36 AC D4 A9
Time         0.255 ms
```
