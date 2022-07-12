use actix_web::{web, App, HttpServer};
use mongodb::Client;

#[path = "app/constants/index.rs"]
mod constants;
#[path = "app/models/user.rs"]
pub(crate) mod model;
#[path = "routes/index.rs"]
mod routes;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::scope("/routes"))
            .service(routes::index)
            .service(routes::create_user)
            .service(routes::update_user)
            .service(routes::get_all_users)
            .service(routes::delete_user)
            .service(routes::create_jwt_token)
            .service(routes::get_user)
            .service(routes::search_users)
            .service(routes::upload_file)
            // .service(routes::download_file),
            .service(routes::download_file),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str(constants::DB_URL)
        .await
        .expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .configure(init)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}

// Header authorization---------------------------------------------------------

// .wrap_fn(|req, srv| {
//     let header = req
//         .headers()
//         .get("authorization")
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_owned();
//     let fut = srv.call(req);
//     async move {
//         let res = fut.await?;
//         Ok(res)
//     }
// })

// pub fn validate() {
//     let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpZCI6IiIsInBhc3MiOiIifQ.P8BwWVmDHft3Eh4L6yeHL5OKmGNgITt_GGFtA3e1dIcYtQOrVmXP12Vxmd2EvijyqYvbiYQ3noyuc8MuvLO9jA";

//     let token_data = match decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(JWT_SECRET),
//         &Validation::new(Algorithm::HS512),
//     ) {
//         Ok(t) => t,
//         Err(e) => {
//             eprintln!("error ------------->{}", e);
//             panic!("Could not decode")
//         }
//     };

//     println!("token_data ----- {:?}", token_data)
// }
