// I2TH OPC UA Streams Types
// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2020 Hauke Jung

#![deny(
    bad_style,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features
)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
pub mod config;
pub mod sensor_data;
pub mod sensor_type;