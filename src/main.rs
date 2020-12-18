use std::process;
use std::process::Command;
use signal_hook::{iterator::Signals, SIGTERM, SIGINT};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>>
{
    Command::new("ps")
        .status()
        .expect("");
    
    println!("Running with PID {}. Waiting for signals..", process::id());
    
    let signals = Signals::new(&[SIGTERM, SIGINT])?;

    thread::spawn(move || 
    {
        for sig in signals.forever() 
        {
            println!("Received signal {:?}", sig);
        }
    });

    thread::sleep(Duration::from_secs(1000000));

    Ok(())
}
