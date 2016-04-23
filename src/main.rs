extern crate iron;
#[macro_use]
extern crate mime;
extern crate router;
extern crate urlencoded;

use std::str::FromStr;
use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;


fn main() {
    let mut router = Router::new();
    router.get("/", get_form);
    router.post("/gcd", post_gcd);
    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#);
    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let hashmap;
    match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("表单数据解析失败: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => {
            hashmap = map;
        }
    }

    let unparsed_numbers;
    match hashmap.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("表单数据缺少参数'n'\n"));
            return Ok(response);
        }
        Some(nums) => {
            unparsed_numbers = nums;
        }
    }

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(format!("'n'不是一个数字: {:?}\n", unparsed));
                return Ok(response);
            }
            Ok(n) => {
                numbers.push(n);
            }
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("{:?}的最大公约数是:<b>{}</b>\n", numbers, d));

    Ok(response)
}

fn gcd(mut m : u64, mut n : u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m; m = n; n = t;
        }
        m = m % n;
    }
    n
}
