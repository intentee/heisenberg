use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ContextualError<TContext, TSource> {
    pub context: TContext,
    pub source: TSource,
}

impl<TContext, TSource: Display> Display for ContextualError<TContext, TSource> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.source, formatter)
    }
}

impl<TContext: Debug, TSource: Error> Error for ContextualError<TContext, TSource> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.source()
    }
}
