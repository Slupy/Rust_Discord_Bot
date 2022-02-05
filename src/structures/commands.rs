use serenity::framework::standard::macros::group;

use crate::commands::other::*;

#[group]
#[help_available(false)]
#[commands(ping)]
pub struct General;
