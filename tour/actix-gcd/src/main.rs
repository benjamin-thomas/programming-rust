use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use elm_rs::{Elm, ElmDecode, ElmEncode};
use serde::Deserialize;

#[derive(Deserialize, Elm, ElmEncode, ElmDecode)]
struct GcdParameters {
    n: u64,
    m: u64,
    other: Option<u8>,
}

/*
  cargo watch --ignore elm/ -x run
*/

#[actix_web::main]
async fn main() {
    println!("==> Exporting Elm types...");
    let mut elm_target = vec![];
    let generated_name = "Domain";
    let generated_module_name = "Generated.Domain";
    let generated_path = format!("elm/src/Generated/{}.elm", generated_name);
    elm_rs::export!(generated_module_name, &mut elm_target, {
        encoders: [GcdParameters],
        decoders: [GcdParameters],
        // queries: [Query],
        // query_fields: [Size],
    })
    .expect("Could not generate Elm types!");
    let elm_output =
        String::from_utf8(elm_target).expect("Could not build String from exported Elm types");

    std::fs::write(generated_path, elm_output).expect("Could not write file: Generated.elm");

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    let socket = "127.0.0.1:3000".to_string();
    println!("Running on http://{}...", socket);

    server
        .bind(socket.clone())
        .expect(&format!("Failed to bind to {}", socket))
        .run()
        .await
        .expect("Server failed to start!");
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>

        <form action="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="m" />
            <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} \
        is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
