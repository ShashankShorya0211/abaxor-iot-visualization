use actix_files::Files;
use actix_web::{App, HttpServer};
use env_logger::Env;
use colored::Colorize;


mod reg_bank;
mod reg_fifo;
mod fifo_stream;
mod macros;
mod settings;
mod flags;
mod sys_log;
mod oscillator;

// API handlers
mod handler;
mod models;
mod parser;

fn hardware_init() {
    // Syslog init
    println!("-- syslog init");
    sys_log::init();

    // Open register bank and FIFO
    println!("-- reg_bank init");
    let reg = reg_bank::init();
    
    println!("-- reg_fifo init");
    let fifo = reg_fifo::init();

    // Initialize FIFO
    println!("-- fifo init");
    fifo_stream::init_fifo(fifo, reg);

    println!("--oc st");
    // Initialize oscillators
    for ch in 0..settings::ANZAHL_AN_OSZILLATOREN {
        oscillator::init(reg, ch).unwrap_or_else(|_| {
            eprintln!("❌ Failed to init oscillator on channel {}", ch);
        });
    }

    println!("✅ Hardware initialization complete.");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("🚀 Initializing hardware...");
    hardware_init();

    println!("🚀 Actix server running on http://192.168.1.164:433");

    HttpServer::new(|| {
        App::new()
            // API routes from your handler.rs
            .configure(handler::config)
            // Serve frontend from NGINX-mounted directory
            .service(Files::new("/", "/var/www/html").index_file("index.html"))
    })
    .bind(("0.0.0.0", 433))?
    .run()
    .await
}

