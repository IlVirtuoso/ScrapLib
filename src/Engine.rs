use thirtyfour::prelude::WebDriverResult;

use crate::Strategies::{self, Strategy::IStrategy};

use std::collections::VecDeque;

use self::Strategies::Strategy;


static mut STRATEGY_QUEUE: VecDeque<&dyn Strategy::IStrategy> = VecDeque::new();

pub fn enqueue_task(strategy: &'static dyn IStrategy) {
    unsafe {
        STRATEGY_QUEUE.push_front(strategy);
    }
}

fn deque_task() -> Option<&'static dyn IStrategy> {
    unsafe {
        if let Some(m) = STRATEGY_QUEUE.front() {
            STRATEGY_QUEUE.pop_front();
            return Some(*m);
        }
        return None;
    }
}

pub async fn execute() -> WebDriverResult<()> {
      loop {
          if let Some(strategy) = deque_task(){
            strategy.init();
            strategy.execute();
            strategy.end();
          }
          else {
              return Ok(());
          }
      }
}
