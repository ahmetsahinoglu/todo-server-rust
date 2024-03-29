# todo-server-rust

todo-server-rust is a Rust(actix-web) application provides Rest APIs that uses create, update and delete operations.

## Installation
1. Download and install **Rust** from [here](https://www.rust-lang.org/en-US/downloads.html)
2. Download and install **Cargo** from [here](http://doc.crates.io/)

```bash 
git clone https://github.com/ahmetsahinoglu/todo-server-rust.git
cd todo-server-rust
```
##### Set your environment variable

```bash
$ export PORT=YOUR_PORT_NUMBER
```

##### How to start project in development mode

```bash
$ cargo run
```


##### How to build for production

```bash
$ cargo build --release
```


##### How to run unit tests.

```bash
$ cargo test
```

### API List

* Get TodoList http:127.0.0.1:{PORT}/v1/todo-list GET
* Create Todo  http:127.0.0.1:{PORT}/v1/todo-list POST
* Update Todo  http:127.0.0.1:{PORT}/v1/todo-list/{id} PATCH
* Delete Todo  http:127.0.0.1:{PORT}/v1/todo-list/{id} DELETE

###USAGE

#### Get TodoList
```
curl --location --request GET 'http:127.0.0.1:8080/v1/todo-list'
```

#### Create Todo
```
curl --location --request POST 'http:127.0.0.1:8080/v1/todo-list' \
--header 'Content-Type: application/json' \
--data-raw '{
    "text": "Pay rent.",
    "status": "ACTIVE"
}'
```

#### Update Todo
```
curl --location --request PUT 'localhost:8080/v1/todo-list/1' \
--header 'Content-Type: application/json' \
--data-raw '{
    "text": "Pay rent.",
    "status": "DONE"
}'
```

#### Delete Todo
```
curl --location --request DELETE 'localhost:8080/v1/todo-list/1'
```

## Sample Response
```json
[
  {
    "id": 1,
    "text": "Pay rent.",
    "status": "DONE"
  },
  {
    "id": 2,
    "text": "Prepare suitcase.",
    "status": "ACTIVE"
  }
]
```