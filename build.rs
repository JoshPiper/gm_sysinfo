fn main() {
    // Captured for sysinfo.get_build_info()'s repository/run_url fields so a
    // downloaded binary can be traced back to the CI run that produced it.
    // Empty on non-CI builds; env! (not option_env!) is safe because these
    // rustc-env vars are always emitted, just possibly blank.
    println!("cargo:rerun-if-env-changed=GITHUB_REPOSITORY");
    println!("cargo:rerun-if-env-changed=GITHUB_SERVER_URL");
    println!("cargo:rerun-if-env-changed=GITHUB_RUN_ID");

    let repository = std::env::var("GITHUB_REPOSITORY").unwrap_or_default();
    let run_id = std::env::var("GITHUB_RUN_ID").unwrap_or_default();
    let run_url = if repository.is_empty() || run_id.is_empty() {
        String::new()
    } else {
        let server_url =
            std::env::var("GITHUB_SERVER_URL").unwrap_or_else(|_| "https://github.com".to_string());
        format!("{server_url}/{repository}/actions/runs/{run_id}")
    };

    println!("cargo:rustc-env=BUILD_REPOSITORY={repository}");
    println!("cargo:rustc-env=BUILD_RUN_URL={run_url}");

    built::write_built_file().expect("Failed to acquire build-time information");
}
