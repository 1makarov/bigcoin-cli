# bigcoin-cli

```console
$ bigcoin-cli --help

Usage: bigcoin-cli [OPTIONS] --path <PATH> <COMMAND>

Commands:
  initialize    Initial payment [aliases: init]
  add-starter   Add starter miner [aliases: start]
  claim         Claims available rewards
  transfer      Transfer tokens to address [aliases: send]
  transfer-eth  Transfer eth to address [aliases: eth]
  print         Print total rewards
  help          Print this message or the help of the given subcommand(s)

Options:
  -m, --max-threads <MAX_THREADS>  Max concurrent threads [default: 20]
  -p, --path <PATH>                Path to the file with private keys
      --rpc <RPC>                  RPC endpoint [default: https://api.mainnet.abs.xyz]
  -h, --help                       Print help
```

## Contracts

- [0x09Ee83D8fA0f3F03f2aefad6a82353c1e5DE5705](https://abscan.org/address/0x09Ee83D8fA0f3F03f2aefad6a82353c1e5DE5705) (main)
- [0xDf70075737E9F96B078ab4461EeE3e055E061223](https://abscan.org/address/0xDf70075737E9F96B078ab4461EeE3e055E061223) (token)