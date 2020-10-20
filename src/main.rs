#[warn(unused_imports)]
#[warn(dead_code)]
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;

mod db;
mod models;
mod schema;

fn main() {
    let email = "banana@man.com";
    let password = "banana_password";

    let user = models::user::NewUser::create(&email, &password).unwrap_or_else(|err| {
        panic!("Problem creating user: {}", err);
    });

    println!("created user: {:#?}", &user);

    let got_user = models::user::User::show(&user.id).unwrap_or_else(|err| {
        panic!("Problem getting the user again: {}", err);
    });

    println!("got user: {:#?}", &got_user);

    let authenticated = models::user::AuthenticableUser::authenticate(&email, &password)
        .unwrap_or_else(|err| {
            panic!("Problem authentication the user: {}", err);
        });

    println!("got authenticated user: {:#?}", &authenticated);

    let todo_content = "Something random";
    let todo = got_user.add_todo(&todo_content).unwrap_or_else(|err| {
        panic!("Problem creating a todo: {}", err);
    });

    println!("created a new todo: {:#?}", todo);

    let todo_content = "Something random2";
    let todo = got_user.add_todo(&todo_content).unwrap_or_else(|err| {
        panic!("Problem creating a todo: {}", err);
    });

    let user_with_todos = models::user::UserWithTodo::show(&user.id).unwrap_or_else(|err| {
        panic!("Problem getting the user with todos: {}", err);
    });

    println!("user with todos: {:#?}", user_with_todos);
}
