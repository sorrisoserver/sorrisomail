// SPDX-FileCopyrightText: MailCraft contributors
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Windows only: embed the app icon and version info (VERSIONINFO) into `mailcraft.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `MAILCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/mailcraft.ico");
    println!("cargo:rerun-if-changed=../../assets/sorriso-server/sorriso-server.ico");
    println!("cargo:rerun-if-env-changed=MAILCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    let branded_icon = "../../assets/sorriso-server/sorriso-server.ico";
    let icon_path = if std::path::Path::new(branded_icon).exists() { branded_icon } else { "../../assets/app-icon/mailcraft.ico" };
    res.set_icon(icon_path)
        .set("ProductName", "Sorriso Server Mail")
        .set("FileDescription", "Sorriso Server Mail - e-mail, calendario e contatos")
        .set("CompanyName", "Sorriso Server")
        .set("LegalCopyright", "Copyright (c) the MailCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "mailcraft.exe")
        .set("InternalName", "SorrisoServerMail");
    if let Err(e) = res.compile() {
        if std::env::var_os("MAILCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=mailcraft.exe built without icon/version resources: {e}");
    }
}
