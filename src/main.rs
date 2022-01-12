use std::path::PathBuf;
use structopt::StructOpt;
use json::object;
use json::array;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use log4rs;
use log;

mod models;
mod handlers;

type ItemsDb = Arc<Mutex<HashMap<usize, models::ShoppingListItem>>>;
type Result<T> = std::result::Result<T, Rejection>;

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct Cli {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set port
    #[structopt(short, long, default_value = "5000")]
    port: u16,


    /// bind address
    #[structopt(short, long, default_value = "127.0.0.1")]
    bind: String,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}


fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let opt = Cli::from_args();
    println!("{:#?}", opt);

    let data = object!{
        "code" => 200,
        "success" => true,
        "payload" => object!{
            "features" => array![
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };

    println!("{:#?}", data.dump());
    println!("{}", data["code"]);
    println!("Start");
    start_http(opt.port, opt.bind);
    println!("Stop");
}



#[tokio::main]
async fn start_http(port: u16, bind :String) {
    log::warn!("Starting !!");
    let items_db: ItemsDb = Arc::new(Mutex::new(HashMap::new()));
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    

    let socket = create_socket(port, bind);

    let shopping_list_items_route = warp::path("shopping_list_items")
        .and(warp::get())
        .and(with_items_db(items_db.clone()))
        .and_then(handlers::get_shopping_list_items);

    let shopping_list_item_route = warp::path("shopping_list_item")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_items_db(items_db.clone()))
        .and_then(handlers::create_shopping_list_item)
    .or(warp::path!("shopping_list_item" / usize)
        .and(warp::get())
        .and(with_items_db(items_db.clone()))
        .and_then(handlers::get_shopping_list_item_by_id));


    let routes = root
        .or(shopping_list_item_route)
        .or(shopping_list_items_route)
        .with(warp::cors().allow_any_origin());

        
    warp::serve(routes).run(socket).await;
}

fn with_items_db(
    items_db: ItemsDb,
) -> impl Filter<Extract = (ItemsDb,), Error = Infallible> + Clone {
    warp::any().map(move || items_db.clone())
}

fn create_socket(port: u16, bind :String) -> SocketAddr {
    let bind_addr = bind.parse::<Ipv4Addr>().unwrap();
    let socket = SocketAddr::new(IpAddr::V4(bind_addr), port);
    return socket;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_create() {
        let socket = create_socket(1234, "1.2.3.4".to_owned());

        assert_eq!(socket.port(), 1234);
        assert_eq!(socket.is_ipv4(), true);
        

    }
}