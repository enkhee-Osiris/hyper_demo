# Warp demo

Personally wanted to try out warp.

Codes are from [warp-examples](https://github.com/seanmonstar/warp/tree/master/examples)

## Structure

```
├── helpers
│   └── mod.rs
├── main.rs
├── models
│   ├── mod.rs
│   └── todo.rs
├── mod.rs
├── routes
│   ├── mod.rs
│   └── todo.rs
├── services
│   ├── mod.rs
│   └── todo.rs
└── tests
```

`Models` are for models.  
`Routes` are for apis.  
`Services` are for handlers.  
`Helpers` are for utils etc.  
`tests` are for tests.

## How to try out

Clone and run project then try CRUD (curls are below).

To get todos list.

```sh
$ curl -H "Accept: application/json" http://localhost:3030/todos
```

To create new todo.

```sh
$ curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "text": "lorem ipsum", "completed": false}' http://localhost:3030/todos
```

To update todo.

```sh
$ curl -X PUT -H "Content-Type: application/json" -d '{"id":1, "text":"lorem ipsum", "completed": true}' http://localhost:3030/todos/1
```

To delete todo.

```sh
$ curl -X DELETE -H "authorization: Bearer admin" http://localhost:3030/todos/1
```
