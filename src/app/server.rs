use crate::config::Config;
use std::sync::Arc;
use crate::error::Error;
use std::net::SocketAddr;
use crate::app::routes;
use crate::monzo;
use crate::monzo::Collector;
use warp::Filter;

pub struct Server {
    pub config: Arc<Config>,
    pub(super) monzo_client: Arc<monzo::Client>,
    pub(super) collector: Arc<Collector>,
}

//type RouteFilter = impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone;

impl Server {
    pub fn new(config: Arc<Config>, monzo_client: Arc<monzo::Client>, collector: Arc<Collector>) -> Server {
        Server {
            config,
            monzo_client,
            collector,
        }
    }

    pub async fn start(self: Arc<Self>) -> Result<(), Error> {
        let address: SocketAddr = self.config.server_addr.clone().parse().unwrap();

        let filter =
            self.clone().filter_authorize()
                .or(self.clone().filter_callback())
                .or(self.clone().filter_stats());

        warp::serve(filter)
            .run(address)
            .await;

        Ok(())
    }

    fn filter_authorize(self: Arc<Self>) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path("authorize")
            .and(warp::any().map(move || self.clone()))
            .and(warp::query::<routes::AuthorizeQuery>())
            .and_then(routes::authorize)
    }

    fn filter_callback(self: Arc<Self>) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path("callback")
            .and(warp::any().map(move || self.clone()))
            .and(warp::query::<routes::CallbackQuery>())
            .and_then(routes::callback)
    }

    fn filter_stats(self: Arc<Self>) -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path("stats")
            .and(warp::any().map(move || self.clone()))
            .and_then(routes::stats)
    }
}
