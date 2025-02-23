use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

type DBClient = Surreal<Client>;
// const TABLE_NAME: &'static str = "tasks";

pub async fn establish_db_connection() -> DBClient {
    let _ = dotenv::dotenv().ok();

    let url = std::env::var("DB_URL").expect("Unable to get DB_URL variable!");
    let username = std::env::var("DB_USERNAME").expect("Unable to get DB_USERNAME variable!");
    let password = std::env::var("DB_PASSWORD").expect("Unable to get DB_PASSWORD variable!");
    let namespace = std::env::var("DB_NS").expect("Unable to get DB_NS variable!");
    let database = std::env::var("DB_DATABASE").expect("Unable to get DB_DATABASE variable!");

    let db_client = Surreal::new::<Ws>(url)
        .await
        .expect("Unable to connect to Database!");
    db_client
        .signin(Root {
            username: username.as_str(),
            password: password.as_str(),
        })
        .await
        .expect("Unable to signin! Please checkout username and password!");

    db_client
        .use_ns(namespace)
        .use_db(database)
        .await
        .expect("An error occured with using namespace or database!");

    db_client
} // task managament

pub async fn get_redis_client() -> redis::Client {
    let _ = dotenv::dotenv().ok();

    let url = std::env::var("REDIS_URL").expect("Unable to get REDIS_URL variable!");
    let redis_client = redis::Client::open(url).expect("Unable to connect to Redis!");
    redis_client
}
