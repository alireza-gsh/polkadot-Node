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
//! SHORT-NAME: `extrinsic`, LONG-NAME: `ExtrinsicBase`, RUNTIME: `Development`
//! WARMUPS: `10`, REPEAT: `100`
//! WEIGHT-PATH: `runtime/kusama/constants/src/weights/`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1`, WEIGHT-ADD: `0`

// Executed Command:
//   ./target/production/polkadot
//   benchmark-overhead
//   --chain
//   kusama-dev
//   --execution=wasm
//   --wasm-execution=compiled
//   --weight-path
//   runtime/kusama/constants/src/weights/
//   --warmup
//   10
//   --repeat
//   100

use frame_support::{
	parameter_types,
	weights::{constants::WEIGHT_PER_NANOS, Weight},
};

parameter_types! {
	/// Time to execute a NO-OP extrinsic, for example `System::remark`.
	/// Calculated by multiplying the *Average* with `1` and adding `0`.
	///
	/// Stats [NS]:
	///   Min, Max: 80_176, 81_558
	///   Average:  80_350
	///   Median:   80_335
	///   Std-Dev:  136.56
	///
	/// Percentiles [NS]:
	///   99th: 80_492
	///   95th: 80_459
	///   75th: 80_377
	pub const ExtrinsicBaseWeight: Weight = 80_350 * WEIGHT_PER_NANOS;
}

#[cfg(test)]
mod test_weights {
	use frame_support::weights::constants;

	/// Checks that the weight exists and is sane.
	// NOTE: If this test fails but you are sure that the generated values are fine,
	// you can delete it.
	#[test]
	fn sane() {
		let w = super::ExtrinsicBaseWeight::get();

		// At least 10 µs.
		assert!(w >= 10 * constants::WEIGHT_PER_MICROS, "Weight should be at least 10 µs.");
		// At most 1 ms.
		assert!(w <= constants::WEIGHT_PER_MILLIS, "Weight should be at most 1 ms.");
	}
}
