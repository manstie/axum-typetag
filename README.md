# Axum Typetag MRE

This is a minimal reproducible example of an error experienced when integrating the `typetag` crate into an axum endpoint which uses diesel-deadpool.

### Setup

Install the recommended extension "Remote Development" in VSCode.

Run command <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>P</kbd>:
```
> Dev Container: Rebuild and Reopen in Container
```

### Running App

In the dev container:

```
cd code
cargo run
```

### Testing issue

The code is in the broken state in order to see the error when you `cargo run`.

It can be fixed by commenting out main.rs:92:
```diff
-   let conn = pool.get().await?;
+   // let conn = pool.get().await?;
```

Or it can be fixed by not serializing the typetag dyn trait and using an enum instead:
```diff
    // Using typetag, `conn` can not be present, else the route has an error
-   let queueable: Box<dyn Queueable> = Box::new(example.clone());
+   // let queueable: Box<dyn Queueable> = Box::new(example.clone());
    let conn = pool.get().await?;

    // Works with or without `conn`
-   // let queueable = NotTypeTag::ExampleJob(example);
+   let queueable = NotTypeTag::ExampleJob(example);
```

The error in full:
```
   Compiling axum_typetag v0.1.0 (/workspaces/axum-typetag/code)
error[E0277]: the trait bound `fn(axum::extract::State<Pool<Manager<diesel::pg::connection::PgConnection>>>, axum::extract::Path<u64>) -> impl Future<Output = Result<Json<Value>, AppError>> {test}: Handler<_, _>` is not satisfied
   --> src/main.rs:68:35
    |
68  |         .route("/test/:data", get(test))
    |                               --- ^^^^ the trait `Handler<_, _>` is not implemented for fn item `fn(axum::extract::State<Pool<Manager<diesel::pg::connection::PgConnection>>>, axum::extract::Path<u64>) -> impl Future<Output = Result<Json<Value>, AppError>> {test}`
    |                               |
    |                               required by a bound introduced by this call
    |
    = help: the following other types implement trait `Handler<T, S>`:
              <Layered<L, H, T, S> as Handler<T, S>>
              <MethodRouter<S> as Handler<(), S>>
note: required by a bound in `axum::routing::get`
   --> /usr/local/cargo/registry/src/index.crates.io-6f17d22bba15001f/axum-0.7.4/src/routing/method_routing.rs:385:1
    |
385 | top_level_handler_fn!(get, GET);
    | ^^^^^^^^^^^^^^^^^^^^^^---^^^^^^
    | |                     |
    | |                     required by a bound in this function
    | required by this bound in `get`
    = note: this error originates in the macro `top_level_handler_fn` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `axum_typetag` (bin "main") due to previous error
```
