use std::slice::{Iter, IterMut};
use tracing::level_filters::LevelFilter;
use tracing::span::{Attributes, Record};
use tracing::subscriber::Interest;
use tracing::{Event, Id, Metadata, Subscriber};
use tracing_subscriber::Layer;

pub struct DynLayerList<S>(Vec<Box<dyn Layer<S> + Send + Sync + 'static>>);

impl<S> DynLayerList<S> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn Layer<S> + Send + Sync>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Box<dyn Layer<S> + Send + Sync>> {
        self.0.iter_mut()
    }
}

impl<S> DynLayerList<S>
where
    S: Subscriber,
{
    pub fn add<L: Layer<S> + Send + Sync>(&mut self, layer: L) {
        self.0.push(Box::new(layer));
    }
}

impl<S> Layer<S> for DynLayerList<S>
where
    S: Subscriber,
{
    fn on_layer(&mut self, subscriber: &mut S) {
        self.iter_mut().for_each(|l| l.on_layer(subscriber));
    }

    fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
        // Return highest level of interest.
        let mut interest = Interest::never();
        for layer in &self.0 {
            let new_interest = layer.register_callsite(metadata);
            if (interest.is_sometimes() && new_interest.is_always())
                || (interest.is_never() && !new_interest.is_never())
            {
                interest = new_interest;
            }
        }
        interest
    }

    fn enabled(
        &self,
        metadata: &Metadata<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        self.iter().any(|l| l.enabled(metadata, ctx.clone()))
    }

    fn on_new_span(
        &self,
        attrs: &Attributes<'_>,
        id: &Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.iter()
            .for_each(|l| l.on_new_span(attrs, id, ctx.clone()));
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        self.iter().filter_map(|l| l.max_level_hint()).max()
    }

    fn on_record(
        &self,
        span: &Id,
        values: &Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.iter()
            .for_each(|l| l.on_record(span, values, ctx.clone()));
    }

    fn on_follows_from(
        &self,
        span: &Id,
        follows: &Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.iter()
            .for_each(|l| l.on_follows_from(span, follows, ctx.clone()));
    }

    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.iter().for_each(|l| l.on_event(event, ctx.clone()));
    }

    fn on_enter(&self, id: &Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.iter().for_each(|l| l.on_enter(id, ctx.clone()));
    }

    fn on_exit(&self, id: &Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.iter().for_each(|l| l.on_exit(id, ctx.clone()));
    }

    fn on_close(&self, id: Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.iter()
            .for_each(|l| l.on_close(id.clone(), ctx.clone()));
    }

    fn on_id_change(&self, old: &Id, new: &Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        self.iter()
            .for_each(|l| l.on_id_change(old, new, ctx.clone()));
    }
}
