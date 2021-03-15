use crate::{box_office, event_marshalling::EventDescription};
use bastion::prelude::*;
use tide::StatusCode;

#[derive(Debug)]
pub(crate) struct CreateEvent(pub String, pub EventDescription);

pub(crate) fn sp(sp: Supervisor) -> Supervisor {
    sp
}

pub(crate) fn group(children: Children) -> Children {
    children
        .with_name("rest_api")
        .with_redundancy(1)
        .with_exec(rest_api)
}

async fn rest_api(ctx: BastionContext) -> Result<(), ()> {
    let box_office = ctx
        .supervisor()
        .expect("missing supervisor")
        .children(box_office::group)?;
    loop {
        msg! { ctx.recv().await?,
            msg: CreateEvent =!> {
                let answer = ctx.ask(&box_office.elems()[0].addr(), box_office::CreateEvent {
                    name: msg.0,
                    tickets: msg.1.tickets,
                })
                .expect("unable to send message");
                msg! { answer.await.expect("unable to recv answer"),
                    msg: box_office::EventCreated => {
                        answer!(ctx, StatusCode::Created)
                            .expect("unable to answer message");
                    };
                    msg: box_office::EventExists => {
                        answer!(ctx, StatusCode::NoContent)
                            .expect("unable to answer message");
                    };
                    _: _ => ();
                }
            };
            _: _ => ();
        }
    }
}
