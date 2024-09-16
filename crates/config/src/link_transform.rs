use std::env::current_dir;

use crate::Links;

pub fn link_transform(links: Links) -> Links {
    let current_dir = current_dir().unwrap();
    let links = links
        .iter()
        .map(|(source, target)| {
            (
                current_dir.join(source).to_str().unwrap().to_string(),
                // TODO Fine, we need to use some placeholders like `~` to
                // TODO represent the home directory...
                // The target should be the absolute path
                target.to_string(),
            )
        })
        .collect();

    links
}
