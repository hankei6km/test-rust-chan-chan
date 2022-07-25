pub mod chanchan;
pub mod normal;

use rand::{prelude::ThreadRng, Rng};
use std::{thread, time::Duration};

pub struct Wait {
    rng: ThreadRng,
}
impl Wait {
    pub fn wait(&mut self, _i: i32) {
        // let range = if _i % 2 == 0 { 3000..3001 } else { 499..501 };
        let range = 499..501;
        thread::sleep(Duration::from_millis(self.rng.gen_range(range)));
    }
}
