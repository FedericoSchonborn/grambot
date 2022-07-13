use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tokio_stream::Stream;

use crate::{methods::GetUpdates, types::Update, Bot, Error};

#[allow(dead_code)]
pub struct UpdateStream<'bot> {
    bot: &'bot Bot,
    request: GetUpdates,
    #[allow(clippy::type_complexity)]
    future: Option<Pin<Box<dyn Future<Output = Result<VecDeque<Update>, Error>>>>>,
    updates: VecDeque<Update>,
}

impl<'bot> UpdateStream<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot, request: GetUpdates) -> UpdateStream<'bot> {
        Self {
            bot,
            request,
            future: None,
            updates: VecDeque::new(),
        }
    }
}

impl<'bot> Stream for UpdateStream<'bot> {
    type Item = Result<Update, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        enum Status {
            Ready,
            Pending,
            Err(Error),
        }

        let this = self.get_mut();

        loop {
            let status = if let Some(ref mut future) = this.future {
                match future.as_mut().poll(cx) {
                    Poll::Ready(result) => {
                        this.future = None;

                        match result {
                            Ok(updates) => {
                                this.updates = updates;
                                Status::Ready
                            }
                            Err(err) => Status::Err(err),
                        }
                    }
                    Poll::Pending => Status::Pending,
                }
            } else {
                Status::Ready
            };

            match status {
                Status::Err(err) => return Poll::Ready(Some(Err(err))),
                Status::Pending => return Poll::Pending,
                Status::Ready if !this.updates.is_empty() => {
                    let update = this.updates.pop_front().unwrap();
                    *this.request.offset.get_or_insert(0) += 1;

                    return Poll::Ready(Some(Ok(update)));
                }
                Status::Ready => {
                    todo!()

                    // TODO(FedericoSchonborn): E0495 here.
                    // this.future = Some(Box::pin(this.bot.send(this.request)));
                }
            }
        }
    }
}
