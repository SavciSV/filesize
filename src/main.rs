use std::{env::args, process::{Command, exit}};

fn main() {
    let args = args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{}: Usage: {} URL", args[0], args[0]);
        exit(1);
    }

    // Safety: Checks if curl exists or not.
    curl_check(&args[0]);

    // Gets file size
    let size = get_size(&args[0], &args[1]);

    // Output
    println!("{size}");
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

fn get_content_length(url: &str) -> Option<u128> {
    let mut url = url.to_owned();
    loop {
        // Gets link info with curl.
        let curled = Command::new("curl")
            .arg("-sI")
            .arg(url)
            .output()
            .expect("failed to run curl");
        let stdout = String::from_utf8_lossy(&curled.stdout);

        match stdout.lines()
            .find(|line| line.split_once(':')
                .map(|(h, _)| h.trim().eq_ignore_ascii_case("location"))
                .unwrap_or(false))
        {
            // Reaches the final direct link if not provided.
            Some(location) => {
                let (_, value) = location.split_once(':').unwrap();
                url = value.trim().to_owned();
                continue;
            },
            // If no 'location' exists, then this is the final link so it returns its size.
            None => {
                break stdout.lines().find_map(|line| {
                    if let Some((header, value)) = line.split_once(':') {
                        if header.trim().eq_ignore_ascii_case("content-length") {
                            return value.trim().parse().ok();
                        }
                    }
                    None
                })
            }
        }

        
    }
}

fn get_size(app_name: &str, url: &str) -> String {
    let content_size = get_content_length(url);
    let size;
    match content_size {
        Some(s) => size = s,
        None => {
            eprintln!("{}: Error: Cannot get file size", app_name);
            exit(3);
        }
    }

    // Makes the number human readable
    match size {
        0..1024 => format!("{size}B"),

        1024..1_048_576 => {
            let fract = (size % 1024) * 10 / 1024;
            let size = size / 1024;
            match fract {
                0 => format!("{size}KiB"),
                n => format!("{size}.{n}KiB")
            }
        },

        1_048_576..1_073_741_824 => {
            let fract = (size % 1_048_576) * 10 / 1_048_576;
            let size = size / 1_048_576;
            match fract {
                0 => format!("{size}MiB"),
                n => format!("{size}.{n}MiB")
            }
        },

        _ => {
            let fract = (size % 1_073_741_824) * 10 / 1_073_741_824;
            let size = size / 1_073_741_824;
            match fract {
                0 => format!("{size}GiB"),
                n => format!("{size}.{n}GiB")
            }
        }
    }
}
