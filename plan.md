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
HSET key field value                         ⏺️
HGET key field                               ⏺️
HGETALL key                                  ⏺️
HDEL key field [field ...]                   ⏺️
```

```rs
// 列表操作
LPUSH key element [element ...]              ⏺️
RPUSH key element [element ...]              ⏺️
LPOP key                                     ⏺️
RPOP key                                     ⏺️
LRANGE key start stop                        ⏺️
```

```rs
// 集合操作
SADD key member [member ...]                 ⏺️
SMEMBERS key                                 ⏺️
SREM key member [member ...]                 ⏺️
SISMEMBER key member                         ⏺️
```

### 2. 系统操作

```rs
PING [message]                               ✅
ECHO message                                 ⏺️
QUIT                                         ⏺️
INFO [section]                               ⏺️
FLUSHDB                                      ⏺️
FLUSHALL                                     ⏺️
```



