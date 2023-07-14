use std::{fmt::format, process::Command, time::Duration};
use std::time;
use futures::Future;
use log::{info, warn};
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::time::{error::Elapsed, Timeout};

pub async fn build_firefox_driver(server: String, headless: bool) -> Option<WebDriver> {
    let mut caps = DesiredCapabilities::firefox();
    let _ = caps.set_headless();
    let driver = WebDriver::new(&server, caps).await;
    if let Ok(d) = driver {
        return Some(d);
    } else if let Err(e) = driver {
        let message = e.to_string();
        warn!("Failed to build for firefox is the server down, error: {message} ?");
    }
    return None;
}



pub fn buildgecko(port: i32) -> Option<std::process::Child>{

      let r = Command::new("firefox.geckodriver")
      .args(["--port",port.to_string().as_str()]).spawn();
      std::thread::sleep(Duration::from_secs(1));
      if let Ok(result) = r{
            return Some(result);
      }
      else if let Err(error) = r{
            let message = error.to_string();
            log::error!("unexpected error {message}");
      }

      return None;
}

pub fn build_firefox(){

}


#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_firefox(){
      let proc = buildgecko(4444);
      assert!(proc.is_some());
      let driver = build_firefox_driver("http://localhost:4444".to_string(), true).await;
      assert!(driver.is_some());
      assert!(driver.unwrap().quit().await.is_ok());
      let _ = proc.unwrap().kill();
    }
}
