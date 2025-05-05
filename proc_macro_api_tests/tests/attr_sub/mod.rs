#[cfg(not(feature = "deny_group_attr"))]
mod attr_gp;

#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_append"))]
mod attr_gp_ap;

#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_append"))]
#[cfg(not(feature = "deny_override"))]
mod attr_gp_ap_ov;

#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_override"))]
mod attr_gp_ov;

#[cfg(not(feature = "deny_group_attr"))]
#[cfg(not(feature = "deny_override"))]
#[cfg(not(feature = "deny_shadow"))]
mod attr_gp_ov_sh;
