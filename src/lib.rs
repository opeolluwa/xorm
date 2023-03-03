// Copyright 2022 The Racoon Authors. All Rights Reserved.
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

/// the model trait is ued to provide
pub trait IntoModel {
    // const TABLE_NAME: String;
    /// create a new model
    fn create() {
        println!("create a new record")
    }

    /// find all record
    fn find(condition: String) {
        println!("the find model where {}", condition)
    }

    /// find record, if not fount create
    fn find_or_create() {
        println!("the find or create associated function")
    }

    /// find record by primary key
    // TODO: find a way to deduce the primary key
    fn find_by_pk() {
        println!("the find by pk associated function")
    }

    /// delete a model record
    fn destroy() {
        println!("Delete a model record")
    }
}

pub enum ImplModel {
    Create,
    Find,
    FindOrCreate,
    Destroy,
    FindByPk,
}
