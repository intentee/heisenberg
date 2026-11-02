use crate::contextual_error::ContextualError;

pub fn error_context<TContext, TSource, TError>(
    context: TContext,
    variant: fn(ContextualError<TContext, TSource>) -> TError,
) -> impl FnOnce(TSource) -> TError {
    move |source| variant(ContextualError { context, source })
}
