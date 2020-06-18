#[macro_use]

extern crate serde;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::{task, time};

// use std::sync::mpsc::channel;
// use std::thread;

use std::env;
use rumq_client::{eventloop, MqttOptions, Publish, QoS, Request};
use std::time::Duration;
use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;

extern crate env_logger;
use log::{info, error};

mod model;

#[tokio::main]
async fn main() {
  env_logger::init();

  let address = env::var("MQTT_BROKER_ADDRESS").unwrap();
  let port = env::var("MQTT_BROKER_PORT").unwrap().parse::<u16>().unwrap();

  info!("Starting client. Host at {}:{}", address.clone(), port.clone());

  let count = get_lines();
  
  let (mut requests_tx, requests_rx) = channel(50);
  let mut eloop;
  let mut mqttoptions = MqttOptions::new("client", address.clone(), port);
  mqttoptions.set_keep_alive(5).set_throttle(Duration::from_secs(1));
  eloop = eventloop(mqttoptions, requests_rx);


  let tx_c = requests_tx.clone();
  for i in 0..count {
    let topic = get_topic(i);
    let coordinates = assort_coordinates(i);
    
    let tx = tx_c.clone();
    task::spawn(async move {
      info!("Thread {}", topic.clone());

      let api_address = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", 
        coordinates.lat.to_string(),
        coordinates.lon.to_string(),
        "f02ef3d664e7ad0b33020a0ecbf896b2"
      );

      let mut weather: model::Weather;
      loop {
        weather = reqwest::get(&api_address.clone()).await.unwrap().json().await.unwrap();
        let payload = (weather.condition.temp - 273.0).to_string();
        
        for _i in 0..5 {
          tx.clone().send(publish_request(&payload, &topic.clone())).await.unwrap();
          info!("{}:{}", payload, topic.clone());
          time::delay_for(Duration::from_millis(100)).await;
        }
      }
    });
  }

  let mut stream = eloop.connect().await.unwrap();
  // Sim , isso é muito feio mas ainda nao descobri outro jeito de fazer isso -R
  while let Some(_item) = stream.next().await {}
}

fn publish_request(payload: &str, topic: &str) -> Request {
  let topic = topic.to_owned();
  let message = String::from(payload);

  let publish = Publish::new(&topic, QoS::AtLeastOnce, message);
  Request::Publish(publish)
}

fn assort_coordinates(mode: usize) -> model::Coords{
  let coordinates: model::Coords;

  match mode {
    0 => { // Porto Alegre
      coordinates = model::Coords{ lat: -30.0331, lon: -51.2287 };
    }
    1 => { // Dublin
      coordinates = model::Coords{ lat: 53.34, lon: -6.26 };
    }
    2 => { // Tokyo
      coordinates = model::Coords{ lat: 35.6894, lon: 139.6920 };
    }
    3 => { // Moscou
      coordinates = model::Coords{ lat: 55.7508, lon: 37.6172 };
    }
    4 => { // Sydney
      coordinates = model::Coords{ lat: -33.8679, lon: 151.2073 };
    }
    5 => { // Salvador
      coordinates = model::Coords{ lat: -12.97, lon: -38.47 };
    }
    6 => { // Oslo
      coordinates = model::Coords{ lat: 59.91, lon: 10.73 };
    }
    _ => { return assort_coordinates(0); }
  }

  return coordinates;
}

fn get_topic(line_num: usize) -> String {
  let f = File::open("topic_names.txt").expect("Open topic file");
  let buffer = BufReader::new(f);
  let mut topic = String::new();

  let mut counter = 0;

  for line in buffer.lines() {
    if counter == line_num {
      topic = line.unwrap();
    }
    counter += 1;
  }

  topic
}

fn get_lines() -> usize {
  let f = File::open("topic_names.txt").expect("Open topic file");
  let buffer = BufReader::new(f);

  buffer.lines().count()
}