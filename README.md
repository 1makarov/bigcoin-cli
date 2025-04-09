# bigcoin-cli

```console
$ bigcoin-cli --help

Usage: bigcoin-cli [OPTIONS] --path <PATH> <COMMAND>

Commands:
  initialize   Initial payment [aliases: init]
  add-starter  Add starter miner [aliases: start]
  claim        Claims available rewards
  transfer     Transfer tokens to address [aliases: send]
  print        Print total rewards from all keys
  help         Print this message or the help of the given subcommand(s)

Options:
  -m, --max-threads <MAX_THREADS>  Max concurrent threads [default: 20]
  -p, --path <PATH>                Path to the file with private keys
      --rpc <RPC>                  RPC endpoint [default: https://api.mainnet.abs.xyz]
  -h, --help                       Print help
```