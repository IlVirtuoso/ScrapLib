use thirtyfour::{WebDriver, By, WebElement};

use super::Strategy::IStrategy;


pub struct EbayStrategy{
      target:String,
      currentPage: usize,
      driver: WebDriver
}

impl IStrategy for EbayStrategy{
    fn init(&self) {
        todo!()
    }

    fn execute(&self) {
        todo!()
    }

    fn end(&self) {
        
    }
}


impl EbayStrategy{
    pub fn new(driver: WebDriver, target: String) -> EbayStrategy{
        EbayStrategy { target: target, currentPage: 0, driver: driver }
    }

    pub async fn get_page_elements(&mut self) -> Option<Vec<WebElement>>{
        self.driver.goto(self.target.clone());
        let list = self.driver.find(By::ClassName("srp-list")).await;
        if let Ok(res) = list{
            let items = res.find_all(By::Tag("li")).await;
            if let Ok(it) = items{
                return Some(it);
            }
        }
        return None;
    }


}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_get_elements() {
        
    }
}
