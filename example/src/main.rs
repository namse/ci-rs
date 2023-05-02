use ci_rs_core::apk::ApkPackageListLayer;

fn main() {
    // let image = "alpine:3";
    // let arch = "arm64";

    let apk = ApkPackageListLayer::update("2023-05-02");

    apk.add("nodejs");
    apk.add("npm");
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
