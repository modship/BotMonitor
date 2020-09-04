extern crate reqwest;

use std::env;
use std::fs;
use std::time::{Duration, Instant};

// use webex::types::{Room, RoomsReply};
use job_scheduler::{Job, JobScheduler};
use log::{info, SetLoggerError};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use webex::error::Error;
use webex::types::Message;


fn generate_logger() -> Result<Handle, SetLoggerError> {
    let level = log::LevelFilter::Info;
    let file_path = "./survey.log";
    let pattern_log = "{d(%Y-%m-%d %H:%M:%S)} {l} - {m}\n";

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).encoder(Box::new(PatternEncoder::new(pattern_log))).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new(pattern_log)))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(level),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    log4rs::init_config(config)
}

struct Service {
    bot_token: String,
    name: String,
    url: String,
    room_id: String,
    schedule: String,
    last_value_sucess: bool,
    last_msg_time: Instant,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigJson {
    bot_token: String,
    name: String,
    url: String,
    room_id: String,
    schedule: String,
}

fn main() {
    // Get command line arg
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage => $ survey config.json");
    }

    // Read file conf
    let contents = fs::read_to_string(args[1].to_string())
        .expect("Something went wrong reading the file");
    let configs: Vec<ConfigJson> = serde_json::from_str(&contents).expect("Error parsing json file");

    // Create logger
    let result = generate_logger();
    match result {
        Ok(_handle) => info!("Starting survey ..."),
        Err(e) => panic!(e)
    }

    let mut sched = JobScheduler::new();

    for config in configs.iter() {
        let mut service = Service {
            name: config.name.to_owned(),
            bot_token: config.bot_token.to_owned(),
            url: config.url.to_owned(),
            room_id: config.room_id.to_owned(),
            schedule: config.schedule.to_owned(),
            last_value_sucess: true,
            last_msg_time: Instant::now(),
        };

        // Start Job
        &sched.add(Job::new(
            (&service.schedule).parse().unwrap(),
            move || {
                check_service(&mut service);
            },
        ));
    }

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}


#[tokio::main]
async fn check_service(service: &mut Service) {
    let mut msg = String::new();
    let result = request_to_service(service).await;

    match result {
        Ok(_r) => {
            if service.last_value_sucess == false {
                // La derniere fois il était offline , on notifie de la remise en service
                msg = format!("{} is now Online", service.name);
            }
            service.last_value_sucess = true;
            info!("{} is Online", service.name);
        }
        Err(e) => {
            if service.last_value_sucess == true {
                // La derniere fois il était online donc changement d'etat => notification
                msg = format!("{} is now Offline : {}", service.name, &e.to_string());
            } else if service.last_msg_time.elapsed().as_secs() >= 300 {
                // Si c'etait deja offline mais que le dernier message envoyé etait il y à 5 min
                msg = format!("{} is still Offline : {}", service.name, &e.to_string());
            }

            service.last_value_sucess = false;
            info!("Error with {} : {:#?}", service.name, e);
        }
    }

    // Si msg n'est pas vide on le transmet sur la room webex
    if msg.is_empty() == false {
        let send_response = send_msg_webex_room(service, msg).await;

        match send_response {
            Ok(msg) => {
                info!("Webex msg transmited : {:?}", msg.text.unwrap());
            }
            Err(e) => {
                info!("Webex msg error : {}", e);
            }
        }
        service.last_msg_time = Instant::now();
    }
}


async fn request_to_service(service: &Service) -> Result<(), Box<dyn std::error::Error>> {
    info!("{} is checking", service.name);
    let timeout = Duration::new(7, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    client.head(&service.url).send().await?;
    Ok(())
}

async fn send_msg_webex_room(service: &Service, msg: String) -> Result<Message, Error> {
    let webex = webex::Webex::new(&service.bot_token);
    let room = Some(String::from(&service.room_id));
    let msg_to_send = webex::types::MessageOut {
        // to_person_email: Some(to_email.parse().unwrap()),
        text: Some(msg),
        room_id: room,
        ..Default::default()
    };

    webex.send_message(&msg_to_send).await
}


//Get message from webex room
// #[tokio::main]
// async fn survey() {
//
//     let webex = webex::Webex::new(TOKEN);
//     let mut event_stream = webex.event_stream().await.expect("event stream");
//
//     while let Ok(event) = event_stream.next().await {
//         // Dig out the useful bit
//         if event.data.event_type.as_str() == "conversation.activity" {
//             if let Some(activity) = &event.data.activity {
//                 if activity.verb.as_str() == "post" {
//                     // The event stream doesn't contain the message -- you have to go fetch it
//                     if let Ok(msg) = webex.get_message(&activity.id.as_str()).await {
//                         println!("new msg");
//                         match &msg.person_email {
//                             // Reply as long as it doesn't appear to be our own message
//                             // In practice, this shouldn't happen since bots can't see messages
//                             // that don't specifically mention them (i.e., appears in the special
//                             // "mentions" field).
//                             Some(sender) if sender != email => {
//                                 let mut reply = webex::types::MessageOut::from(&msg);
//                                 reply.text =
//                                     Some(format!("{}, you said: {}", sender, msg.text.unwrap()));
//                                 webex.send_message(&reply).await.unwrap();
//                             }
//                             _ => (),
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
