use crate::box_office::Event;
use bastion::prelude::*;
#[derive(Debug)]
pub(crate) struct Ticket(pub u32);

#[derive(Debug)]
pub(crate) struct Add(pub Vec<Ticket>);

#[derive(Debug)]
pub(crate) struct Buy(pub u32);

#[derive(Debug)]
pub(crate) struct Tickets(pub Vec<Ticket>);

#[derive(Debug)]
pub(crate) struct GetEvent;

#[derive(Debug)]
pub(crate) struct Cancel;

pub(crate) fn group_named<'a>(name: &'a str) -> impl Fn(Children) -> Children + 'a {
    move |children: Children| {
        children
            .with_name(name)
            .with_redundancy(1)
            .with_exec(ticket_seller)
    }
}

async fn ticket_seller(ctx: BastionContext) -> Result<(), ()> {
    let mut tickets = vec![];
    loop {
        msg! { ctx.recv().await?,
            msg: Add => {
                let mut msg = msg;
                tickets.append(&mut msg.0);
            };
            msg: Buy =!> {
                let nr_of_tickets = msg.0 as usize;
                if tickets.len() >= nr_of_tickets {
                    let entries = tickets.split_off(nr_of_tickets);
                    answer!(ctx, Tickets(entries))
                        .expect("unable to send message");
                } else {
                    answer!(ctx, Tickets(vec![]))
                        .expect("unable to send message");
                }
            };
            msg: GetEvent =!> {
                let event = ctx.current().name();
                answer!(ctx, Event { name: event.to_owned(), tickets: tickets.len() as u32 })
                    .expect("unable to send message");
            };
            msg: Cancel =!> {
                let event = ctx.current().name();
                answer!(ctx, Event { name: event.to_owned(), tickets: tickets.len() as u32 })
                    .expect("unable to send message");
                ctx.current().stop().expect("unable to send message");
            };
            _: _ => ();
        }
    }
}
