// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-24 (Y/M/D)
//!
//! SHORT-NAME: `block`, LONG-NAME: `BlockExecution`, RUNTIME: `Development`
//! WARMUPS: `10`, REPEAT: `100`
//! WEIGHT-PATH: `runtime/westend/constants/src/weights/`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1`, WEIGHT-ADD: `0`

// Executed Command:
//   ./target/production/polkadot
//   benchmark-overhead
//   --chain
//   westend-dev
//   --execution=wasm
//   --wasm-execution=compiled
//   --weight-path
//   runtime/westend/constants/src/weights/
//   --warmup
//   10
//   --repeat
//   100

use frame_support::{
	parameter_types,
	weights::{constants::WEIGHT_PER_NANOS, Weight},
};

parameter_types! {
	/// Time to execute an empty block.
	/// Calculated by multiplying the *Average* with `1` and adding `0`.
	///
	/// Stats [NS]:
	///   Min, Max: 3_553_100, 3_737_847
	///   Average:  3_592_873
	///   Median:   3_573_460
	///   Std-Dev:  34948.46
	///
	/// Percentiles [NS]:
	///   99th: 3_699_717
	///   95th: 3_660_927
	///   75th: 3_608_068
	pub const BlockExecutionWeight: Weight = 3_592_873 * WEIGHT_PER_NANOS;
}

#[cfg(test)]
mod test_weights {
	use frame_support::weights::constants;

	/// Checks that the weight exists and is sane.
	// NOTE: If this test fails but you are sure that the generated values are fine,
	// you can delete it.
	#[test]
	fn sane() {
		let w = super::BlockExecutionWeight::get();

		// At least 100 µs.
		assert!(w >= 100 * constants::WEIGHT_PER_MICROS, "Weight should be at least 100 µs.");
		// At most 50 ms.
		assert!(w <= 50 * constants::WEIGHT_PER_MILLIS, "Weight should be at most 50 ms.");
	}
}
