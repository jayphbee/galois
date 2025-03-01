// Copyright 2021 UINB Technologies Pte. Ltd.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::config;
use lazy_static::lazy_static;
use mysql::*;
use redis::Client;

lazy_static! {
    pub static ref DB: Pool = Pool::new(&config::C.mysql.url).unwrap();
    pub static ref REDIS: Client = Client::open((&config::C.redis.url).to_string()).unwrap();
}
