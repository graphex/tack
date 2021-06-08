use actix::prelude::*;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
struct SendTempestDatum {
    pub tempest_message: TempestMessage,
}

struct SenderActor;

impl Actor for SenderActor {
    type Context = AsyncContext<SenderActor>;

}