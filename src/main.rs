fn main() {
    // let image = "alpine:3";
    // let arch = "arm64";

    let apk = ApkPackageListLayer::update("2023-05-02");

    install_node(&apk);
    // install_python3();
    // let rust = install_rustup("1.69");

    // step(
    //     "install cargo-lambda",
    //     ["pip3 install cargo-lambda"],
    //     [rust],
    // );

    // let git = github_checkout();

    // let servers = [
    //     "clip-segment-saver",
    //     "clipper",
    //     "clip-server",
    //     "stream-on-checker",
    // ]
    // .into_iter()
    // .map(|server_dir| {
    //     step(
    //         format!("build {server_dir}"),
    //         [
    //             &format!("cd servers/{server_dir}"),
    //             r#"RUSTFLAGS="-C strip=none" cargo lambda build --release --output-format zip"#,
    //         ],
    //         [rust, git.servers[server_dir]],
    //     )
    // });

    // step(
    //     "npm install in cdk",
    //     ["cd cdk", "npm ci"],
    //     [node, git.cdk.package_lock_json],
    // )
    // .chain([
    //     step(
    //         "bootstrap cdk",
    //         ["cd cdk", "npx cdk bootstrap --force"],
    //         [git.cdk],
    //     ),
    //     step(
    //         "deploy cdk",
    //         [
    //             "cd cdk",
    //             "npx cdk deploy \
    //             --ci true \
    //             --require-approval never \
    //             --no-previous-parameters",
    //         ],
    //         servers.chain(std::iter::once(git.cdk)),
    //     ),
    // ]);
}

struct ApkPackageListLayer {}

impl ApkPackageListLayer {
    fn update(cache_key: &str) -> Self {
        // apk update

        Self {}
    }
    fn add(&self, package: &str) {
        step(
            format!("apk add {package}"),
            [&format!("apk add {}", package), "asdf"],
            [package],
        );
    }
}

fn install_node(apk: &ApkPackageListLayer) {
    apk.add("nodejs");
    apk.add("npm");
}

fn install_python3(apk: &ApkPackageListLayer) {
    apk.add("python3");
}

fn install_rustup(apk: &ApkPackageListLayer, version: &str) -> String {
    let cache_key = format!("rust-{}", version);
    apk.add("curl");
    step(
        "install_rustup",
        [
            "curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $version",
            r#"source "$HOME/.cargo/env"#,
        ],
        [&cache_key],
    );

    cache_key
}

fn step<Command: AsRef<str>, CacheKey: AsRef<str>>(
    title: impl AsRef<str>,
    commands: impl IntoIterator<Item = Command>,
    cache_keys: impl IntoIterator<Item = CacheKey>,
) -> String {
    todo!()
    /*
    TODO
    1. try restore layer from cache
    2. if cache miss, run commands
    3. extract OCI layer, save it.
    */
}
