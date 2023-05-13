// Take a look at the generated `cornucopia.rs` file if you want to
// see what it looks like under the hood.
use codegen::{
    client::Params,
    queries::{
        module_1::insert_book,
        module_2::{
            author_name_by_id, author_name_starting_with, authors, books, select_translations,
            select_voice_actor_with_character, AuthorNameStartingWithParams,
        },
    },
    types::public::SpongeBobCharacter,
};

pub fn main() {
    // You can learn which database connection types are compatible with Cornucopia in the book
    // https://cornucopia-rs.netlify.app/book/using_queries/db_connections.html
    let mut client = get_client().unwrap();

    // The `all` method returns queried rows collected into a `Vec`
    let authors = authors().bind(&mut client).all().unwrap();
    dbg!(authors);

    // Queries also accept transactions. Let's see how that works.
    {
        // Once you've created a transaction, you can pass it to your queries
        // just like you would with a regular query. Nothing special to do.
        let mut transaction = client.transaction().unwrap();

        // Insertions work just like any other query.
        // Note that queries with a void return type (such as regular insertions)
        // are executed as soon as you `bind` their parameters.
        insert_book()
            .bind(&mut transaction, &"The Great Gatsby")
            .unwrap();

        // Bind parameters are "smart". A query that expects a `&str` will also accept other
        // "string-like" types like `String`. See https://cornucopia-rs.netlify.app/book/using_queries/ergonomic_parameters.html
        // for more details.
        insert_book()
            .bind(&mut transaction, &String::from("Moby Dick"))
            .unwrap();

        // You can use a `map` to transform query results ergonomically.
        let uppercase_books = books()
            .bind(&mut transaction)
            .map(|book_title| book_title.to_uppercase())
            .all()
            .unwrap();
        dbg!(uppercase_books);

        // Don't forget to `commit` when you're done with the transaction!
        // Otherwise, it will be rolled back without further effect.
        transaction.commit().unwrap();
    }

    // Using `opt` returns an optional row (zero or one).
    // Any other number of rows will return an error.
    let author_name = author_name_by_id().bind(&mut client, &0).opt().unwrap();
    dbg!(author_name);

    // Using named structs as parameters and rows can be more convenient
    // and less error-prone, for example when a query has a lot of parameters.
    // This query doesn't benefit much, but is still shown for demonstration purposes.
    // ! Note: To use this feature you need to:
    // ! 1. Have a struct generated for your parameters
    // !    (see https://cornucopia-rs.netlify.app/book/writing_queries/type_annotations.html for
    // !    general information and the `queries/module_2.sql` file to see how this particular
    // !    parameter type was created).
    // ! 2. Import the `Params` trait.
    let name_starting_with_jo = author_name_starting_with()
        .params(
            &mut client,
            &AuthorNameStartingWithParams { start_str: "Jo" },
        )
        .all()
        .unwrap();
    dbg!(name_starting_with_jo);

    // Custom PostgreSQL types from your queries also work!
    // This includes domains, composites and enums.
    // They will be automatically generated by Cornucopia.
    // You can use them as bind parameters (as shown here)
    // or receive them in returned rows.
    let patrick_voice_actor = select_voice_actor_with_character()
        .bind(&mut client, &SpongeBobCharacter::Patrick)
        .one()
        .unwrap();
    dbg!(patrick_voice_actor);

    // Cornucopia also supports PostgreSQL arrays, which you
    // can use as bind parameters or in returned rows.
    let translations = select_translations()
        .bind(&mut client)
        .map(|row| format!("{}: {:?}", row.title, row.translations))
        .all()
        .unwrap();
    dbg!(translations);
}

/// Connection client configuration.
///
/// This is just a simple example config, please look at
/// `postgres` for details.
use postgres::{Config, NoTls};
fn get_client() -> Result<postgres::Client, postgres::Error> {
    Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5435)
        .dbname("postgres")
        .connect(NoTls)
}
