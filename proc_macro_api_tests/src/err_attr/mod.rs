#[cfg(not(feature = "with_default"))]
#[allow(unused)]
use api;
#[cfg(feature = "with_default")]
#[allow(unused)]
use error as api;

mod err_gp;

mod err_ap;

mod err_ov;

mod err_sh;
