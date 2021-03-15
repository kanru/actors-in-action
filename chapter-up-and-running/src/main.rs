use bastion::prelude::*;
use tide::Request;

mod box_office;
mod event_marshalling;
mod rest_api;
mod ticket_seller;

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    Bastion::init();
    Bastion::start();

    let rest_api = Bastion::supervisor(rest_api::sp)
        .expect("Unable to create supervisor")
        .children(rest_api::group)
        .expect("Unable to create actor group");
    let mut app = tide::with_state(rest_api);
    app.at("/events/:event").post(create_event);
    run! {
        app.listen("127.0.0.1:8000").await.expect("service failed");
    }

    Bastion::stop();
    Bastion::block_until_stopped();
}

async fn create_event(mut req: Request<ChildrenRef>) -> tide::Result {
    let rest_api = req.state().clone();
    let name = req.param("event")?.to_owned();
    let event_description = req.body_json().await?;
    let answer = rest_api.elems()[0]
        .ask_anonymously(rest_api::CreateEvent(name, event_description))
        .expect("unable to send message");
    msg! { answer.await.expect("unable to recv message"),
        status: tide::StatusCode => Ok(status.into());
        _: _ => Ok(tide::StatusCode::InternalServerError.into());
    }
}
