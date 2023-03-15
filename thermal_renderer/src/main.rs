use thermal_parser::{context::Context, command::Command};
use thermal_renderer::{image_renderer::ImageRenderer, renderer::CommandRenderer};

fn main(){


    let bytes = std::fs::read("ori.bin").unwrap();
    let vec_bytes:Vec<u8> = bytes.to_vec();
    let mut context = Context::new();
    let mut image_renderer = ImageRenderer::new(format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "resources", "out"));
    println!("{:?}",image_renderer.out_path);
    let on_new_command = move |cmd: Command| {
        image_renderer.process_command(&mut context, &cmd);
    };

    let mut command_parser = thermal_parser::new_esc_pos_parser(Box::from(on_new_command));
    command_parser.parse_bytes(&vec_bytes);

}


