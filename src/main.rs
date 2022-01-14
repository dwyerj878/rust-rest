use structopt::StructOpt;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use log4rs;
use log;

mod models;
mod handlers;
mod cli;

type ItemsDb = Arc<Mutex<HashMap<usize, models::ShoppingListItem>>>;
type Result<T> = std::result::Result<T, Rejection>;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let opt = cli::Cli::from_args();

    if opt.debug {
        log::debug!("Options");
        log::debug!("{:#?}", opt);
    }

    log::info!("Start");
    start_http(opt.port, opt.bind);
    log::info!("Stop");
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