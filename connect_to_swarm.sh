#!/usr/bin/env bash

#./connect_to_swarm.sh 45549 ../libra/ /tmp/b8fa2886e24b6fb34072458429532275/0/consensus_peers.config.toml /tmp/60787786b009cc80b74e238da2013ce7/temp_faucet_keys

if [ $# -le 1 ]
then
  echo "Missing arguments error!"
  echo "Usage: ./connect_to_swarm.sh <port> <path to libra repo> <path to consensus peers config> <path to faucet keys>"
  exit -1
fi

PORT=$1;
PATH_TO_LIBRA=$2;

if [ $# -gt 2 ]
then
  CONSENSUS_PEERS_CONFIG_PATH=$3;
  FAUCET_KEYS_PATH=$4;
fi

ssh -NL 5000:127.0.0.1:$PORT pablo@sprint-swarm.uksouth.cloudapp.azure.com &

mkdir -p $PATH_TO_LIBRA/swarm_server_files;

if [ $# -gt 2 ]
then
  scp pablo@sprint-swarm.uksouth.cloudapp.azure.com:$CONSENSUS_PEERS_CONFIG_PATH $PATH_TO_LIBRA/swarm_server_files/;
  scp pablo@sprint-swarm.uksouth.cloudapp.azure.com:$FAUCET_KEYS_PATH $PATH_TO_LIBRA/swarm_server_files/;
fi

cargo run --manifest-path=$PATH_TO_LIBRA/Cargo.toml --bin client -- -a localhost -p 5000 -s "$PATH_TO_LIBRA/swarm_server_files/consensus_peers.config.toml"  -m "$PATH_TO_LIBRA/swarm_server_files/temp_faucet_keys"
