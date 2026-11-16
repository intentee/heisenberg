use crate::heisenberg_check_error::HeisenbergCheckError;

pub fn failure_in<TContext, TSource>(
    context: TContext,
    failure: fn(TContext, TSource) -> HeisenbergCheckError,
) -> impl FnOnce(TSource) -> HeisenbergCheckError {
    move |source| failure(context, source)
}
