## Sample usage

```rust
#[tokio::main]
async fn main()
{
    let mut zlib: Zlibrary = Zlibrary::new();
    zlib.login(Some("username".to_string()), Some("password".to_string()), None, None).await;

    // Search and wait for results.
    let result = zlib.search("book name".to_string(), 1).await;
    println!("{:?}", result);
    if result.is_ok() {
        let result = result.unwrap();
        let book = result["books"][0].as_object().unwrap();

        let author = book["author"].as_str().unwrap().to_string();
        let hash_id = book["hash"].as_str().unwrap().to_string();
        let title = book["title"].as_str().unwrap().to_string();
        let book_id = book["id"].as_str().unwrap().to_string();

        let result = zlib.get_book_file(book_id, hash_id).await;
        println!("{:?}", result);
    }
}
```
