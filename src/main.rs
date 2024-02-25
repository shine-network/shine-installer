use std::{io::Read, path::PathBuf};

use installer::*;
use spinoff::*;
use steamlocate::SteamDir;
use colored::{control, Colorize};

use winsafe::{co, SHGetKnownFolderPath};

mod installer;

const ASCII_LINES: [&str; 23] = ["              .,ldOKNWMMWWWX0kdc,.              ..",
"           .;d0NWNXKKNMMMMMMMMMMNKd;.      .clc;  ",
"         .cONNNKd;...,dKWMMMMMMMMMMWXOkk0XNKx;.   ",
"        ,OWWNWX:       .cdOWMMMMMMMMMMMMMKo.      ",
"       ,0MWNWWd.   ';;.   .;xXMMMMMMMMMWk'        ",
"      .kMMWNWMx.   .kNk'     'l0WMMMMMM0'         ",
"      ,KMMWNWMXc   .xWWKl;.    .:kNMMMWd.         ",
"      .OMMMWNWMNx;..,oOXNNKx:.    'oKWM0,         ",
"       cXMMMWNNWWWKxlc:cooolc,      .;okxc'.      ",
"        :KWMMMWNNNNWMMWNXKK0kxdoc:;'              ",
"         .l0NMMMMWNXXXXXXXNNNNWMMMWNKkl,          ",
"           .,lxOXNWMMWNNNNNXXXXXXNWMMMMNk,        ",
"     .;l:,.     .';cloxk00KXNWMMWNNNWMMMMK;       ",
"      'OWN0d;.    .,clooool:,;cdKWMWWNWMMMk.      ",
"       lWMMMW0d,. .:kNMMMMNo    .cKMMWWMMM0'      ",
"      'OMMMMMMMNOl.  ,xXMMWx.     ,0MMWWMWd.      ",
"     ;0WMMMMMMMMMWXx;. ,ONXx'     .xMWNWWO'       ",
"  .;xXNNXNNWWMMMMMMMNOc..,.       ;0WNWNx.        ",
".,:cc;'...',:ld0NMMMMMWXxol;'..';o0NNXx;          ",
"               .,o0WMMMMMMMWNXXXNXKkl'            ",
"                  .:OWMMMMMWXOxoc,.               ",
"                    .c0WWXOc.                     ",
"                      ,Ox;.         "];

fn find_gmod() -> Result<PathBuf, ()> {
    let mut sp = Spinner::new(spinners::Point, "Garry's Mod aranıyor...", Color::Yellow);

    let mut steamdir = SteamDir::locate().unwrap();

    match steamdir.app(&4000) {
        Some(app) => {
            sp.success("Garry's Mod bulundu.".into());
            Ok(app.path.clone())
        },
        None => {
            sp.fail("Garry's Mod bulunamadı, yüklü olduğundan emin olun, eğer yüklü ise Discord sunucusundan teknik destek alınız.".into());
            Err(())
        }
    }
}

fn find_ts3() -> Result<PathBuf, ()> {
    let mut sp = Spinner::new(spinners::Point, "TS3 aranıyor...", Color::Yellow);

    let app_data = SHGetKnownFolderPath(&co::KNOWNFOLDERID::RoamingAppData, co::KF::DEFAULT, None).expect("AppData bulunamadı");

    let mut path = PathBuf::from(app_data);
    path.push("TS3Client");
    path.push("plugins");

    if path.exists() {
        sp.success("TS3 bulundu.".into());
        Ok(path)
    } else {
        sp.fail("TS3 bulunamadı, yüklü olduğundan emin olun, eğer yüklü ise Discord sunucusundan teknik destek alınız.".into());
        Err(())
    }
}

fn main() {
    if cfg!(windows) {
        control::set_virtual_terminal(true).unwrap();
    }

    ASCII_LINES.map(|l| println!("{}", l.yellow()));
    println!();
    println!();

    check_version().inspect_err(terminate).ok();

    let ts3_path = find_ts3().inspect_err(terminate).ok().unwrap();
    install_gspeak(ts3_path);

    let gmod_path = find_gmod().inspect_err(terminate).ok().unwrap();
    install_tslib(gmod_path.clone());

    println!();
    println!("----------------------");
    println!("{}, sunucuda vakit geçirirken Discord durumunuzu günceller.", "ShineSync".bold().yellow());
    println!();
    println!("{}", "ShineSync kullanıcıları Shine Hogwarts'da oynarken 2x XP kazanıyorlar!".bold().blue());
    println!("----------------------");
    println!();

    println!("ShineSync'i de kurmak ister misin? (Sunucuda 2x XP kazanmanı sağlar)");
    println!();
    println!("{}", "Yüklemek için ENTER'a bas, atlamak için n tuşuna basıp ENTER'a bas: ".dimmed());
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
    
    if buffer[0] == 110 {
        return terminate(&());
    }

    install_gdiscord(gmod_path);

    terminate(&());
}

fn terminate(_error: &()) -> () {
    let _ = press_btn_continue::wait("Herhangi bir tuşa basarak programdan çıkabilirsin...");
    std::process::exit(0);
}