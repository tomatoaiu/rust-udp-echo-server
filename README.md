# UdpEchoServer by Rust

> client in host -> server1 in container -> server2 in container -> server3 in host

## Usage

1. Open docker-compose.yml
2. Edit line 30 [`"127.0.0.1:34256"` -> `"<your host ip address>:34256"`]
3. Please do the following in order

```sh
make up

# server 1 in container
make logs1

# server 2 in container
# open new terminal
make logs2

# server 3 in host
# open new terminal
nc -lu -p 34256

# client in host
# open new terminal
nc -u localhost 34254
foobar
```

## note

### item

```sh
apt-get update && apt-get install -y kill
# You can use the nc command by installing the netcat command
apt-get update && apt-get install -y netcat
apt-get update && apt-get install -y lsof
```

### my ip address
mac user
```sh
ifconfig en0 | grep inet | awk '$1=="inet" {print $2}'
```

windows user
```sh
ifconfig eth0 | grep inet | awk '$1=="inet" {print $2}'
```
