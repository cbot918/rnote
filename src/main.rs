use fltk::{app, frame::Frame, group::Flex, prelude::*, window::Window, tree::Tree};

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    // file io
    let path = Path::new("."); // current directory
    visit_dirs(path, 0).unwrap();

    let app = app::App::default();
    let mut win = Window::default().with_size(800, 600).with_label("rnote");
    let mut flex = Flex::default().size_of_parent().row(); // 水平排列
    let mut tree = Tree::default().with_size(390, 290).center_of_parent();
    tree.add("Item 1");
    tree.add("Item 2");
    tree.add("Item 3");
    flex.set_size(&tree, 200); // 設定為總寬度的 1/4 (假設 800px)

    let mut right = Frame::default().with_label("Right");
    right.set_color(fltk::enums::Color::Green);

    flex.end();

    // Tree
    win.end();
    win.show();

    app.run().unwrap();
}


fn visit_dirs(dir: &Path, depth: usize) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            for _ in 0..depth {
                print!("  ");
            }

            println!("{}", path.display());

            if path.is_dir() {
                visit_dirs(&path, depth + 1)?;
            }
        }
    }
    Ok(())
}