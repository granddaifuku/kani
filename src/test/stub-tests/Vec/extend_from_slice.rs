// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// rmc-flags: --use-abs --abs-type rmc
include!{"../../rmc-prelude.rs"}

fn main() {
    fn extend_from_slice_test() {
        let mut vec = rmc_vec![1];
        vec.extend_from_slice(&[2, 3, 4]);
        assert!(vec == [1, 2, 3, 4]);
    }

    extend_from_slice_test();
}