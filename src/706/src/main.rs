/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

struct MyHashMap {
    internals: Vec<(i32, i32)>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            internals: Vec::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        for each in self.internals.iter_mut() {
            if each.0 == key {
                each.1 = value;
                return;
            }
        }

        self.internals.push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        for each in self.internals.iter() {
            if each.0 == key {
                return each.1;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        for (ind, each) in self.internals.iter().enumerate() {
            if each.0 == key {
                self.internals.remove(ind);
                return;
            }
        }
    }
}

fn main() {}
