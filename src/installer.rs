use std::path::PathBuf;

use reqwest::blocking::Client;
use spinoff::*;

const TS3_PLUGIN_64: &str = "https://github.com/cborac/gspeak-binaries/raw/main/gspeak_win64.dll";
const TS3_PLUGIN_32: &str = "https://github.com/cborac/gspeak-binaries/raw/main/gspeak_win32.dll";

const GMOD_PLUGIN_64: &str =
    "https://github.com/cborac/gspeak-binaries/raw/main/gmcl_tslib_win64.dll";
const GMOD_PLUGIN_32: &str =
    "https://github.com/cborac/gspeak-binaries/raw/main/gmcl_tslib_win32.dll";

const SYNC_PLUGIN_64: &str =
    "https://github.com/cborac/gspeak-binaries/raw/main/gmcl_shinesync_win64.dll";
const SYNC_PLUGIN_32: &str =
    "https://github.com/cborac/gspeak-binaries/raw/main/gmcl_shinesync_win32.dll";

const VERSION_URL: &str = "https://github.com/cborac/gspeak-binaries/raw/main/VERSION";
const VERSION: &str = "3";

pub fn check_version() -> Result<(), ()> {
    let mut sp = Spinner::new(spinners::Point, "Versiyon doğrulanıyor...", Color::Yellow);

    let client = Client::new();

    let res = client
        .get(VERSION_URL)
        .send()
        .expect("Dosya Hizmetlerine ulaşılamadı.");

    let status = res.status();

    if !status.is_success() {
        sp.fail(&format!("Dosya Hizmetleri {} ile cevap verdi.", status.as_u16()));
        return Err(())
    }

    let current_version = res.text().unwrap();

    if current_version.trim() == VERSION {
        sp.success(&format!("Versiyon doğrulandı (v{})", VERSION));
        Ok(())
    } else {
        sp.fail(&format!("Versiyon uyuşmazlığı. Lütfen programı güncelleyin. (Yüklü: v{} - Güncel Versiyon: v{})", VERSION, current_version.trim()));
        Err(())
    }
}

fn install_to(url: &'static str, path: &PathBuf) -> Result<(), String> {
    let client = Client::new();

    let res = client
        .get(url)
        .send()
        .expect("Dosya Hizmetlerine ulaşılamadı.");

    let status = res.status();

    if !status.is_success() {
        return Err(format!("Dosya Hizmetleri {} ile cevap verdi.", status.as_u16()))
    }

    std::fs::write(path, res.bytes().map_err(|_| "Dosya açılamadı".to_string())?).map_err(|err| format!("Dosya yazılamadı: {}", err))?;

    Ok(())
}

pub fn install_gspeak(_path: PathBuf) {
    let mut sp: Spinner = Spinner::new(spinners::Point, "GSpeak indiriliyor... (TS3)", Color::Yellow);

    let mut path = _path;
    path.push("gspeak_win64.dll");
    let _ = install_to(TS3_PLUGIN_64, &path);

    path.set_file_name("gspeak_win32.dll");
    let _ = install_to(TS3_PLUGIN_32, &path);

    
    sp.success("GSpeak indirildi (TS3)");
}

pub fn install_tslib(_path: PathBuf) {
    let mut sp = Spinner::new(spinners::Point, "GSpeak indiriliyor... (Garry's Mod)", Color::Yellow);

    let mut path = _path;
    path.push("gmcl_tslib_win64.dll");
    let _ = install_to(GMOD_PLUGIN_64, &path);

    path.set_file_name("gmcl_tslib_win32.dll");
    let _ = install_to(GMOD_PLUGIN_32, &path);

    sp.success("GSpeak indirildi (Garry's Mod)");
}

pub fn install_gdiscord(_path: PathBuf) {
    let mut sp = Spinner::new(spinners::Point, "ShineSync indiriliyor... (Garry's Mod)", Color::Yellow);

    let mut path = _path;
    path.push("gmcl_shinesync_win64.dll");
    let _ = install_to(SYNC_PLUGIN_64, &path);

    path.set_file_name("gmcl_shinesync_win32.dll");
    let _ = install_to(SYNC_PLUGIN_32, &path);

    
    sp.success("ShineSync indirildi (Garry's Mod)");
}
