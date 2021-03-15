use bastion::prelude::*;
#[derive(Debug)]
pub(crate) struct Ticket(pub u32);

#[derive(Debug)]
pub(crate) struct Add(pub Vec<Ticket>);

pub(crate) fn group_named<'a>(name: &'a str) -> impl Fn(Children) -> Children + 'a {
    move |children: Children| {
        children
            .with_name(format!("ticket_seller_{}", name))
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
            _: _ => ();
        }
    }
}
