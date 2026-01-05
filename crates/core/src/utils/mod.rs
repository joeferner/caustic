pub mod orthonormal_basis;
pub mod perlin;

pub use orthonormal_basis::OrthonormalBasis;
pub use perlin::Perlin;

#[cfg(not(target_arch = "wasm32"))]
pub fn to_absolute(path: &str) -> std::io::Result<std::path::PathBuf> {
    use std::env;
    use std::path::Path;

    let path = Path::new(path);

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}
