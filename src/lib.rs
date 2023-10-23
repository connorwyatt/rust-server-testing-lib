use std::net::{SocketAddr, TcpListener};

use axum::Router;

pub fn create_test_server(router: Router) -> SocketAddr {
    let socket_address = "0.0.0.0:0"
        .parse::<SocketAddr>()
        .expect("socket address should always be parseable");
    let listener = TcpListener::bind(socket_address).expect("binding should always be possible");
    let socket_address = listener
        .local_addr()
        .expect("local_addr should always be retrievable from listener");

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    socket_address
}
