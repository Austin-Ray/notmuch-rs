//! Helper functions for unit testing.
// Due to how cargo compiles tests, some modules will report dead code.
#![allow(dead_code)]
use maildir::Maildir;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{fs::File, path::PathBuf};
use tempfile::{tempdir, TempDir};

/// Rust representation of the testing environment.
pub struct TestEnv {
    pub root: TempDir,
    pub maildir: Maildir,
    /// Location of the notmuch configuration.
    pub nm_cfg: PathBuf,
}

impl TestEnv {
    pub fn add_email(&self, email: lettre::Message) {
        self.maildir
            .store_cur_with_flags(&email.formatted(), "")
            .unwrap();
    }

    pub fn new() -> Self {
        let tmp_dir = tempdir().unwrap();

        // Create a maildir in our tmpdir
        let mut mail_path = tmp_dir.path().to_path_buf();
        mail_path = mail_path.join("mail");
        fs::create_dir(&mail_path).unwrap();

        // Create maildir folders
        fs::create_dir(&mail_path.join("cur")).unwrap();
        fs::create_dir(&mail_path.join("new")).unwrap();
        fs::create_dir(&mail_path.join("tmp")).unwrap();

        let maildir = Maildir::from(mail_path.clone());

        // Create a .notmuch-config file with maildir path
        let mut nm_cfg = tmp_dir.path().to_path_buf();
        nm_cfg = nm_cfg.join(".notmuch-config");

        let mut nm_cfg_file = File::create(&nm_cfg).unwrap();
        write!(
            nm_cfg_file,
            r#"
            [database]
            path={}
            "#,
            mail_path.to_str().unwrap(),
        )
        .unwrap();

        let env = Self {
            root: tmp_dir,
            maildir,
            nm_cfg,
        };

        // Create the database.
        env.notmuch(["new"]);

        env
    }

    /// Call the `notmuch` binary.
    pub fn notmuch<I, S>(&self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new("notmuch")
            .env("NOTMUCH_CONFIG", &self.nm_cfg)
            .args(args)
            .stdout(Stdio::null()) // Mute stdout. Only care about errors.
            .status()
            .unwrap();
    }
}
