#![no_std]

pub mod boot;
pub mod config;
pub mod console;
pub mod fs;
pub mod graphics;
pub mod guid;
pub mod memory;
pub mod runtime;
pub mod services;
pub mod status;
pub mod system;
pub mod table;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Handle(pub usize);
