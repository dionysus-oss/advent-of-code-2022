use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct Timer {
    instant: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Timer {
            instant: Instant::now(),
        }
    }

    pub fn stop(self) {
        println!("Elapsed: {:.2?}", self.instant.elapsed());
    }
}
