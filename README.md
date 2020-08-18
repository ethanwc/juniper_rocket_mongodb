# Rust Juniper Rocket GraphQL MongoDB Example

Are you trying to figure out how to use Juniper with Rocket and MongoDB?

Clone this project and follow the instructions below and start playing today! Unlike the [Rocket example](https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/examples/rocket_server.rs) in the Juniper repo, it can be built without relying on types defined in Juniper tests.

## Running the project

Since Rocket uses the latest and greatest features of Rust, you need to use a nightly version of rust. As per the [Rocket documentation](https://rocket.rs/guide/getting-started/), you can do this using

```bash
$ rustup default nightly
```

or per project basis

```bash
$ rustup override set nightly
```

Now you need to provide a mongodb uri to the connection method in create_mongo_client, and specify the collection name where the db interacts.

Lastly, you can run the server using

```bash
$ cargo run
```

If all goes well, you will be able to visit http://localhost:8000/ with a graphiql interface.

You can then query / mutate with the following:

Query 

```graphql
{
  human (id: "human-id") {
    id
    name
  }
}

```

Mutation

```graphql
mutation{
  createHuman(
    newHuman: {
      id: "human-id",
      name: "Steve Jobs"
    }
  ){
    id
    name
  }
}
```

## Attributions

I went off [this example](https://github.com/martimatix/Rust-Juniper-Rocket-GraphQL-Example) by martimatix. Hopefully this helps you, I found the process poorly documented and hope this is an easier starting experience.
# juniper-mongo
