use std::collections::HashMap;

use dbus::arg::RefArg;

use crate::common::ControllerOpt;

pub(super) trait Controller {
    type Error;

    fn apply(
        options: &ControllerOpt,
        systemd_version: u32,
        properties: &mut HashMap<&str, Box<dyn RefArg>>,
    ) -> Result<(), Self::Error>;
}
