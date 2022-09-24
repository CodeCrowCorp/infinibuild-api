mod r2utils {
    /*use worker::{Env, Method, Request, Response, Result};

    pub async fn fetch(req: Request, env: Env) -> Result<Response> {
        let bucket = env.get("bucket_name")?;
        let url = req.url();
        let key = url.pathname.slice(1);
        return if req.headers().get("X-Custom-Auth-Key") == env.AUTH_KEY_SECRET {
            match req.method().as_ref() {
                "PUT" => {
                    bucket.put(key, req.body()).await?;
                    Response::ok({})
                }
                "GET" => {
                    let object = bucket.get(key).await?;
                    if object.is_none() {
                        Response::not_found();
                    }
                    let headers = object.headers();
                    object.writeHttpMetadata(&mut headers);
                    headers.set("etag", object.http_etag());
                    Ok(Response::new()
                        .with_headers(headers)
                        .with_body(object.body()))
                }
                "DELETE" => {
                    bucket.delete(key).await?;
                    Response::ok({})
                }
                _ => {
                    Response::error("Bad Request", 400)
                }
            }
        } else {
            Response::error("Unauthorized", 401)
        }
    }*/
}