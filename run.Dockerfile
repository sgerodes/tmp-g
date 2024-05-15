FROM solo-chain-build

WORKDIR /workspace

ENV NODE_EXECUTABLE="./target/release/g6-solo-node"
ENV STATE_DATA_BASE_FOLDER=./chain-data/

CMD $NODE_EXECUTABLE \
    --$WHO \
    --chain $CHAIN \
    --base-path $STATE_DATA_BASE_FOLDER/$WHO/ \
    --name "${WHO}-validator" \
    --port 30300 \
    --rpc-port 8600 \
    --node-key $NODE_KEY \
    --validator
