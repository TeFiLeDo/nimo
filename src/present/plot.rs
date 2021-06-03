use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use plotters::prelude::*;
use tera::Context as Ctx;

pub fn prepare_images<'ctx>(
    data: &crate::Data,
    ctx: &'ctx mut Ctx,
    path: &str,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
) -> Result<&'ctx mut Ctx> {
    assert_eq!(from.is_some(), to.is_some());

    Ok(ctx)
}
