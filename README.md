# Rust Experiments
Small experiments in writing Rust programs to perform specific tasks.  Clone the repository and use `cargo run` with the `-p <crate name>` flag to run one of programs.

## General Rust
+ [Error Handling](https://github.com/keithsharp/rust-experiments/tree/main/error-tests) - Basic experiments using Rust Errors.
+ [More Error Handling](https://github.com/keithsharp/rust-experiments/tree/main/more-errors) - More experiments with Rust Errors and the [`thiserror`](https://docs.rs/thiserror/latest/thiserror/) crate.
+ [Options](https://github.com/keithsharp/rust-experiments/tree/main/option) - Experiments using Rust Option enums and their methods.

## Interesting Crates
+ [Parsing TOML](https://github.com/keithsharp/rust-experiments/tree/main/read-toml) - Read values from a TOML formatted file using [`serde`](https://serde.rs).
+ [Reedline](https://github.com/keithsharp/rust-experiments/tree/main/reedline) - Reading from the terminal with [`reedline`](https://docs.rs/reedline/latest/reedline/). 
+ [REPL](https://github.com/keithsharp/rust-experiments/tree/main/repl) - Create a Read, Eval, Print, Loop (REPL) interface using [`reedline-repl-rs`](https://docs.rs/reedline-repl-rs/latest/reedline_repl_rs/).

## AWS Rust SDK
[GitHub](https://github.com/awslabs/aws-sdk-rust) and [documentation](https://awslabs.github.io/aws-sdk-rust/).
+ [Create Bucket](https://github.com/keithsharp/rust-experiments/tree/main/aws-create-bucket) - Create and tag an S3 Bucket, then delete it.
+ [Create VPC](https://github.com/keithsharp/rust-experiments/tree/main/aws-create-vpc) - Create a VPC with Subnets spread across different Availability Zones.
+ [List Buckets](https://github.com/keithsharp/rust-experiments/tree/main/aws-list-buckets) - List all the S3 Buckets in an account.
+ [AWS Profile](https://github.com/keithsharp/rust-experiments/tree/main/aws-profile) - Choose which AWS Credentials profile to use.
+ [AWS VPC](https://github.com/keithsharp/rust-experiments/tree/main/aws-vpc) - Tagging and describing VPCs.
+ [Create Instance](https://github.com/keithsharp/rust-experiments/tree/main/create-instance) - Create an EC2 Instance and all the support VPC and IAM bits.
+ [Create Instance Profile](https://github.com/keithsharp/rust-experiments/tree/main/create-instance-profile) - Create an Instance Profile with a Role and Trust Policy.
+ [Default VPC Security Groups](https://github.com/keithsharp/rust-experiments/tree/main/default-vpc-sg) - Security Group tests using the default VPC.
+ [Inspect VPC](https://github.com/keithsharp/rust-experiments/tree/main/inspect-vpc) - Describe the details of a VPC.
+ [Internet Gateway](https://github.com/keithsharp/rust-experiments/tree/main/internet-gateway) - Create a VPC with an Internet connection using an Internet Gateway.
+ [S3 File Upload](https://github.com/keithsharp/rust-experiments/tree/main/s3-file-upload) - Create an S3 bucket and upload a file.
+ [S3 Gateway Endpoint](https://github.com/keithsharp/rust-experiments/tree/main/s3-gateway-endpoint) - Create a VPC containing an S3 Gateway Endpoint.
+ [Security Groups](https://github.com/keithsharp/rust-experiments/tree/main/security-group) - Create security groups and create trust between them.
+ [SQS](https://github.com/keithsharp/rust-experiments/tree/main/sqs) - Create, delete, describe, and send messages to SQS queues.
+ [VPC Filter](https://github.com/keithsharp/rust-experiments/tree/main/vpc-filter) - Describe a VPC based on it's tags.

## Axum
[GitHub](https://github.com/tokio-rs/axum) and [documentation](https://docs.rs/axum/latest/axum/).
+ [Axum Basics](https://github.com/keithsharp/rust-experiments/tree/main/axum-basic) - Basic routing and HTTP methods.
+ [Axum WS Ping](https://github.com/keithsharp/rust-experiments/tree/main/axum-ws-ping) - A WebSocket ping server using Axum.
+ [Axum JavaScript Messages](https://github.com/keithsharp/rust-experiments/tree/main/axum-js-msg) - Axum WebSocket server with JavaScript messages.
+ [Axum JWT](https://github.com/keithsharp/rust-experiments/tree/main/axum-jwt) - Authentication in Axum using JWTs.
+ [Axum WS Auth](https://github.com/keithsharp/rust-experiments/tree/main/axum-ws-jwt) - WebSocket authentication in Axum using JWTs.
+ [Axum Errors](https://github.com/keithsharp/rust-experiments/tree/main/axum-errors) - Returning errors when Axum handlers fail.

## Clap
[GitHub](https://github.com/clap-rs/clap) and [documentation](https://docs.rs/clap/latest/clap/).
+ [Argument Parsing](https://github.com/keithsharp/rust-experiments/tree/main/clap-basic-args) - Basic Clap argument parsing.
+ [Positional Arguments](https://github.com/keithsharp/rust-experiments/tree/main/clap-positional) - Experiment with Clap positional arguments.
+ [Subcommands](https://github.com/keithsharp/rust-experiments/tree/main/clap-subcommands) - Experiment with Clap subcommands and args.

## NATS
[Github](https://github.com/nats-io/nats.rs) and [documentation](https://docs.rs/async-nats/0.29.0/async_nats/).
+ [NATS Basic](https://github.com/keithsharp/rust-experiments/tree/main/nats-basic) - Basic pub/sub using NATS.

# Copyright and License
Copyright 2023, Keith Sharp, kms@passback.co.uk.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.