use std::net::{IpAddr, Ipv4Addr};

pub mod env {
    pub const API_HOST_VAR: &str = "MULLVAD_API_HOST";
    pub const API_ADDR_VAR: &str = "MULLVAD_API_ADDR";
    pub const API_FORCE_DIRECT_VAR: &str = "MULLVAD_API_FORCE_DIRECT";
    pub const DISABLE_TLS_VAR: &str = "MULLVAD_API_DISABLE_TLS";
    pub const SIGSUM_TRUSTED_PUBKEYS_VAR: &str = "MULLVAD_SIGSUM_TRUSTED_PUBKEYS";
}

// SocialCommand VPN gateway defaults.
//
// Live deployment (2026-11-08): the gateway runs on Sameer's VPS and is served by
// Caddy with a public Let's Encrypt certificate (TLS 1.3 only, `default_sni`
// because the client sends no SNI).  Override at runtime with MULLVAD_API_HOST /
// MULLVAD_API_ADDR in development builds (the `api-override` feature); release
// builds always use these constants.
pub const API_HOST_DEFAULT: &str = "vpn.socialcommand.site";
pub const PARTNER_API_HOST_DEFAULT: &str = "vpn.socialcommand.site";
pub const API_IP_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(169, 58, 191, 8));
pub const API_PORT_DEFAULT: u16 = 443;
pub const SIGSUM_TRUSTED_PUBKEYS_DEFAULT: &str = include_str!("trusted-sigsum-signing-pubkeys");

pub const API_HOST_STAGING: &str = "api.stagemole.eu";
pub const PARTNER_API_HOST_STAGING: &str = "partner.stagemole.eu";
pub const API_IP_STAGING: IpAddr = IpAddr::V4(Ipv4Addr::new(185, 217, 116, 129));
pub const API_PORT_STAGING: u16 = 443;
