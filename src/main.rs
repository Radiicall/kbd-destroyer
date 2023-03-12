use clap::Parser;
use std::io::{BufWriter, Write};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'b', long = "backlight", help = "Backlight name")]
    backlight: Option<String>,
    #[arg(
        short = 's',
        long = "sleep",
        help = "Sleep time between execution in micros"
    )]
    sleeptime: Option<u64>,
    #[arg(
        short = 'S',
        long = "range-start",
        help = "Start number for backlight range"
    )]
    rstart: Option<i32>,
    #[arg(
        short = 'E',
        long = "range-end",
        help = "End number for backlight range"
    )]
    rend: Option<i32>,
    #[arg(
        short = 'r',
        long = "reverse",
        help = "If the range should go in reverse too"
    )]
    reverse: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let time = std::time::Duration::from_micros(args.sleeptime.unwrap_or_else(|| 5000));
    let backlight = args
        .backlight
        .unwrap_or_else(|| "kbd_backlight".to_string());
    let path = format!("/sys/class/leds/{}/brightness", backlight.clone());
    let rstart = args.rstart.unwrap_or_else(|| 0);
    let rend = args.rend.unwrap_or_else(|| {
        std::fs::read_to_string(format!("/sys/class/leds/{}/max_brightness", backlight))
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap()
    });
    loop {
        for i in rstart..=rend {
            let file = std::fs::File::create(&path).expect("Must run as root");
            let mut writer = BufWriter::new(file);

            println!("{}", &i);
            writer.write_all(i.to_string().as_bytes()).ok().unwrap_or_else(|| println!("Failed to write file"));

            writer.flush().ok();

            std::thread::sleep(time);
        }
        if args.reverse.unwrap_or_else(|| true) {
            for i in -rend..=-rstart {
                let file = std::fs::File::create(&path).unwrap();
                let mut writer = BufWriter::new(file);
                println!("{}", &i.abs());
                writer.write_all((i.abs()).to_string().as_bytes()).ok();

                writer.flush().ok();

                std::thread::sleep(time);
            }
        }
    }
}
