# RustAn

[Spartan](https://portal.mozz.us/spartan/spartan.mozz.us/) server written in Rust.

## Short info

Small server that will cover all basic needs.

It supports:
 * Serving static files
 * Serving file lists
 * CGI-like scripts(all input data will be send to stdin of process)
 * Basic configuration

## Configuration

Log level always fetched via enviroment. 

Use `RUST_LOG` variable for setting log level(info, debug, trace etc).

There two ways of configuration of server:
 * Via enviroment
 * Via toml-file
 * Nothing - will be used default configuration

### Configuration via enviroment

Just setup next variables:
 * `RUSTAN_HOST`(for example `RUSTAN_HOST="127.0.0.1:3001" ./rustan`)
 * `RUSTAN_SERVER_ROOT` (for example `RUSTAN_SERVER_ROOT="/var/spartan ./rustan"`)
 * `RUSTAN_MAX_UPLOAD_SIZE` (for example `RUSTAN_MAX_UPLOAD_SIZE=1024`)

Entire command can look like this:
```
RUST_LOG=info RUSTAN_HOST="127.0.0.1:3001" RUSTAN_SERVER_ROOT="/var/spartan" RUSTAN_MAX_UPLOAD_SIZE=1 ./rustan
```

### TOML-configuration

There example file `settings.toml.example` that can be used as template.

When RustAn starting it looking for `settings.toml` in current directory and in `/etc/rustan/`. 

Settings can be overrided via enviroment configuration.

## ToDo
 
 - [ ] Add redirects support
 - [ ] Handle directories without slashes on end
 - [ ] Windows support(may be it will work out of the box, not tested)
 - [ ] Maybe something else?

## License

I've licensed project by [Nihirash's Coffeeware License](LICENSE).

Please respect it - it isn't hard.