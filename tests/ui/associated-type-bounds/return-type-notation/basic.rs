// revisions: with without
// edition: 2021
//[with] check-pass

#![feature(return_type_notation, async_fn_in_trait)]
//~^ WARN the feature `return_type_notation` is incomplete
//~| WARN the feature `async_fn_in_trait` is incomplete

trait Foo {
    async fn method() -> Result<(), ()>;
}

async fn foo<T: Foo>() -> Result<(), ()> {
    T::method().await?;
    Ok(())
}

fn is_send(_: impl Send) {}

fn test<
    #[cfg(with)] T: Foo<method(..): Send>,
    #[cfg(without)] T: Foo,
>() {
    is_send(foo::<T>());
    //[without]~^ ERROR future cannot be sent between threads safely
}

fn main() {}
