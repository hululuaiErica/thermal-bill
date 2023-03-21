use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

lazy_static! {
    static ref SETTING: Setting = load_setting();
    pub static ref SKIP_DATA_IS_EMPTY: bool = get_skip_data_is_empty();
    pub static ref SKIP_CONTEXT_GRAPHRICS_Y_PLUS: bool = get_skip_context_graphics_y_plus();
    pub static ref SKIP_NEW_Y_PLUS_LAYOUT_LINE_HEIGHT:bool = get_skip_new_y_plus_layout_line_height();
    pub static ref SKIP_FEED:bool = get_skip_feed();
}
pub const SETTING_PATH: &str = "/etc/xbox/convert_to_bit_image.json";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub skip_data_is_empty:bool,
    pub skip_context_graphics_y_plus:bool,
    pub skip_new_y_plus_layout_line_height:bool,
    pub skip_feed:bool
}

fn load_setting() -> Setting {
    match std::fs::read(SETTING_PATH) {
        Ok(buf) => {
            match serde_json::from_slice::<Setting>(&buf[..]) {
                Ok(setting) => {
                    println!("load setting {:?}", setting);
                    return setting;
                }
                Err(err) => {
                    println!("load setting error {:?}", err);
                }
            }
        }
        Err(e) => {
            println!("read(SETTING_PATH):{}",e)
        }
        
    }
    
    Setting {
        skip_data_is_empty:false,
        skip_context_graphics_y_plus:false,
        skip_new_y_plus_layout_line_height:false,
        skip_feed:false
    }
}

#[inline]
pub fn setting_ref() -> &'static Setting {
    &SETTING
}

fn get_skip_data_is_empty()->bool {
    setting_ref().skip_data_is_empty
}
fn get_skip_context_graphics_y_plus()->bool {
    setting_ref().skip_context_graphics_y_plus
}
fn get_skip_new_y_plus_layout_line_height()->bool {
    setting_ref().skip_new_y_plus_layout_line_height
}
fn get_skip_feed()->bool {
    setting_ref().skip_feed
}