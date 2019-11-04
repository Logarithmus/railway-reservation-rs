mod filters;

fn main() {
    warp::serve(filters::all_filters())
        .tls("tls/cert.pem", "tls/key.rsa")
        .run(([127, 0, 0, 1], 8080));
}
