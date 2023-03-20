# 小票数据识别

## 可以识别文本与图片

## 代码改动转换成ori.bin
thermal-main/thermal_renderer/src/image_renderer 209

```
fn draw_text(){
            ...
            new_x = x;
            //new_y += layout.line_height as usize
}
            
```

thermal-main/thermal_renderer/src
```
fn process_command(&mut self, context: &mut Context, command: &Command) {
            ...
            GraphicsCommand::Image(image) => {
                            if command.data.is_empty()// && context.graphics.y >= image.height as usize
                            {
                                // println!("{}",image.height as usize);
                                // context.graphics.y -= 30;

                                return;
                            }

                            if image.advances_xy { context.graphics.x = context.graphics_x_offset(image.width) as usize; }
                            self.draw_image(context, image.as_grayscale(), image.width as usize, image.height as usize);

                            if image.advances_xy {
                                context.graphics.x = 0;
                                context.graphics.y += image.height as usize;
                                // context.graphics.y += context.line_height_pixels() as usize;
                            }
                        }
             ...
}

fn handle_device_commands(&mut self, device_commands: &Option<Vec<DeviceCommand>>, context: &mut Context){



              ...
              DeviceCommand::Feed(num) => {
                       // context.graphics.y += context.motion_unit_y_pixels() as usize * *num as usize;
                    }
              ...
}
```

