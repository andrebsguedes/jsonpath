// use serde_json::Value;
use super::value::{FbValue, Value};
use std::collections::HashSet;

pub(super) struct ValueWalker;

impl<'a> ValueWalker {
    pub fn all_with_num(vec: &[FbValue<'a>], tmp: &mut Vec<FbValue<'a>>, index: f64) {
        Self::walk(vec, tmp, &|v| if let Value::Array(v) = v.into() {
            if let Some(item) = v.get(index as usize) {
                Some(vec![item.clone()])
            } else {
                None
            }
        } else {
            None
        });
    }

    pub fn all_with_str(vec: &[FbValue<'a>], tmp: &mut Vec<FbValue<'a>>, key: &str, is_filter: bool) {
        if is_filter {
            Self::walk(vec, tmp, &|v| match v.clone().into() {
                Value::Object(map) if map.contains_key(key) => Some(vec![v]),
                _ => None,
            });
        } else {
            Self::walk(vec, tmp, &|v| match v.into() {
                Value::Object(map) => match map.get(key) {
                    Some(v) => Some(vec![v.clone()]),
                    _ => None,
                },
                _ => None,
            });
        }
    }

    pub fn all(vec: &[FbValue<'a>], tmp: &mut Vec<FbValue<'a>>) {
        Self::walk(vec, tmp, &|v| match v.into() {
            Value::Array(vec) => Some(vec.into_iter().collect()),
            Value::Object(map) => {
                let mut tmp = Vec::new();
                for (_k, v) in map {
                    tmp.push(v);
                }
                Some(tmp)
            }
            _ => None,
        });
    }

    fn walk<F>(vec: &[FbValue<'a>], tmp: &mut Vec<FbValue<'a>>, fun: &F) where F: for<'b> Fn(FbValue<'b>) -> Option<Vec<FbValue<'b>>> {
        for v in vec {
            Self::_walk(v.clone(), tmp, fun);
        }
    }

    fn _walk<F>(v: FbValue<'a>, tmp: &mut Vec<FbValue<'a>>, fun: &F) where F: for<'b> Fn(FbValue<'b>) -> Option<Vec<FbValue<'b>>> {
        if let Some(mut ret) = fun(v.clone()) {
            tmp.append(&mut ret);
        }

        match v.into() {
            Value::Array(vec) => {
                for v in vec {
                    Self::_walk(v, tmp, fun);
                }
            }
            Value::Object(map) => {
                for (_k, v) in map {
                    Self::_walk(v, tmp, fun);
                }
            }
            _ => {}
        }
    }

    pub fn walk_dedup(v: FbValue<'a>,
                      tmp: &mut Vec<FbValue<'a>>,
                      key: &str,
                      visited: &mut HashSet<FbValue<'a>>, ) {
        match v.clone().into() {
            Value::Object(map) => {
                if map.contains_key(key) {
                    if !visited.contains(&v) {
                        visited.insert(v.clone());
                        tmp.push(v)
                    }
                }
            }
            Value::Array(vec) => {
                for v in vec {
                    Self::walk_dedup(v, tmp, key, visited);
                }
            }
            _ => {}
        }
    }
}

