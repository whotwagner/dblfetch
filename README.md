# dblfetch

Fetches dynamic denylists from web sources and executes commands with the IP-ranges.

# How it works

dblfetch downloads denylists or ipsets from web-sources and parses all ip-ranges(in cidr-format or single ip-addresses).
These IP ranges are then used in commands that are executed by dblfetch. Typically, these are commands for adding
addresses to ipsets. dblfetch distinguishes whether it is an ipv4 or ipv6 address and executes different commands for
each IP version. All downloaded denylists are stored in a cache, which is only updated when the time for this cache has expired.
In this way, downloads are optimized to a minimum.

# Features

* Use cache with timeouts
* Allow seperated commands for IPv4 and IPv6
* Write logs
* Multiple sources
* Use http-loader

# Build

Use cargo to build a release

```
cargo build -r
```

## Using Docker

First create a build-container:

```
$ docker build -t dblfetchbuilder .
```

Use this builder to compile the app:

```
$ docker run -v $PWD/.:/myapp --rm dblfetchbuilder cargo build -r
```

# Install

Copy the binary to `$PATH`

```
$ sudo cp target/release/dblfetch /usr/local/bin
```

# Configure

Copy the config-file to `/etc/dblfetch.yaml` and edit its content.

```
$ sudo cp dblfetch.yaml /etc/dblfetch.yaml 
```

After that create the logfile `/var/log/dblfetch.log` and make it
writeable for the user that runs dblfetch:

```
$ sudo touch /var/log/dblfetch.log
$ sudo chown dblfetch.dblfetch /var/log/dblfetch.log
$ sudo chmod 660 /var/log/dblfetch.log
```

You need to create the dblfetch-user first. Please note that dblfetch
will write the cache-files to `$HOME/.cache/dblfetch`

# Schedule Task

Set the following cronjob:

```
*/15 * * * * /usr/local/bin/dblfetch
```

**Please note that it is recommended to execute dblfetch manually for the first time in order to find out how long it
takes to run the task**

# License

GPL v3.0

# Author

Wolfgang Hotwagner
