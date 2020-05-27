
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{channel, Sender};
use tokio::{task, time};
//use rand::{thread_rng, Rng};

use rumq_client::{eventloop, MqttEventLoop, MqttOptions, Publish, QoS, Request};
use std::time::Duration;

#[tokio::main(basic_scheduler)]
async fn main() {

    let (requests_tx, requests_rx) = channel(10);
    let mut mqttoptions = MqttOptions::new("CLIENT", "localhost", 1883);

    mqttoptions.set_keep_alive(5).set_throttle(Duration::from_secs(1));
    let mut eventloop = eventloop(mqttoptions, requests_rx);
    task::spawn(async move {
        requests(requests_tx).await;
        time::delay_for(Duration::from_secs(3)).await;
    });

    stream_it(&mut eventloop).await;
}

async fn stream_it(eventloop: &mut MqttEventLoop) {
    let mut stream = eventloop.connect().await.unwrap();

    //Sim , isso é muito feio mas ainda nao descobri outro jeito de fazer isso -R

    while let Some(_item) = stream.next().await {}

}

async fn requests(mut requests_tx: Sender<Request>) {

    loop 
    {
        requests_tx.send(publish_request()).await.unwrap();
        time::delay_for(Duration::from_secs(1)).await;
    }

    
}

fn publish_request() -> Request {
    let topic = "TEST".to_owned();
    let payload = "Mensagem";

    let publish = Publish::new(&topic, QoS::AtLeastOnce, payload);
    Request::Publish(publish)
}