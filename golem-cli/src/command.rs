// Copyright 2024 Golem Cloud
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

use crate::model::ComponentUriArg;
use crate::oss::model::OssContext;
use golem_common::uri::oss::uri::ComponentUri;

pub mod api_definition;
pub mod api_deployment;
pub mod api_security;
pub mod component;
pub mod plugin;
pub mod profile;
pub mod worker;

pub trait ComponentRefSplit<ProjectRef> {
    fn split(self) -> (ComponentUri, Option<ProjectRef>);
}

impl ComponentRefSplit<OssContext> for ComponentUriArg {
    fn split(self) -> (ComponentUri, Option<OssContext>) {
        (self.uri, None)
    }
}
