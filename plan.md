# 功能支持

> ☑️ 部分支持该功能
> ✅ 完全支持该功能
> ⏺️ 暂未支持该功能

### 1. 基本数据操作

```rs
// 字符串操作
SET key value [EX seconds] [PX milliseconds] ✅
GET key                                      ✅
DEL key [key ...]                            ✅
EXISTS key [key ...]                         ✅
INCR key                                     ✅
DECR key                                     ✅
```

```rs
// 哈希操作
HSET key field value [field value ...]       ✅
HGET key field                               ✅
HGETALL key                                  ✅
HDEL key field [field ...]                   ✅
HEXISTS key field                            ✅
HLEN key                                     ✅
HKEYS key                                    ✅
HVALS key                                    ✅
```

```rs
// 列表操作
LPUSH key element [element ...]              ✅
RPUSH key element [element ...]              ✅
LPOP key                                     ✅
RPOP key                                     ✅
LINDEX key index                             ✅
LRANGE key start stop                        ✅
LLEN key                                     ✅
LSET key index value                         ✅
LREM key count value                         ⏺️
LTRIM key start stop                         ⏺️
```

```rs
// 集合操作
SADD key member [member ...]                 ✅
SCARD key                                    ✅
SMEMBERS key                                 ✅
SREM key member [member ...]                 ✅
SISMEMBER key member                         ✅
SINTER key1 [key2]                           ⏺️
SINTERSTORE destination key1 [key2]          ⏺️
SUNION key1 [key2]                           ⏺️
SUNIONSTORE destination key1 [key2]          ⏺️
SDIFF key1 [key2]                            ⏺️
SDIFFSTORE destination key1 [key2]           ⏺️
```

### 2. 系统操作

```rs
PING [message]                               ✅
ECHO message                                 ✅
QUIT                                         ⏺️
INFO [section]                               ⏺️
FLUSHDB                                      ⏺️
FLUSHALL                                     ⏺️
AUTH                                         ⏺️
SAVE                                         ⏺️
```



