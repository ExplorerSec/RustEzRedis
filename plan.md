### 1. 基本数据操作

```text
// 字符串操作
SET key value [EX seconds] [PX milliseconds]
GET key
DEL key [key ...]
EXISTS key [key ...]
INCR key  // 这啥？不支持！
DECR key  // 这啥？不支持！
```

```text
// 哈希操作
HSET key field value
HGET key field
HGETALL key
HDEL key field [field ...]
```

```text
// 列表操作
LPUSH key element [element ...]
RPUSH key element [element ...]
LPOP key
RPOP key
LRANGE key start stop
```

```text
// 集合操作
SADD key member [member ...]
SMEMBERS key
SREM key member [member ...]
SISMEMBER key member
```

### 2. 系统操作

```text
PING [message]
ECHO message
QUIT
INFO [section]
FLUSHDB
FLUSHALL
```
