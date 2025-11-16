use std::{env::args, process::{Command, exit}};

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{}: Usage: {} URL", args[0], args[0]);
        exit(1);
    }

    // Safety: Checks if curl exists or not.
    curl_check(&args[0]);

    // Gets link info with curl
    let url = args[1].clone();
    let curled = Command::new("curl")
        .arg("-sI")
        .arg(url)
        .output()
        .expect("failed to run curl");
    let stdout = String::from_utf8_lossy(&curled.stdout);

    // Determines file size.
    let content_size = stdout.lines()
        .find_map(|line| {
            if line.to_lowercase().starts_with("content-length:") {
                line.split(':')
                    .nth(1)
                    .map(|v| v.trim().parse::<u128>().ok())
                    .flatten()
            } else {
                None
            }
        });
    
    let size;
    match content_size {
        Some(s) => size = s,
        None => {
            eprintln!("{}: Error: Cannot get file size", args[0]);
            exit(3);
        }
    }

    // Makes the number human readable
    let human_readable_size;
    match size {
        0..1024 => human_readable_size = format!("{size}B"),

        1024..1_048_576 => {
            let fract = (size % 1024) * 10 / 1024;
            let size = size / 1024;
            match fract {
                0 => human_readable_size = format!("{size}KiB"),
                n => human_readable_size = format!("{size}.{n}KiB")
            }
        },

        1_048_576..1_073_741_824 => {
            let fract = (size % 1_048_576) * 10 / 1_048_576;
            let size = size / 1_048_576;
            match fract {
                0 => human_readable_size = format!("{size}MiB"),
                n => human_readable_size = format!("{size}.{n}MiB")
            }
        },

        _ => {
            let fract = (size % 1_073_741_824) * 10 / 1_073_741_824;
            let size = size / 1_073_741_824;
            match fract {
                0 => human_readable_size = format!("{size}GiB"),
                n => human_readable_size = format!("{size}.{n}GiB")
            }
        }

    }

    // Output
    println!("{human_readable_size}");

}



fn curl_check(app_name: &str) {
    let check = Command::new("curl").arg("--version").output();
    match check {
        Err(_) => {
            eprintln!("{app_name}: Error: `curl` is not installed or not in your PATH.");
            exit(2);
        },
        Ok(_) => {}
    }
}
