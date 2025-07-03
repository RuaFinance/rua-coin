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

use crate::handler;
use actix_web::{guard, web, Scope};

// ==================
// Main route
// ==================
pub fn base_route() -> Scope {
    web::scope("")
        .service(api_route())
        .route("/", web::get().to(handler::hello_page))
}

// ==================
// Sub routes
// ==================
pub fn api_route() -> Scope {
    web::scope("/api/oms")
        .service(user_route())
        .service(order_route())
        .guard(guard::Get())
}

pub fn user_route() -> Scope {
    web::scope("/users")
        .route("", web::get().to(handler::hello_page))
        .route("/{id}", web::get().to(handler::hello_page))
}

pub fn order_route() -> Scope {
    web::scope("/orders")
        .route("/{user_id}", web::post().to(handler::create_order))
        .route("/{order_id}", web::get().to(handler::get_order))
        .route("/{user_id}/list", web::get().to(handler::list_order))
}
