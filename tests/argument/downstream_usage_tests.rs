// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Consumer-shaped tests derived from Qubit downstream validation code.

use std::time::Duration;

use qubit_argument::{
    ArgumentError,
    ArgumentErrorKind,
    ArgumentResult,
    ArgumentResultExt,
    ArgumentValue,
    ComparisonConstraint,
    DurationArgument,
    FloatArgument,
    NumericArgument,
    OptionArgument,
    StringArgument,
};

#[derive(Debug, PartialEq, Eq)]
enum DomainError {
    InvalidArgument(ArgumentError),
}

impl From<ArgumentError> for DomainError {
    /// Preserves a structured argument failure in the simulated domain error.
    fn from(error: ArgumentError) -> Self {
        Self::InvalidArgument(error)
    }
}

/// Returns the structured argument error wrapped by the domain error.
fn into_argument_error(error: DomainError) -> ArgumentError {
    let DomainError::InvalidArgument(error) = error;
    error
}

/// Simulates `rs-event-bus` topic construction with an owned name.
fn build_topic(name: String) -> Result<String, DomainError> {
    let name = name.require_non_blank("name")?;
    Ok(name)
}

/// Verifies that the existing string API already fits event-bus construction.
#[test]
fn test_event_bus_topic_uses_domain_conversion_without_map_err() {
    let name = String::from("orders.created");
    assert_eq!(
        build_topic(name).expect("non-blank topic name is valid"),
        "orders.created",
    );

    let error = build_topic(String::from("\u{2003}"))
        .expect_err("blank topic name must fail");
    let error = into_argument_error(error);
    assert_eq!(error.path().as_str(), "name");
    assert_eq!(error.kind(), &ArgumentErrorKind::Blank);
}

/// A minimal model of the shared executor and thread-pool builder checks.
struct ThreadPoolOptions {
    maximum_pool_size: usize,
    core_pool_size: usize,
    queue_capacity: Option<usize>,
    stack_size: Option<usize>,
    keep_alive: Duration,
}

impl ThreadPoolOptions {
    /// Validates count, cross-field, optional, and duration preconditions.
    fn validate(&self) -> Result<(), DomainError> {
        self.maximum_pool_size
            .require_positive("maximum_pool_size")?;
        self.core_pool_size
            .require_at_most("core_pool_size", self.maximum_pool_size)?;
        self.queue_capacity.validate_if_some(|capacity| {
            (*capacity).require_positive("queue_capacity")?;
            Ok(())
        })?;
        self.stack_size.validate_if_some(|stack_size| {
            (*stack_size).require_positive("stack_size")?;
            Ok(())
        })?;
        self.keep_alive.require_positive("keep_alive")?;
        Ok(())
    }
}

/// Verifies that executor-style optional counts compose with duration checks.
#[test]
fn test_thread_pool_builder_composes_argument_traits() {
    let options = ThreadPoolOptions {
        maximum_pool_size: 8,
        core_pool_size: 4,
        queue_capacity: Some(0),
        stack_size: Some(2 * 1024 * 1024),
        keep_alive: Duration::from_secs(30),
    };
    let error = options
        .validate()
        .expect_err("zero optional queue capacity must fail");
    let error = into_argument_error(error);
    assert_eq!(error.path().as_str(), "queue_capacity");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(0_usize),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(
                0_usize
            ),),
        },
    );
}

/// Verifies that a zero keep-alive remains a structured duration value.
#[test]
fn test_thread_pool_builder_reports_duration_value() {
    let options = ThreadPoolOptions {
        maximum_pool_size: 8,
        core_pool_size: 4,
        queue_capacity: Some(32),
        stack_size: None,
        keep_alive: Duration::ZERO,
    };
    let error = options.validate().expect_err("zero keep-alive must fail");
    let error = into_argument_error(error);
    assert_eq!(error.path().as_str(), "keep_alive");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(Duration::ZERO),
            constraint: ComparisonConstraint::GreaterThan(ArgumentValue::from(
                Duration::ZERO
            ),),
        },
    );
}

/// A nested timeout configuration modeled after `rs-http`.
struct TimeoutOptions {
    connect: Duration,
}

impl TimeoutOptions {
    /// Validates the local timeout path without knowing its parent object.
    fn validate(&self) -> ArgumentResult<()> {
        self.connect.require_positive("connect")?;
        Ok(())
    }
}

/// A top-level HTTP configuration that owns nested timeout options.
struct HttpOptions {
    timeouts: TimeoutOptions,
}

impl HttpOptions {
    /// Adds parent context only when nested timeout validation fails.
    fn validate(&self) -> Result<(), DomainError> {
        self.timeouts.validate().with_path_prefix("timeouts")?;
        Ok(())
    }
}

/// Verifies that nested HTTP errors gain a parent path without `map_err`.
#[test]
fn test_http_options_prefix_nested_error_path() {
    let options = HttpOptions {
        timeouts: TimeoutOptions {
            connect: Duration::ZERO,
        },
    };
    let error = options
        .validate()
        .expect_err("zero connection timeout must fail");
    let error = into_argument_error(error);
    assert_eq!(error.path().as_str(), "timeouts.connect");
    assert!(matches!(error.kind(), ArgumentErrorKind::Comparison { .. },));
}

/// A retry delay configuration modeled after `rs-retry`.
struct RetryDelayOptions {
    initial: Duration,
    max: Duration,
    multiplier: f64,
}

impl RetryDelayOptions {
    /// Validates duration relationships and a finite multiplier locally.
    fn validate(&self) -> ArgumentResult<()> {
        self.initial.require_positive("initial")?;
        self.max.require_at_least("max", self.initial)?;
        self.multiplier
            .require_finite("multiplier")?
            .require_greater_than("multiplier", 1.0)?;
        Ok(())
    }
}

/// A top-level retry configuration that prefixes delay errors.
struct RetryOptions {
    delay: RetryDelayOptions,
}

impl RetryOptions {
    /// Validates nested retry delay options and converts with `?`.
    fn validate(&self) -> Result<(), DomainError> {
        self.delay.validate().with_path_prefix("delay")?;
        Ok(())
    }
}

/// Verifies that retry validation rejects infinity before lower comparison.
#[test]
fn test_retry_options_require_finite_multiplier() {
    let options = RetryOptions {
        delay: RetryDelayOptions {
            initial: Duration::from_millis(100),
            max: Duration::from_secs(10),
            multiplier: f64::INFINITY,
        },
    };
    let error = options
        .validate()
        .expect_err("infinite retry multiplier must fail");
    let error = into_argument_error(error);
    assert_eq!(error.path().as_str(), "delay.multiplier");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::NotFinite {
            actual: ArgumentValue::from(f64::INFINITY),
        },
    );
}

/// Verifies that retry cross-field duration failures retain both values.
#[test]
fn test_retry_options_compare_duration_fields() {
    let initial = Duration::from_secs(2);
    let max = Duration::from_secs(1);
    let options = RetryOptions {
        delay: RetryDelayOptions {
            initial,
            max,
            multiplier: 2.0,
        },
    };
    let error = options
        .validate()
        .expect_err("maximum delay below initial delay must fail");
    let error = into_argument_error(error);
    assert_eq!(error.path().as_str(), "delay.max");
    assert_eq!(
        error.kind(),
        &ArgumentErrorKind::Comparison {
            actual: ArgumentValue::from(max),
            constraint: ComparisonConstraint::AtLeast(ArgumentValue::from(
                initial,
            )),
        },
    );
}
