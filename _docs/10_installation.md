---
layout: doc
title: Installation
sort_index: 10
---

## Manual installation

The minimal setup of mediarepo only requires the client 
application (mediarepo-ui). This allows you to connect to remote repositories
via IP but not to create and open local repositories.
In order to open local repositories you also need to have the
*mediarepo-daemon* application installed on your system.
This application manages a single repository and enables you to
have it running in the background while using minimal resources.
Both daemon and client can be downloaded from the [Releases]({{ site.urls.releases }})
page.

The client application comes in several bundle variants
and as a single executable. Depending on your system configuration
you can use the executable directly or need to use one of the bundles.
I haven't really tested that so you have to figure it out yourself.

The daemon application only comes as a single executable as it only requires
minimal external dependencies which should be installed on most systems.
You can use the daemon manually to initialize and host a repository or you
can use the client to do the initialization and daemon startup for you.

When starting the client for the first time it creates a config `settings.toml` file somewhere
in your users application config folder (usually `$HOME/.config/mediarepo` on linux,
somewhere in `%APPDATA%` on windows and in the nether dimension on mac).
This application contains an entry `daemon_path` which points to the
`mediarepo-daemon` executable on your system. The client application searches for the daemon
executable in the `PATH` automatically on startup. If it can't find it the entry will stay empty.
If this entry doesn't exist or has an empty value, you have to 
configure it yourself and set it to the folder you placed the daemon executable in.
Either way a valid configuration for local usage should look as follows:

```toml
# settings.toml
daemon_path = "/usr/bin/mediarepo-daemon"
```


## AUR

Mediarepo can also be installed from the AUR which is much easier than
installing it the manual way (not everyone uses arch tho). All you need to install
is the `mediarepo` package for the client and the `mediarepo-daemon` package for
the daemon. As both applications are installed in `/usr/bin` the client should be
able to find the daemon executable automatically.