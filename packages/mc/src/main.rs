// logs, -pl prismlauncher
use clap::{ Arg, Command as ClapCommand};
use std::process::{Command, exit};
use toml::Value;

fn main() {
  let (pack_name, pack_version) = load_pack_toml("pack.toml");
  let base_dir = format!("./instances/{}{}/.minecraft", pack_name, pack_version);
  let instance_dir = format!("./instances/{}{}", pack_name, pack_version);


  let matches = ClapCommand::new("mc")
    .about("Modpack helper command")
    .arg(
      Arg::new("command")
        .help("One of: mr, rf, r, fixup")
        .required(true)
        .index(1),
    )

    .arg(
      Arg::new("args")
        .help ("Extra args")
        .index(2)
        .num_args(0..),
    )
    .get_matches();

  let command = matches.get_one::<String>("command").unwrap();
  let args: Vec<&str> = matches
    .get_many::<String>("args")
    .unwrap_or_default()
    .map(String::as_str)
    .collect();;

  match command.as_str() {
    "mr" || "modrinth" => {
      let mut full_command = vec!["modrinth"];
      if let Some(first_arg) = args.get(0) {
        match *first_arg {
          "d" | "download" => full_command.push("download"),
          "export" => full_command.push("export"),
          _ => full_command.push(first_arg),
        }
      }
      full_command.extend(&args[1..]);
      exec("packwiz", &full_command);
    }
    "rf" || "refresh" => {
      exec("packwiz", &["refresh"]);
    }
    "r" | "run" => {
      let mut pmc_args = vec![
        "--main-dir",
        &base_dir,
        "-v",
        "start",
        "fabric:1.20.1",
      ];

      let mut pmc_args_dry = pmc_args.clone();
      pmc_args_dry.push("--dry");

      let mut mrpack_args = vec![
        "./*.mrpack",
        &base_dir
      ];

      let is_dry_run = args.contains(&"--dry") || args.contains(&"-d");
      let use_prism_launcher = args.contains(&"--prism-launcher") || args.contains(&"-pl");



      if use_prism_launcher {
        let mut pl_args = vec![
          "-I", 
          "./*.mrpack"
        ];
        if exec("prismlauncher", &pl_args);


      } else {
        mcrun(
          &base_dir,
          &pmc_args,
          &pmc_args_dry,
          &mrpack_args
          
        )
      }
    }
    "fixup" => {
      exec("minecraft-fixup", &[]);
    }
    "prune" => {
      exec("rm -r {}", &instance_dir);
    }
    _ => {
      eprintln! ("Unknown command: {}", command);
      exit(1);
    }
  }
}        

fn mcrun () {
  if !check_for_installation( &base_dir ) { //check for installation returns 0 (it exists)
    run(
      &base_dir
      &pmc_args
    )
  } else { //install and run runless dry
    in install (
    install(
    run(
      &base_dir
      &pmc_args
    )

    
    
    
      


fn check_for_installation () {
  if std::path:Path::new(base_dir).exists() {
  println!("Installation exists");
    return(0);
  } else {
    return(1);
  }
}


fn run(
  base_dir: &str,
  pmc_args: &[&str],
) {
  if std::path:Path::new(base_dir).exists() {
    exec("portablemc", pmc_args);
  } else {
    install()
  }
}
    

fn install(
  pmc_args: &[&str],
  pmc_args_dry: &[&str],
  mrpack_args: &[&str],
  is_dry_run: bool,
) {
  println!("Installing...");
  if exec("portablemc", pmc_args_dry) {
    if exec("mrpack-install", mrpack_args) {
      if !is_dry_run {
        println!("Running...");
        exec("portablemc", pmc_args);
      } else {
        println! ("Finishing dry run");
      }
    } else {
      eprintln!("Failed to install with mrpack-install")
    exit(1);
    }
  } else {
    eprintln!("Failed to dry run with portablemc.");
    exit(1);
  }
}

fn exec(program: &str, args: &[&str]) {
  println!("Running command: {} {:?}", program, args);
  let status = Command::new(program)
    .args(args)
    .status();
  match status {
    Ok(exit_status) if exit_status.success() => { 
      exit(0)
    }
    Ok(exit_status) => {
      eprintln!("Command failed with exit code: {}", exit_status);
      exit(exit_status.code().unwrap_or(1));
    }
    Err(e) => {
      eprintln!("Failed to execute date command: {}", e);
      exit(1);
    }
  }
}

fn load_pack_toml(path: &str -> (String, String) {
  let content = stg::fs::read_to_string(path).expect("Failed to read pack.toml");
  let value = content.parse::<value>().expect("Failed to parse TOML");
  let pack_name = value["name"]
    .as_str()
    .expect("Missing 'name' in pack.toml")
    .to_string();
  let pack_version = value["version"]
    .as_str()
    .expect("Missing 'version' in pack.toml")
    .to_string();
  (pack_name, pack_version)
}
  
