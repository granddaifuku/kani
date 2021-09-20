// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// rmc-flags: --use-abs --abs-type rmc
include!{"../../rmc-prelude.rs"}

fn main() {
    fn resize_with_test() {
        let mut vec = rmc_vec![1, 2, 3];
        vec.resize_with(5, Default::default);
        assert_eq!(vec, [1, 2, 3, 0, 0]);

        let mut vec = rmc_vec![];
        let mut p = 1;
        vec.resize_with(4, || {
            p *= 2;
            p
        });
        assert_eq!(vec, [2, 4, 8, 16]);
    }

    resize_with_test();
}