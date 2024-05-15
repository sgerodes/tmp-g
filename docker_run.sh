
docker build -t solo-chain-build -f build.Dockerfile .


#docker build --build-arg WHO=alice --build-arg NR=1 -t solo-chain-run -f run.Dockerfile .
docker build -t solo-chain-run -f run.Dockerfile .
#docker run solo-chain-run
docker run -e WHO=alice -e CHAIN=local -e NODE_KEY=0000000000000000000000000000000000000000000000000000000000000001 solo-chain-run
docker run -e WHO=bob -e CHAIN=local -e NODE_KEY=0000000000000000000000000000000000000000000000000000000000000002 solo-chain-run
