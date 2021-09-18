use std::env;
use rand::Rng;
use warp::Filter;

const RUST_LOG: &str = "RUST_LOG";
const ENV: &str = "ENV";

/// Format the small html body with the word / emoji that you want to input. By default arg is a heart
fn format_html() -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(127538..129510);

    format!("
        <html>
            <head>
                <title>Hello !</title>
                <meta charset='UTF-8'>
                <link rel='preconnect' href='https://fonts.googleapis.com'>
                <link rel='preconnect' href='https://fonts.gstatic.com' crossorigin>
                <link href='https://fonts.googleapis.com/css2?family=Playfair+Display&display=swap' rel='stylesheet'>
            </head>
            <body style='text-align: center; background-color: #0C031C'>
                <h1 style='margin-top: 20%; color: white; font-size: 45px; font-family: Playfair Display;'>Coding & DevOps = &#{}</h1>
            </body>
        </html>
    ", number)
}

/// Setup the logger
fn setup() {
    if env::var_os(RUST_LOG).is_none() {
        env::set_var(RUST_LOG, "info");
    }

    env_logger::init();
}

#[tokio::main]
async fn main() {
    let mut addr: [u8; 4] = [127, 0, 0, 1];
    let default = warp::any()
        .map(|| {
            let body = format_html();
            warp::reply::html(body)
        });

    // setup the loggr
    setup();

    // log something...
    log::info!("Starting server !");

    if let Some(env) = env::var_os(ENV) {
        if env == "prod" {
            addr = [0, 0, 0, 0];
        }
    }

    // starting the server
    warp::serve(warp::get().and(default))
        .run((addr, 3000))
        .await;
}

#[cfg(test)]
mod tests {
    use crate::format_html;

    #[test]
    fn expect_to_return_html() {
        let html = format_html();
        assert!(!html.is_empty());
    }
}