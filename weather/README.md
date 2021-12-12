# Rust [weatherapi](https://weatherapi.com) client
Get outside temperature in any city.

## Installing
Installation is done with cargo.

### Download
```shell
$ git clone https://github.com/romeq/rust-crap
```

### Configure
[weatherapi](https://weatherapi.com) requires an API key to work, and thus you need to create account there.
After you've registered your account, you can find your api key in account portal.

Now, you can find `key` variable in `src/main.rs:15`. Insert your key there.


### Build
```shell
$ cd rust-crap/weather 
$ cargo build
```

You can now find your weather app as binary in `target/debug/weather`
