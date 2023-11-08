use clap::Parser;
use rppal as rpi;
use signal_hook;
use system_shutdown as shutdown;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

const DEFAULT_CHECK_WAIT_MS: u64 = 100;
const DEFAULT_DELAY_MS: u64 = 0;
const DEFAULT_SHUTDOWN_GPIO: u8 = 3;
const DEFAULT_INITIAL_WAIT_MS: u64 = 500;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(long, default_value_t = DEFAULT_CHECK_WAIT_MS)]
  check_wait_ms: u64,

  #[arg(long, default_value_t = DEFAULT_DELAY_MS)]
  delay_ms: u64,

  #[arg(long, default_value_t = DEFAULT_SHUTDOWN_GPIO)]
  shutdown_gpio: u8,

  #[arg(long, default_value_t = DEFAULT_INITIAL_WAIT_MS)]
  initial_delay_ms: u64,
}

fn main() {
  // creating a signal hook to end process in case of shutdown by other means
  let end = Arc::new(AtomicBool::new(false));
  signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&end))?;

  // parsing command line arguments
  let args = Args::parse();

  // createing gpio input pin
  let pin = rpi::gpio::Gpio::new()?
    .get(args.shutdown_gpio)?
    .into_input_pullup();

  // wait so that everything settles
  sleep(Duration::from_millis(args.initial_delay_ms));

  while !end.load(Ordering::Relaxed) {
    if pin.is_low() {
      sleep(Duration::from_millis(args.delay_ms));
      shutdown::shutdown();
      break;
    }

    sleep(Duration::from_millis(args.check_wait_ms));
  }
}
