extern crate gio;
extern crate gtk;
extern crate reqwest;
extern crate json;

use std::env::args;

use std::process;
use std::io;
use std::fs::File;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gio;
    use gtk;

    use gio::prelude::*;
    use gtk::prelude::*;

    use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

    use std::env::args;


    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("RenegadeX_launcher.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder.get_object("launcher_window").expect("Couldn't get window1");
        //let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
        //let dialog: MessageDialog = builder.get_object("messagedialog1")
        //                                   .expect("Couldn't get messagedialog1");

        window.set_application(application);
        window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });
/*
        bigbutton.connect_clicked(move |_| {
            dialog.run();
            dialog.hide();
        });
*/
        window.show_all();
    }

    pub fn main() {
        let application = gtk::Application::new("com.github.builder_basics",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
    }
}


#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example requires GTK 3.10 or later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}

pub struct Launcher {
  //for example: ~/RenegadeX/
  RenegadeX_location: Option<String>,
  //For example: DRI_PRIME=1
  env_arguments: Option<String>,
  player_name: Option<String>,
  servers: Option<json::JsonValue>,
  ping: Option<json::JsonValue>,
  x64_bit: bool
}

impl Launcher {
  pub fn new(game_folder: String) -> Launcher {
    Launcher {
      RenegadeX_location: Some(game_folder),
      env_arguments: None,
      player_name: None,
      servers: None,
      ping: None,
      x64_bit: true
    }
  }

  fn download_wine(&mut self, version: String) {
    //grab wine version from play-on-linux, wine itself, or from lutris.
    //...
    self.instantiate_wine_prefix();
  }

  //Checks if the (paranoid) kernel blocks ICMP by programs such as wine, otherwise prompt the user to enter password to execute the followiwng commands
  fn ping_test(&mut self) {
    let successful = false;
    
    if !successful {
      let mut wine_location = self.RenegadeX_location.clone().unwrap();
      wine_location.push_str("libs/wine/bin/wine64");
      let mut pkexec = process::Command::new("pkexec")
        .args(&["--user","root","setcap","cap_net_raw+epi",wine_location.as_str()])
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");
      pkexec.wait();
      /*
      Need to use pkexec to show the user a dialog questioning to allow executing setcap in order to allow the launcher (and wine?) to ping.
      https://wiki.archlinux.org/index.php/Polkit

      sudo setcap cap_net_raw+epi /usr/bin/wine-preloader
      sudo setcap cap_net_raw+epi /usr/bin/wine
      sudo setcap cap_net_raw+epi /usr/bin/wine64-preloader
      sudo setcap cap_net_raw+epi /usr/bin/wine64
      */
    }
  }

  //Checks if wine prefix exists, if not create it, install necessary libraries.
  fn instantiate_wine_prefix(&mut self) {
    //at the very least we need vcrun2005 and dotnet40 (or perhaps mono works)
    //what else? corefonts, vcrun2008 and vcrun2010 probs? xact,
    //overides?
    //At some point we may be able to use VK9 to improve performance.
    let mut wine_location = self.RenegadeX_location.clone().unwrap();
    wine_location.push_str("libs/wine/bin/wine64");
    //process::Command::new(wine_location)
  }

  pub fn refresh_server_list(&mut self) {
    
  }

  pub fn launch_game(&mut self, server_index: Option<u16>) -> std::process::Child {
    if server_index == None {
      let mut wine_location = self.RenegadeX_location.clone().unwrap();
      wine_location.push_str("libs/wine/bin/wine64");
      let mut game_location = self.RenegadeX_location.clone().unwrap();
      game_location.push_str("game_files/Binaries/Win64/UDK.exe");

      let mut wine_prefix = self.RenegadeX_location.clone().unwrap();
      wine_prefix.push_str("wine_instance/");
/*
      return process::Command::new("strace")
        .arg("-e")
        .arg("openat")
        .arg("-f")
        .arg(wine_location)
*/
      return process::Command::new(wine_location)
        //.env("WINEDEBUG","fixme-all,warn-dll,-heap")
        .env("WINEARCH","win64")
        .env("WINEPREFIX",wine_prefix)
        .env("DRI_PRIME", "1")
        .arg(game_location)
        //.arg("5.39.74.177:7777")
        .arg("-nomoviestartup")
        .arg("-ini:UDKGame:DefaultPlayer.Name=SonnyX")	
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");

    } else {

      let mut wine_location = self.RenegadeX_location.clone().unwrap();
      wine_location.push_str("libs/wine/bin/wine");
      let mut game_location = self.RenegadeX_location.clone().unwrap();
      game_location.push_str("game_files/Binaries/Win64/UDK.exe");

      return process::Command::new(wine_location)
        .arg(game_location)
        .arg("some server")
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn().expect("failed to execute child");
    }
  }

  /* Check if there are wine processes, if so prompt the user if these should be killed */
  pub fn kill_wine_instances() {
    let mut killall = process::Command::new("pkexec")
      .arg("killall")
      .arg("-9 wineserver winedevice.exe UDK.exe plugplay.exe services.exe explorer.exe mscorsvw.exe")
      .stdout(process::Stdio::piped())
      .stderr(process::Stdio::inherit())
      .spawn().expect("failed to execute child");
     killall.wait();
  }

}

#[cfg(test)]
mod tests {
  use super::*;
/*
  #[test]
  fn downloader() {
    let mut patcher : Downloader = Downloader::new("Linux".to_string());
    let update : bool = patcher.update_available();
    println!("{}", patcher.mirrors.unwrap().pretty(2 as u16));
 
    assert_eq!(update,true);
  }
*/
  #[test]
  fn launcher() {
    let mut launcher_instance : Launcher = Launcher::new("/home/sonny/RenegadeX/".to_string());
    let mut child = launcher_instance.launch_game(None);
    if child.wait().expect("failed to wait on child").success() {
      println!("Succesfully terminated wine");
      assert!(false);
    } else {
      println!("Child exited with exit code:");
      //Launcher::kill_wine_instances();
      assert!(false);
    }
  }
}


/*
pub fn update_game() -> Result<(), reqwest::Error> {
  //TODO: check if instuctions_hash has changed since last time the game was started and if the previous update was succesfully completed.
  let mirrors = &release_data["game"]["mirrors"];
  let mirror_url = format!("{}{}/", &mirrors[0]["url"], &release_data["game"]["patch_path"]);
  let instructions_url = format!("{}instructions.json", &mirror_url);
  println!("Downloading instructions.json:");
  let mirror_response = reqwest::get(&instructions_url)?.text()?;
  println!("Downloading complete! Rustifying!");
  let mirror_data = json::parse(&mirror_response).unwrap();
  println!("Rustifying complete! Showing first entry:");
  println!("{}", &mirror_data[0]);
  //probably the part where tokio should kick in!

  let first_file_download_url = format!("{}full/{}",&mirror_url,&mirror_data[0]["NewHash"]);
  let mut first_file_download_response = reqwest::get(&first_file_download_url)?;
  println!("Downloaded first file into memory!");
  let mut file_delta: Vec<u8> = vec![];
  let file_delta_size = match first_file_download_response.copy_to(&mut file_delta) {
    Ok(result) => result,
    Err(e) => panic!("Copy failed: {}", e)
  };
  if file_delta_size != mirror_data[0]["FullReplaceSize"].as_u64().unwrap() {
    panic!("delta file does not match the correct size.");
  }
  
  let mut slice: &[u8] = &file_delta;
  let mut dest = {
    let fname = "/home/sonny/eclipse-workspace/renegade_x_launcher/delta";
        match File::create(&fname) {
          Ok(file) => file,
          Err(e) => panic!("Error!")
        }
  };
  match io::copy(&mut slice, &mut dest) {
    Ok(o) => o,
    Err(e) => panic!("Error!")
  };

  Ok(())
}
*/
