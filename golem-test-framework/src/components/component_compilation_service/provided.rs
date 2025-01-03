// Copyright 2024-2025 Golem Cloud
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

use crate::components::component_compilation_service::ComponentCompilationService;
use async_trait::async_trait;

use tracing::info;

pub struct ProvidedComponentCompilationService {
    host: String,
    http_port: u16,
    grpc_port: u16,
}

impl ProvidedComponentCompilationService {
    pub fn new(host: String, http_port: u16, grpc_port: u16) -> Self {
        info!("Using already running golem-compilation-component-service on {host}, http port: {http_port}, grpc port: {grpc_port}");
        Self {
            host,
            http_port,
            grpc_port,
        }
    }
}

#[async_trait]
impl ComponentCompilationService for ProvidedComponentCompilationService {
    fn private_host(&self) -> String {
        self.host.clone()
    }

    fn private_http_port(&self) -> u16 {
        self.http_port
    }

    fn private_grpc_port(&self) -> u16 {
        self.grpc_port
    }

    async fn kill(&self) {}
}
