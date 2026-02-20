// Challenge 1.3 - Struct Update Syntax (and its Trap)
//
// Define `ServerConfig`, create `default_config`, then build `custom_config` with:
// `ServerConfig { port: 3000, verbose: true, ..default_config }`
//
// Show which fields from `default_config` are still usable after the update.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub verbose: bool,
}

pub fn create_default_config() -> ServerConfig {
    ServerConfig {
        host: String::new(),
        port: 0,
        max_connections: 0,
        verbose: false,
    }
}

pub fn custom_with_surviving_defaults() -> (ServerConfig, u16, u32, bool) {
    let default_config = create_default_config();
    let custom_config = ServerConfig {
        port: 3000,
        verbose: true,
        ..default_config
    };

    (custom_config, 0, 0, false)
}

pub fn host_is_moved_after_update() -> bool {
    false
}


// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::{custom_with_surviving_defaults, create_default_config, host_is_moved_after_update};

    #[test]
    fn default_config_has_prompt_values() {
        let default_config = create_default_config();

        assert_eq!(
            default_config.host, "localhost",
            "Default host should be 'localhost'. Got '{}'.",
            default_config.host
        );
        assert_eq!(
            default_config.port, 8080,
            "Default port should be 8080. Got {}.",
            default_config.port
        );
        assert_eq!(
            default_config.max_connections, 100,
            "Default max_connections should be 100. Got {}.",
            default_config.max_connections
        );
        assert!(
            !default_config.verbose,
            "Default verbose should be false. Got true."
        );
    }

    #[test]
    fn struct_update_moves_non_copy_but_keeps_copy_fields_usable() {
        let (custom, port, max_connections, verbose) = custom_with_surviving_defaults();

        assert_eq!(custom.port, 3000, "Custom port should be 3000. Got {}.", custom.port);
        assert!(custom.verbose, "Custom verbose should be true. Got false.");
        assert_eq!(
            custom.host, "localhost",
            "Custom should inherit host='localhost' from default via update syntax. Got '{}'.",
            custom.host
        );

        assert_eq!(port, 8080, "`default_config.port` should still be usable (Copy). Got {}.", port);
        assert_eq!(
            max_connections, 100,
            "`default_config.max_connections` should still be usable (Copy). Got {}.",
            max_connections
        );
        assert!(
            !verbose,
            "`default_config.verbose` should still be usable (Copy) and false."
        );
    }

    #[test]
    fn host_field_move_is_reported() {
        assert!(
            host_is_moved_after_update(),
            "`host` is String (non-Copy), so update syntax should move it from default config."
        );
    }
}
