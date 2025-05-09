# sjabloon

sjabloon is a modern templating engine for Rust with a JSX-like syntax. writing
templates is comparable to writing JSX on the server.

you can define templates using the `template` macro. this macro will be
transpiled to a `format` macro call at build-time, which will result in a
string at run-time. "components" are just functions/closures which return a
string.

- use braced blocks to write arbitrary Rust code
- braces can be omitted for attribute values
- quoted and unquoted text nodes
- fragments

```rust
fn article(article: Article) -> String {
    template! {
        <a href=format!("articles/{}", article.slug)>
            <article>
                <img
                    src=article.cover.url
                    alt=article.cover.alt
                    width="400"
                    height="200"
                />
                <h2>{article.title}</h2>
                <small>author: {article.author}</small>
            </article>
        </a>
    }
}

fn articles_list(articles: Vec<Article>) -> String {
    template! {
        <section>
            <h1>articles</h1>
            <ul>
                {articles.into_iter().map(article).collect::<Vec<String>>().join("")} 
            </ul>
        </section>
    }
}
```
