## Facts


Facts collects and distributes variables containing information about a host.

The current status of this project is **experimental**.

|            Fact               |    Status    |
|-------------------------------|---------------|
|CPU information                | In progress   |
|Memory Information             | In progress   |
|Operational System Information |       -       |
|Disk Information               |       -       |
|Time Information               |       -       |
|Network Information            |       -       |


This project is currently focused on Unix-like Operating Systems, but in the future other operating systems may be supported.


## Env

```sh
FATCS_BROKER=localhost:9092
# delay in miliseconds
FATCS_CPUINFO_DELAY=5000
FATCS_MEMINFO_DELAY=5000
```

## Messages Broker

Red Panda
```sh
docker-compose run redpanda
```

Kafka with Zookeeper

```sh
docker-compose run kafka-zookeeper
```

Kafka with Raft
```sh
docker-compose run kafka-kraft
```


rpk topic create facts-cpuinfo --replicas 1
rpk topic create facts-meminfo --replicas 1

rpk topic consume facts-cpuinfo
rpk topic consume facts-meminfo
