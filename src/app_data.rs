use crate::templates::helpers::register_helpers;
use handlebars::Handlebars;
use std::sync::Arc;

lazy_static! {
    /// Lazy Static to store app data at runtime.
    static ref APP_DATA: Arc<AppData> = {
        Arc::new(AppData::new())
    };
}

/// Struct to store shared app data and objects.
#[derive(Clone)]
pub struct AppData {
    /// The handlebars template registry.
    template_registry: Arc<Handlebars<'static>>,
}

impl AppData {
    /// Create new App Data object using the global static config.
    fn new() -> Self {
        // Register handlebars templates
        let mut template_registry = Handlebars::new();
        // If compiled in debug mode, do not cache templates
        if cfg!(debug_assertions) {
            template_registry.set_dev_mode(true);
        }
        template_registry
            .register_templates_directory(".hbs", "templates")
            .map_err(|e| {
                error!("Failed to properly register handlebars templates: {}", e);
                e
            })
            .unwrap();

        // We do not use handlebars strict mode anymore since it increasingly breaks templates
        // without warning.
        // template_registry.set_strict_mode(true);

        // Register the helpers defined in the helpers module.
        register_helpers(&mut template_registry);
        info!("Handlebars templates registered.");

        Self {
            template_registry: Arc::new(template_registry),
        }
    }

    /// Get an [`Arc`] reference to the global, lazily generated app-data.
    pub fn global() -> Arc<AppData> {
        APP_DATA.clone()
    }

    /// Get an [`Arc`] reference to the template registry.
    pub fn get_handlebars_registry(&self) -> Arc<Handlebars<'static>> {
        self.template_registry.clone()
    }
}
