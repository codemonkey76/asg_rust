mod app;

#[cfg(not(feature = "deploy"))]
pub use app::initialize_app;
