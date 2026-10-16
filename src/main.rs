mod cli;

use anyhow::Result;
use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::Parser;
use cli::Cli;
use git2::{Commit, Repository};
use languagetool_rust::api::check::{Match, Request};
use languagetool_rust::api::server::ServerClient;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!\n");

    // cli
    let args = Cli::parse();

    // get lt client
    let hostname = "http://".to_owned() + &args.address;
    let client = ServerClient::new(&hostname, &args.port);
    let ping = client.ping().await?;
    println!("client ping with: {}ms", ping);

    // get git metadata
    let current_dir = env::current_dir()?;
    println!("using current directory at: {:?}", current_dir);

    let repository = Repository::open(current_dir)?;
    let commit = repository.head()?.peel_to_commit()?;
    println!(
        "found HEAD with commit sha {:?} and message {:?}",
        commit.id(),
        commit.summary()?.unwrap_or("")
    );

    let commit_message_body = match commit.body()? {
        Some(message) => message,
        None => {
            println!("no commit message body found");
            return Ok(());
        }
    };

    println!("checking commit message body: {:?}", commit_message_body);

    // send to lt
    let mut request = Request::new()
        .with_language("auto".to_owned())
        .with_text(commit_message_body);
    request.level = languagetool_rust::api::check::Level::Picky;

    println!(
        "request constructed with language: {:?} and level {:?}",
        request.language, request.level
    );

    let response = client.check(&request).await?;
    println!(
        "received response with {:?} remarks",
        response.matches.len()
    );

    // evaluate lt response
    println!(
        "found {} sentences in the tested text\n",
        response
            .sentence_ranges
            .map_or(0, |ranges| { ranges.len() })
    );

    if response.matches.is_empty() {
        println!("no errors found");
        return Ok(());
    }

    for error in response.matches {
        display_error(&commit, &error);
    }

    // TODO: interactively apply one of the replacement proposals

    println!("\ndone");
    Ok(())
}

fn display_error(commit: &Commit, error: &Match) {
    // TODO make debug output an option
    //println!("\n{:#?}\n", error);

    let commit_hash = &commit.id().to_string();

    // TODO map error.rule.issue_type to ariadne error/warning/advice

    Report::build(
        ReportKind::Error,
        (
            commit_hash,
            error.context.offset..(error.context.offset + error.context.length),
        ),
    )
    .with_message(error.rule.description.to_string())
    .with_label(
        Label::new((
            commit_hash,
            error.context.offset..error.context.offset + error.context.length,
        ))
        .with_message(format!("{:?}", error.replacements))
        .with_color(Color::Fixed(214)),
    )
    .finish()
    .print((commit_hash, Source::from(error.context.text.clone())))
    .unwrap();
}
