# Run on a fake block chain through tests
1. Clone the Libra repository.
2. Follow the Libra README for how to compile and install it.
3. Create the directory `language/functional_tests/tests/testsuite/sprint` in the `Libra` directory.
4. Copy the `.mvir` file you want to run/test to `language/functional_tests/tests/testsuite/sprint`.
5. Run `cargo test -p functional_tests sprint` to run all the tests in sprint or `cargo test -p functional_tests sprint/<name>` to run a specific one.

Example (run in Libra folder which is a sibiling of Sprint).
```
cp ../sprint/move/contracts/example.mvir ./language/functional_tests/tests/testsuite/sprint/ && cargo test -p functional_tests sprint/example
```

# Running on an actual blockchain

## On the server
```
cargo run -p libra_swarm
```

This will tell you to run something like this:
```
cargo run --bin client -- -a localhost -p <port> -s "<path_to_consensus_peers_config>" -m "<path_to_faucet_keys>"
```

## On your machine

### Gettings connected to the swarm

```
ssh -NL 5000:127.0.0.1:<port> pablo@sprint-swarm.uksouth.cloudapp.azure.com
```

```
mkdir swarm_server_files
```

```
scp pablo@sprint-swarm.uksouth.cloudapp.azure.com:<path_to_consensus_peers_config> swarm_server_files
```

```
scp pablo@sprint-swarm.uksouth.cloudapp.azure.com:<path_to_faucet_keys> swarm_server_files
```

```
cargo run --bin client -- -a localhost -p 5000 -s "swarm_server_files/consensus_peers.config.toml"  -m "swarm_server_files/temp_faucet_keys"
```

### Create an account and give it money
```
libra% account create
```

```
libra% account mintb 0 100
```

```
libra% query balance 0
```

### Compiling your move code
```
libra% dev compile 0 <path to my_module.mvir> <module|script>

> Successfully compiled a program at  <path_to_compiled_code>
```

### Publish a module
```
libra% dev publish 0 <path_to_compiled_code>
```

### Execute a script
```
dev execute 0 <path_to_compiled_code> <main's parameters>
```