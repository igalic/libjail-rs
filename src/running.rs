use param;
use sys;
use JailError;
use StoppedJail;

use std::collections::HashMap;
use std::net;
use std::path;

/// Represents a running jail.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
#[cfg(target_os = "freebsd")]
pub struct RunningJail {
    /// The `jid` of the jail
    pub jid: i32,
}

/// Represent a running jail.
#[cfg(target_os = "freebsd")]
impl RunningJail {
    /// Create a [RunningJail](struct.RunningJail.html) instance given a `jid`.
    ///
    /// No checks will be performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use jail::RunningJail;
    /// # use jail::StoppedJail;
    /// # let jail = StoppedJail::new("/rescue")
    /// #     .name("testjail_from_jid")
    /// #     .start()
    /// #     .expect("could not start jail");
    /// # let jid = jail.jid;
    ///
    /// let running = RunningJail::from_jid(jid);
    /// # running.kill();
    /// ```
    pub fn from_jid(jid: i32) -> RunningJail {
        RunningJail { jid }
    }

    /// Create a [RunningJail](struct.RunningJail.html) given the jail `name`.
    ///
    /// The `jid` will be internally resolved using
    /// [jail_getid](fn.jail_getid.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use jail::RunningJail;
    /// # use jail::StoppedJail;
    /// # let jail = StoppedJail::new("/rescue")
    /// #     .name("testjail_from_name")
    /// #     .start()
    /// #     .expect("could not start jail");
    ///
    /// let running = RunningJail::from_name("testjail_from_name")
    ///     .expect("Could not get testjail");
    /// #
    /// # running.kill();
    /// ```
    pub fn from_name(name: &str) -> Result<RunningJail, JailError> {
        sys::jail_getid(name).map(RunningJail::from_jid)
    }

    /// Return the jail's `name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// #
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_name")
    /// #     .start()
    /// #     .expect("Could not start jail");
    /// assert_eq!(running.name().unwrap(), "testjail_name");
    /// #
    /// # running.kill();
    /// ```
    pub fn name(self: &RunningJail) -> Result<String, JailError> {
        self.param("name")?.unpack_string()
    }

    /// Return the jail's `path`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// # use std::path::PathBuf;
    /// #
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_path")
    /// #     .start()
    /// #     .expect("Could not start jail");
    /// let path = running.path()
    ///     .expect("Could not get path");
    /// # let expected : PathBuf = "/rescue".into();
    /// # assert_eq!(path, expected);
    /// #
    /// # running.kill();
    /// ```
    pub fn path(self: &RunningJail) -> Result<path::PathBuf, JailError> {
        Ok(self.param("path")?.unpack_string()?.into())
    }

    /// Return the jail's `name`.
    ///
    /// The name will be internall resolved using
    /// [jail_getname](fn.jail_getname.html).
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// #
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_name")
    /// #     .hostname("testjail.example.com")
    /// #     .start()
    /// #     .expect("Could not start jail");
    /// assert_eq!(running.hostname().unwrap(), "testjail.example.com");
    /// #
    /// # running.kill();
    /// ```
    pub fn hostname(self: &RunningJail) -> Result<String, JailError> {
        self.param("host.hostname")?.unpack_string()
    }

    /// Get the IP addresses
    ///
    /// # Examples
    /// ```
    /// # use jail::StoppedJail;
    /// # use std::net::IpAddr;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_ip")
    /// #     .ip("127.0.1.2".parse().unwrap())
    /// #     .ip("fe80::2".parse().unwrap())
    /// #     .start()
    /// #     .expect("Could not start jail");
    /// let ips = running.ips()
    ///     .expect("could not get ip addresses");
    /// assert_eq!(ips[0], "127.0.1.2".parse::<IpAddr>().unwrap());
    /// assert_eq!(ips[1], "fe80::2".parse::<IpAddr>().unwrap());
    /// # running.kill();
    /// ```
    pub fn ips(self: &RunningJail) -> Result<Vec<net::IpAddr>, JailError> {
        let mut ips: Vec<net::IpAddr> = vec![];
        ips.extend(
            self.param("ip4.addr")?
                .unpack_ipv4()?
                .iter()
                .cloned()
                .map(net::IpAddr::V4),
        );
        ips.extend(
            self.param("ip6.addr")?
                .unpack_ipv6()?
                .iter()
                .cloned()
                .map(net::IpAddr::V6),
        );
        Ok(ips)
    }

    /// Return a jail parameter.
    ///
    /// # Examples
    /// ```
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .start().unwrap();
    /// #
    /// let hostuuid = running.param("host.hostuuid")
    ///     .expect("could not get jail hostuuid");
    /// #
    /// # println!("jail uuid: {:?}", hostuuid);
    /// # running.kill();
    /// ```
    pub fn param(self: &Self, name: &str) -> Result<param::Value, JailError> {
        param::get(self.jid, name)
    }

    /// Return a HashMap of all jail parameters.
    ///
    /// # Examples
    /// ```
    /// use jail::param;
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_params")
    /// #     .param("allow.raw_sockets", param::Value::Int(1))
    /// #     .start()
    /// #     .expect("could not start jail");
    ///
    /// let params = running.params()
    ///     .expect("could not get all parameters");
    ///
    /// assert_eq!(
    ///     params.get("allow.raw_sockets"),
    ///     Some(&param::Value::Int(1))
    /// );
    /// # running.kill().expect("could not stop jail");
    /// ```
    pub fn params(self: &Self) -> Result<HashMap<String, param::Value>, JailError> {
        param::get_all(self.jid)
    }

    /// Set a jail parameter.
    ///
    /// # Examples
    /// ```
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .start().unwrap();
    /// #
    /// use jail::param;
    /// running.param_set("allow.raw_sockets", param::Value::Int(1))
    ///     .expect("could not set parameter");
    /// # let readback = running.param("allow.raw_sockets")
    /// #   .expect("could not read back value");
    /// # assert_eq!(readback, param::Value::Int(1));
    /// # running.kill();
    /// ```
    pub fn param_set(self: &Self, name: &str, value: param::Value) -> Result<(), JailError> {
        param::set(self.jid, name, value)
    }

    /// Kill a running jail, consuming it.
    ///
    /// This will kill all processes belonging to the jail, and remove any
    /// children of that jail.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .start().unwrap();
    /// running.kill();
    /// ```
    pub fn kill(self: RunningJail) -> Result<(), JailError> {
        sys::jail_remove(self.jid).and_then(|_| Ok(()))
    }

    /// Create a StoppedJail from a RunningJail, while not consuming the
    /// RunningJail.
    ///
    /// This can be used to clone the config from a RunningJail.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_save")
    /// #     .hostname("testjail_save.example.com")
    /// #     .start()
    /// #     .unwrap();
    /// let stopped = running
    ///     .save()
    ///     .expect("could not save jail configuration");
    ///
    /// assert_eq!(stopped.name, Some("testjail_save".into()));
    /// assert_eq!(stopped.hostname, Some("testjail_save.example.com".into()));
    /// # running.kill().unwrap();
    /// ```
    pub fn save(self: &RunningJail) -> Result<StoppedJail, JailError> {
        let mut stopped = StoppedJail::new(self.path()?);

        stopped.name = self.name().ok();
        stopped.hostname = self.hostname().ok();
        stopped.ips = self.ips()?;
        stopped.params = self.params()?;

        Ok(stopped)
    }

    /// Stop a jail, keeping its configuration in a StoppedJail.
    ///
    /// This is a wrapper around `save` and `kill`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .name("testjail_stop")
    /// #     .hostname("testjail_stop.example.com")
    /// #     .start()
    /// #     .unwrap();
    /// let stopped = running
    ///     .stop()
    ///     .expect("failed to stop jail");
    ///
    /// //assert_eq!(stopped.name, Some("testjail_save".into()));
    /// //assert_eq!(stopped.hostname, Some("testjail_save.example.com".into()));
    /// ```
    pub fn stop(self: RunningJail) -> Result<StoppedJail, JailError> {
        let stopped = self.save()?;
        self.kill()?;

        Ok(stopped)
    }

    /// Restart a jail by stopping it and starting it again
    ///
    /// This is a wrapper around `RunningJail::stop` and `StoppedJail::start`
    ///
    /// # Examples
    ///
    /// ```
    /// # use jail::StoppedJail;
    /// # let running = StoppedJail::new("/rescue")
    /// #     .start()
    /// #     .unwrap();
    ///
    /// let old_jid = running.jid;
    /// let running = running.restart()
    ///     .expect("failed to restart jail");
    /// assert!(running.jid != old_jid);
    ///
    /// # running.kill();
    /// ```
    pub fn restart(self: RunningJail) -> Result<RunningJail, JailError> {
        let stopped = self.stop()?;
        stopped.start()
    }

    /// Returns an Iterator over all running jails on this host.
    ///
    /// # Examples
    ///
    /// ```
    /// use jail::RunningJail;
    /// # use jail::StoppedJail;
    /// # let mut running_jails: Vec<RunningJail> = (1..5)
    /// #     .map(|i| {
    /// #         StoppedJail::new("/rescue")
    /// #             .name(format!("testjail_iterate_{}", i))
    /// #             .start()
    /// #             .expect("failed to start jail")
    /// #     })
    /// #     .collect();
    ///
    /// for running in RunningJail::all() {
    ///     println!("jail: {}", running.name().unwrap());
    /// }
    /// #
    /// # for to_kill in running_jails.drain(..) {
    /// #     to_kill.kill().expect("failed to kill jail");
    /// # }
    /// ```
    pub fn all() -> RunningJails {
        RunningJails::default()
    }
}

/// An Iterator over running Jails
///
/// See [RunningJail::all()](struct.RunningJail.html#method.all) for a usage
/// example.
#[cfg(target_os = "freebsd")]
pub struct RunningJails {
    lastjid: i32,
}

#[cfg(target_os = "freebsd")]
impl Default for RunningJails {
    fn default() -> Self {
        RunningJails { lastjid: 0 }
    }
}

#[cfg(target_os = "freebsd")]
impl RunningJails {
    pub fn new() -> Self {
        RunningJails::default()
    }
}

#[cfg(target_os = "freebsd")]
impl Iterator for RunningJails {
    type Item = RunningJail;

    fn next(&mut self) -> Option<RunningJail> {
        let jid = match sys::jail_nextjid(self.lastjid) {
            Ok(j) => j,
            Err(_) => return None,
        };

        self.lastjid = jid;

        Some(RunningJail { jid })
    }
}
