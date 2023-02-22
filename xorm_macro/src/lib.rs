// Copyright 2023 The Adeoye Adefemi. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro::TokenStream;
use syn;

mod destroy;
mod find;
mod find_by_pk;

#[proc_macro_derive(IntoModel)]
pub fn find_by_pk_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
  
    find_by_pk::impl_macro(&ast)
}

