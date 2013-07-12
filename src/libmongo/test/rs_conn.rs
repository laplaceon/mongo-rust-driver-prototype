/* Copyright 2013 10gen Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use mongo::client::*;
use mongo::coll::*;
use mongo::util::*;

use bson::formattable::*;
use bson::encode::*;
#[test]
fn test_rs_conn() {
    // replica set connection
    let client = @Client::new();
    match client.connect_to_rs(~[(~"127.0.0.1", 27018)]) {
        Ok(_) => (),
        Err(e) => fail!("%s", MongoErr::to_str(e)),
    }

    let coll = @Collection::new(~"rust", ~"good_insert_one", client);

    // clear out collection to start from scratch
    //coll.remove(None, None, None, None);

    // create and insert document
    let ins = ~"{ \"a\":0, \"msg\":\"first insert!\" }";
    let ins_doc = match (copy ins).to_bson_t() {
            Embedded(bson) => *bson,
            _ => fail!("what happened"),
        };
    coll.insert::<~str>(ins, None);

/*
    // try to extract it and compare
    match coll.find_one(None, None, None) {
        Ok(ret_doc) => assert!(*ret_doc == ins_doc),
        Err(e) => fail!("%s", MongoErr::to_str(e)),
    }
*/

    match client.disconnect() {
        Ok(_) => (),
        Err(e) => fail!("%s", MongoErr::to_str(e)),
    }
}
