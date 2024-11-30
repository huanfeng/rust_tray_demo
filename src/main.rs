use tray_icon::{menu::{Menu, MenuEvent, MenuItem}, Icon, TrayIconBuilder, TrayIconEvent};
use winit::{
    event_loop::EventLoop,
    event::Event,
};

fn main() {
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();

    // 创建托盘菜单
    let menu: Menu = Menu::new();
    let show_handle: MenuItem = MenuItem::new("显示", true, None);
    let quit_handle: MenuItem = MenuItem::new("退出", true, None);
    
    menu.append(&show_handle).unwrap();
    menu.append(&MenuItem::new("", false, None)).unwrap(); // 分隔符
    menu.append(&quit_handle).unwrap();

    // 创建一个简单的图标 (16x16 像素，RGBA)
    let icon = include_bytes!("../assets/icon.png");
    let icon = image::load_from_memory(icon).unwrap().to_rgba8();
    let (icon_width, icon_height) = (icon.width(), icon.height());
    
    let icon: Icon = Icon::from_rgba(
        icon.into_raw(),
        icon_width,
        icon_height,
    ).unwrap();

    // 构建托盘
    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .with_tooltip("救命程序")
        .build()
        .unwrap();

    println!("托盘程序已启动！");

    // 方式1: 直接使用event_handler, 注意仍然需要有 loop 在运行
    TrayIconEvent::set_event_handler(Some(move |event| {
        println!("TrayIconEvent: {:?}", event);
    }));

    MenuEvent::set_event_handler(Some(move |event| {
        println!("MenuEvent: {:?}", event);
    }));

    // 方式2: 创建一个事件接收器来处理菜单事件, 然后在 loop 中处理
    let menu_channel = MenuEvent::receiver();
    
    // 运行事件循环
    let _ = event_loop.run(move |event: Event<()>, elwt| {

        // 这里配合方式2, 使用方式1则不需要
        if let Ok(menu_event) = menu_channel.try_recv() {
            match menu_event.id {
                id if id == show_handle.id() => {
                    println!("点击了显示选项");
                    // 这里可以添加显示窗口的逻辑
                }
                id if id == quit_handle.id() => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }

        match event {
            Event::NewEvents(_) => {}
            _ => {}
        }
        elwt.set_control_flow(winit::event_loop::ControlFlow::Wait);
    });
}
