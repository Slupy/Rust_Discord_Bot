use std::collections::HashSet;

use crate::structures::commands::*;
use serenity::{
    framework::standard::StandardFramework,
    model::id::UserId,
};

pub fn get_framework(default_prefix: &str, owners: HashSet<UserId>) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| c.prefix(default_prefix).owners(owners))
        .group(&GENERAL_GROUP)
}
