#![feature(plugin)]

#[macro_use]

extern crate serde;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{channel, Sender};
use tokio::{task, time};
use std::env;
use rumq_client::{eventloop, MqttEventLoop, MqttOptions, Publish, QoS, Request};
use std::time::Duration;

mod model;

#[tokio::main(basic_scheduler)]
async fn main() {

    let args:Vec<String> = env::args().collect();

    let mut port = "1883";
    let mut mode = "1";


    match args.len() - 1 {

        1=>{
            port = &args[1];
            }

        2=>{
            port = &args[1];
            mode = &args[2];
            }


        _ =>{
            println!("invalid number of arguments");
             }
    }

  
    let (requests_tx, requests_rx) = channel(10);
    let port = port.parse::<u16>().unwrap();
    let mut mqttoptions = MqttOptions::new("CLIENT", "localhost", port);
    mqttoptions.set_keep_alive(5).set_throttle(Duration::from_secs(1));
    let mut eventloop = eventloop(mqttoptions, requests_rx);

    let coordinates = assort_coordinates(&mode);
    let adress = format!("http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid=f02ef3d664e7ad0b33020a0ecbf896b2",coordinates.lat.to_string(),coordinates.lon.to_string());

    task::spawn(async move {
        requests(requests_tx, adress).await;
        time::delay_for(Duration::from_secs(3)).await;
    });
    

    stream_it(&mut eventloop).await;


}

async fn stream_it(eventloop: &mut MqttEventLoop) {
    let mut stream = eventloop.connect().await.unwrap();

    //Sim , isso é muito feio mas ainda nao descobri outro jeito de fazer isso -R

    while let Some(_item) = stream.next().await {}

}

async fn requests(mut requests_tx: Sender<Request>,adress : String)  ->  Result<(), Box<dyn std::error::Error>>  {
        let mut weather : model::Weather;
     loop{
        weather = reqwest::get(&adress).await?.json().await?; 
        let payload = format!("Weather in {} is {:.4}" ,weather.name,(weather.condition.temp-273.0).to_string());   
        requests_tx.send(publish_request(&payload)).await.unwrap();
        time::delay_for(Duration::from_secs(10)).await;
        }
   
}

fn publish_request(payload : &str) -> Request {

    let topic = "TEST".to_owned();
    let message = String::from(payload);
 
    let publish = Publish::new(&topic, QoS::AtLeastOnce,message);
    Request::Publish(publish)
}


fn assort_coordinates(mode : &str) -> model::Coords{    
let coordinates : model::Coords;
    match mode {

        "1" =>
        {//Porto Alegre
        coordinates = model::Coords{lat : -30.0331 , lon :-51.2287};
        // println!{"lat {} lon {}",coordinates.lat,coordinates.lon};
        }

        "2" =>
        {//Dublin
        coordinates = model::Coords{lat : 53.34 , lon :-6.26};
        // println!{"lat {} lon {}",coordinates.lat,coordinates.lon};    
        }

        "3" =>
        {//Tokyo
        coordinates = model::Coords{lat : 35.6894 , lon :139.6920};
       // println!{"lat {} lon {}",coordinates.lat,coordinates.lon};    
        }

        "4"=>
        {//Moscou
        coordinates = model::Coords{lat : 55.7508 , lon :37.6172};
       // println!{"lat {} lon {}",coordinates.lat,coordinates.lon};    
        }

        _ =>
        {//Sydney
        coordinates = model::Coords{lat : -33.8679 , lon :151.2073};
       // println!{"lat {} lon {}",coordinates.lat,coordinates.lon};    
        }       
    }
    
    return coordinates;
}



 
