use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    List(VecDeque<String>),
    Hash(HashMap<String, String>),
    Set(HashMap<String, bool>),
    Null,
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match &self {
            Self::String(s) => {
                s.to_owned()
            },
            Self::List(list) =>{
                list.to_owned().into_iter().collect::<Vec<_>>().join(" ")
            },
            Self::Hash(hash) =>{
                let mut s = String::new();
                for (k,v) in hash{
                    s.push_str(k);
                    s.push(':');
                    s.push_str(v);
                    s.push(',');
                }
                s
            },
            Self::Set(set) =>{
                set.to_owned().into_keys().collect::<Vec<_>>().join(" ")
            },
            Self::Null => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Database {
    pub data: HashMap<String, (Value, Option<u128>)>, // (value, expire_time)
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key).and_then(|(value, expire)| {
            if let Some(expire_time) = expire {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u128;
                if now > *expire_time {
                    return None;
                }
            }
            Some(value)
        })
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.data.insert(key, (value, None));
    }

    pub fn set_with_expiretime(&mut self, key: String, value: Value, expire_in: Option<u128>) {
        self.data.insert(key, (value, expire_in));
    }

    pub fn set_with_duration(&mut self, key: String, value: Value, duration: Option<Duration>) {
        let expire_time = duration.map(|duration_inner| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u128;
            now + duration_inner.as_millis() as u128
        });

        self.data.insert(key, (value, expire_time));
    }

    pub fn del(&mut self, key: &str) -> Option<(Value, Option<u128>)> {
        self.data.remove(key)
    }

    pub fn exists_include_expired(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn exists(&self, key: &str) -> bool {
        if let Some((_, expire_time)) = self.data.get(key) {
            match expire_time {
                None => true,
                &Some(time) => {
                    let now_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u128;
                    time > now_time
                }
            }
        } else {
            false
        }
    }

    pub fn clean_expired(&mut self) {
        let now_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // 直接函数式清理
        self.data
            .retain(|_, (_, t)| t.is_none_or(|time| time > now_time));
        /* 另一种方法：先收集再清理
        let mut tmp_vec = Vec::new();
        for (key, (_, t)) in &self.data {
            if let Some(expire_time) = t {
                if now_time > *expire_time {
                    tmp_vec.push(key.clone());
                }
            }
        }
        let num = tmp_vec.len();
        for k in tmp_vec {
            self.data.remove(&k);
        }*/
    }

    pub fn len_include_expired(&self) -> usize{
        self.data.len()
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn f1_datebase_part1() {
        let mut db = Database::new();
        // None
        assert_eq!(db.get("Unknown Key1"), None);
        // String
        db.set("key1".into(), Value::String("Val".into()));
        assert_eq!(db.get("key1"), Some(&Value::String("Val".into())));
        // String Duration
        db.set_with_duration(
            "key2".into(),
            Value::String("v2".into()),
            Duration::from_millis(100).into(),
        );
        assert_eq!(db.get("key2"), Some(&Value::String("v2".into())));
        sleep(Duration::from_millis(200));
        assert_eq!(db.get("key2"), None);
    }

    #[test]
    fn f2_datebase_part2() {
        let mut db = Database::new();
        assert_eq!(db.get("key"), None);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let expire_time = Duration::from_millis(100).as_millis() + now;
        db.set_with_expiretime("key".into(), Value::String("val".into()), Some(expire_time));
        assert_eq!(db.get("key"), Some(&Value::String("val".into())));

        sleep(Duration::from_millis(200));
        assert_eq!(db.get("key"), None);
    }

    #[test]
    fn f3_database_part3(){
        let mut db = Database::new();
        assert_eq!(db.len_include_expired(),0);
        let val = Value::String("".into());
        let duration = Some(Duration::from_millis(100));
        db.set("k1".into(),val.clone());
        db.set_with_duration("k2".into(), val.clone(), duration.clone());
        db.set_with_duration("k3".into(), val.clone(), duration.clone());
        assert_eq!(db.len_include_expired(),3);
        sleep(Duration::from_millis(200));
        assert_eq!(db.len_include_expired(),3);
        assert_eq!(db.exists("k2"),false);
        assert_eq!(db.exists_include_expired("k2"),true);
        db.clean_expired();
        assert_eq!(db.exists_include_expired("k2"),false);
        assert_eq!(db.len_include_expired(),1);
        
        assert_eq!(db.del("k2"),None);
        assert!(db.del("k1").is_some());
        assert_eq!(db.len_include_expired(),0);
    }
}
