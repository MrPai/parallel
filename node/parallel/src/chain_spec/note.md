## start relaychain
parachain-launch generate config.yml  \
docker-compose -f output/docker-compose.yml up -d --build  \
docker-compose -f output/docker-compose.yml down

## start collator
### seed file name
### keytypeId + public key
// keystore目录需要在scripts目录下 \
keystore/6175726176c546372e4558bd72c0ad1e0e732fc6e3c835ac4dfefe6840bd5e00898d3436
### 生产环境注意使用snashot
`bash scripts/collator.sh "node-key" keystore kerria-collator kerria polkadot-local`

## generate genesisHead
`docker run --rm -v "/root/parallel/node/parallel/src/chain_spec/kerria-3350.json":/kerria-3350.json  parallelfinance/parallel:kerria-3350 export-genesis-state --chain="/kerria-3350.json"`

## On Relaychain
sudo.registrar.forceRegister \
sudo.slots.forceLease \

## On parachain
### check the key
rpc.author.haskey
(aura)
## 注意区分spec的json和raw-json文件

### 当把宿主机的文件夹映射到docker内使，注意宿主机的文件夹权限，
### 766 权限不可以
chmod 777 keystore
### 外部rpc命令发送到docker容器后，可能不能往该文件夹内写私钥
### keystore目录下文件里的私钥需要加上双引号
### 注意如果keystore包含私钥文件，docker容器需要该私钥的读取权限
-v "$(realpath $KEYSTORE_PATH):/app/keystore" \
--keystore-path=/app/keystore \
chmod 666 6175726176c546372e4558bd72c0ad1e0e732fc6e3c835ac4dfefe6840bd5e00898d3436