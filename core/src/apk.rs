use crate::{step, Step};

pub struct ApkPackageListLayer {
    update_step: Step,
}

impl ApkPackageListLayer {
    pub fn update(cache_key: impl AsRef<str>) -> Self {
        let update_step = step(
            format!("apk update"),
            [&format!("apk update")],
            [format!("apk-{cache_key}", cache_key = cache_key.as_ref())],
        );

        Self { update_step }
    }
    pub fn add(&self, package: &str) {
        step(
            format!("apk add {package}"),
            [&format!("apk add {}", package)],
            [&self.update_step],
        );
    }
}
