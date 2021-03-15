use crate::ticket_seller;
use bastion::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct CreateEvent {
    pub name: String,
    pub tickets: u32,
}

#[derive(Debug)]
pub(crate) struct Event {
    pub name: String,
    pub tickets: u32,
}

#[derive(Debug)]
pub(crate) struct EventCreated;

#[derive(Debug)]
pub(crate) struct EventExists;

pub(crate) fn group(children: Children) -> Children {
    children
        .with_name("box_office")
        .with_redundancy(1)
        .with_exec(box_office)
}

async fn box_office(ctx: BastionContext) -> Result<(), ()> {
    // As of bastion 0.4.4 we can't dynamically launch more child actor
    // into a actor group, so we must create new groups and remember their name
    let mut ticket_sellers = HashMap::new();
    loop {
        msg! { ctx.recv().await?,
            msg: CreateEvent =!> {
                if ticket_sellers.contains_key(&msg.name) {
                    answer!(ctx, EventExists).expect("unable to send answer");
                } else {
                    let ticket_seller = ctx
                        .supervisor()
                        .expect("missing supervisor")
                        .children(ticket_seller::group_named(&msg.name))?;
                    let tickets = (1..=msg.tickets).map(ticket_seller::Ticket).collect();
                    ctx.tell(&ticket_seller.elems()[0].addr(), ticket_seller::Add(tickets))
                        .expect("unable to send message");
                    ticket_sellers.insert(msg.name, ticket_seller);
                    answer!(ctx, EventCreated).expect("unable to send answer");
                }
            };
            _: _ => ();
        }
    }
}
