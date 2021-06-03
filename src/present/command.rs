use std::fs::{create_dir_all, write};


use super::plot::prepare_images;
use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use include_dir::{include_dir, Dir};
use tera::{Context as Ctx, Tera};

static ASSETS: Dir = include_dir!("./present/assets");
static ENTRY_POINTS: [&str; 2] = ["index.html", "presentation.html"];
static TEMPLATES: Dir = include_dir!("./present/templates");
static TMP: &str = "/tmp/nimo/present";

#[derive(Debug, structopt::StructOpt)]
/// Creates a static website with a detailed report
pub struct Command {}

impl Command {
    pub fn execute(&self, data: &crate::Data) -> Result<()> {
        // create output directory
        create_dir_all(TMP)
            .context(format!(r#"failed to create output directory at "{}""#, TMP))?;

        // write out assets
        add_asset_dir(&ASSETS).context("failed to add assets")?;

        // prepare templates
        let mut tera = Tera::default();
        add_template_dir(&TEMPLATES, &mut tera).context("failed to add templates")?;
        for file in TEMPLATES.files() {
            if ENTRY_POINTS.contains(&file.path) {
                tera.add_raw_template(file.path, file.contents_utf8().unwrap())
                    .context(format!(
                        r#"failed to add template "{}""#,
                        file.path().display()
                    ))?;
            }
        }

        // prepare context
        let mut ctx = Ctx::new();
        ctx.insert("generated", &Utc::now());

        // render basic templates
        write(
            format!("{}/{}", TMP, "index.html"),
            tera.render("index.html", &ctx)
                .context(r#"failed to render "index""#)?,
        )
        .context(r#"failed to write "index.html""#)?;

        // render current
        let cur = vec![
            ("all-time.html", "All Time", None, None),
            (
                "today.html",
                "Today",
                Some(Utc::now().date().and_hms(0, 0, 0)),
                Some(Utc::now().date().and_hms(23, 59, 59)),
            ),
            {
                let d = Utc::now().date() - Duration::days(1);
                (
                    "yesterday.html",
                    "Yesterday",
                    Some(d.and_hms(0, 0, 0)),
                    Some(d.and_hms(23, 59, 59)),
                )
            },
        ];
        for (name, title, from, to) in cur {
            let mut ctx = ctx.clone();
            ctx.insert("title", title);

            let path = format!("{}/{}", TMP, name);
            prepare_images(&data, &mut ctx, &path, from, to)
                .context(format!(r#"failed to prepare images for "{}""#, name))?;

            let content = tera
                .render("presentation.html", &ctx)
                .context(format!(r#"failed to render "{}""#, name))?;

            write(format!("{}/{}", TMP, name), content)
                .context(format!(r#"failed to write "{}""#, name))?;
        }

        Ok(())
    }
}

fn add_asset_dir(dir: &Dir) -> Result<()> {
    for dir in dir.dirs() {
        add_asset_dir(dir)?;
    }

    create_dir_all(format!("{}/{}", TMP, dir.path)).context(format!(
        r#"failed to create output directory "{}""#,
        dir.path
    ))?;

    for file in dir.files() {
        write(
            format!("{}/{}", TMP, file.path),
            file.contents_utf8().unwrap(),
        )
        .context(format!(r#"failed to write asset file "{}""#, file.path))?;
    }

    Ok(())
}

fn add_template_dir(dir: &Dir, tera: &mut Tera) -> Result<()> {
    for dir in dir.dirs() {
        add_template_dir(dir, tera)?;
    }

    for file in dir.files() {
        if ENTRY_POINTS.contains(&file.path) {
            continue;
        }

        tera.add_raw_template(file.path, file.contents_utf8().unwrap())
            .context(format!(
                r#"failed to add template "{}""#,
                file.path().display()
            ))?;
    }

    Ok(())
}
