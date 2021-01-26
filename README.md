# joke-web-service

## Overview
- Combine two web services for retrieving a name and a random joke. 
- Input the name into the random joke.
- Post the joke to the provided uri (`http://localhost:5000"`).

## Code 
- The current code handles asynchronous, concurrent requests.
- However, the web service to handle requests to the post uri is **not setup**.
```
Usage:
	joke_collector [options]

Options:
	--num-requests=<n>   Number of concurrent requests [default: 1].
	-h, --help           Show this screen.
	--version            Show version.
```

- Once the web service is setup, the following can be used to post jokes to the post uri:
```
let client = Client::new();
let _joke_post_body = post_body(client, joke.to_string()).await;
```

## Sample Run
```
$ cargo build && ./target/debug/joke_collector --num-requests 4
    Finished dev [unoptimized + debuginfo] target(s) in 0.57s

Joke to post: "Jianfeng Balanoff doesn\'t need garbage collection because he doesn\'t call .Dispose(), he calls .DropKick()."
Joke to post: "The class object inherits from Baldev Shankin"
Joke to post: "When Dorys Caridine is web surfing websites get the message &quot;Warning: Internet Explorer has deemed this user to be malicious or dangerous. Proceed?&quot;."
Joke to post: "The class object inherits from Maria Hirleman"
```

## References
- [Rustlang Async Book: await](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html)
- [Attribute Macro tokio::main](https://docs.rs/tokio/1.0.2/tokio/attr.main.html)
- [Crate docopt](https://docs.rs/docopt/1.1.0/docopt/)
- [Crate reqwest](https://docs.rs/reqwest/0.10.7/reqwest/)
- [Futures StreamExt: for_each_concurrent](https://docs.rs/futures-preview/0.3.0-alpha.18/futures/stream/trait.StreamExt.html#method.for_each_concurrent)
- [Reqwest form example](https://github.com/seanmonstar/reqwest/blob/master/examples/form.rs)

## Next Steps
- I will try to carve out time to implement the **TODO** comments and add unit tests.
