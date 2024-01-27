# caei
caei is an implementation of the 2048 game in Rust :crab:

## How to

## UI
To play the game via a UI, you need to start first the server under `caei-server`:

```shell
cd caei-server/
cargo un
```

And then, in a new terminal, start the client under `caei-ui`:

```shell
cd caei-ui/
npm start
```


## CLI

You can run the game via a CLI by running the `caei-cli` project:

```shell
cargo run --bin caei-cli
```