use actix::prelude::*;
use std::time::Duration;
use std::env;
use crate::tempest_messages::*;
use reqwest;
use crate::LprConvertable;

#[derive(Debug, Default)]
pub struct SenderActor;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendTempestDatum {
    pub tempest_message: TempestMessage,
}
impl Actor for SenderActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("SenderActor Started");
    }
}

// these allow the SenderActor::from_registry() to work
impl Supervised for SenderActor {}
impl SystemService for SenderActor {}

impl Handler<SendTempestDatum> for SenderActor {
    type Result = ();

    fn handle(&mut self, msg: SendTempestDatum, ctx: &mut Self::Context) {
        // actix_web::rt::spawn(async move {
            println!("SenderActor getting ready to send:\n{}", msg.tempest_message.to_lpr().to_string());
            let influx_host = env::var("INFLUX_HOST").expect("INFLUX_HOST is not set");
            let influx_token = env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN is not set");
            let influx_org_id = env::var("INFLUX_ORG_ID").expect("INFLUX_ORG_ID is not set");

            // Create a client
            let client = reqwest::Client::new();

            // Set up the request body
            let body = msg.tempest_message.to_lpr().to_string();


            // "https://influxdb.gfx.tc/api/v2/write?org=$INFLUX_ORG_ID&bucket=tempest&precision=s" \
            // --header "Authorization: Token $INFLUX_TOKEN" \
            // --header "Content-Type: text/plain; charset=utf-8" \
            // --header "Accept: application/json" \
            // --data-binary 'RapidWind,serial_number=ST-00029213,hub_sn=HB-00022471 wind_speed=1.72,wind_direction=28 1670209024'

            // Send a POST request to the specified URL over TLS
            let response = client.post(format!("https://{}/api/v2/write?org={}&bucket=tempest&precision=s", influx_host, influx_org_id))
                .body(body)
                .header("Authorization", format!("Token {}", influx_token))
                .header("Content-Type", "text/plain; charset=utf-8")
                .header("Accept", "application/json")
                .send()
                .into_actor(self);
                // .then(|res, slf, ctx| {
                //     println!("Result Received");
                //     println!("Result: {}",res.unwrap().status());
                //     fut::ready(())
                // });
                // .map(|r, _, _| r.unwrap().text()) // Convert the response to a Result<String, Error>
                // .map_err(|e, _, _| panic!("request failed: {}", e)); // Panic if the request fails
                ctx.spawn(response.then(|result, _, _| {
                    match result {
                        Ok(response) => {
                            // Print the response body
                            println!("Response body: {:?}", response.status());
                        }
                        Err(e) => {
                            // Print the error message
                            println!("Error: {}", e);
                        }
                    }
                    fut::ready(())
                }));

                // .expect("request failed");
            //
            // // Check the status code of the response
            // assert_eq!(response.status(), 200);
            //
            // // Read the response body
            // let body = response.text().expect("response body could not be read");
            //
            // // Print the response body
            // println!("Response body: {}", body);
        println!("SenderActor send completed");

        // })
    }
}