// Copyright 2025 chenjjiaa
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused)]

use std::fmt::format;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};

/// ``` rust
/// #[get("/hello/{name}")]
/// pub async fn greet(name: web::Path<String>) -> impl Responder {
///     format!("Hello {name}!")
/// }
/// ```

pub async fn hello_page() -> impl Responder {
    // HttpResponse::Ok().json("hello".to_string())
    "Hello, this is RUA-COIN!".to_string()
}

// ==================
// Order handler
// ==================
/// POST /orders/{user_id}
pub async fn create_order(req: HttpRequest, user_id: web::Path<String>) -> Result<impl Responder, Error> {
    // 获取用户信息
    // 鉴权
    // 判断余额
    // 创建订单
    // 发送订单创建事件
    // 返回响应
    Ok(HttpResponse::Ok().body(format!("User {} create order success", user_id.into_inner())))
}

/// GET /orders/{order_id}
pub async fn get_order(order_id: web::Path<String>) -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body(format!("User {} create order success", order_id.into_inner())))
}

/// GET /orders/{user_id}/list
pub async fn list_order(user_id: web::Path<String>)  -> impl Responder {
    HttpResponse::Ok().body(format!("List orders for user '{}'", user_id.into_inner()))
}

/// PUT /orders/{order_id}
pub async fn update_order(order_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Update order {}", order_id.into_inner()))
}

/// DELETE /orders/{order_id}
pub async fn delete_order(order_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Delete order {}", order_id.into_inner()))
}

/// POST /orders/{order_id}/cancel
pub async fn cancel_order(order_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Cancel order {}", order_id.into_inner()))
}