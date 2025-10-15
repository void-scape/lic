use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args();
    let name = args.next().unwrap();
    match (args.next(), args.next()) {
        (Some(year), Some(holders)) => {
            let mut mit = false;
            let mut apache = false;

            for i in 0..2 {
                let next = args.next();
                if next.as_ref().is_some_and(|arg| arg == "mit") {
                    mit = true;
                } else if next.as_ref().is_some_and(|arg| arg == "apache") {
                    apache = true;
                } else if let Some(license) = next {
                    println!("{name}: Unknown license {license}");
                    return ExitCode::FAILURE;
                } else if i == 0 {
                    println!("Usage: lic year holders [mit] [apache]");
                    return ExitCode::FAILURE;
                }
            }
            if args.next().is_some() {
                println!("Usage: lic year holders [mit] [apache]");
                return ExitCode::FAILURE;
            }

            let year = year.parse().expect("failed to parse year");
            let postfix = mit && apache;
            if mit {
                let path = if postfix { "LICENSE-MIT" } else { "LICENSE" };
                std::fs::write(path, lic::generate_mit(year, &holders)).unwrap();
            }
            if apache {
                let path = if postfix { "LICENSE-APACHE" } else { "LICENSE" };
                std::fs::write(path, lic::generate_apache(year, &holders)).unwrap();
            }

            ExitCode::SUCCESS
        }
        _ => {
            println!("Usage: lic year holders [mit] [apache]");
            ExitCode::FAILURE
        }
    }
}
