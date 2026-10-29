use egui::Key::K;
use wgpu::naga::back::spv::Capability::Kernel;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use std::env;
use std::sync::Arc;

mod render_ui_manual_integration;
mod math_kernel;
mod types;
mod control;
mod global_registry;
mod abstract_math_calculator;
mod algorithm_editor;
mod error_logger;

use std::io::{self, Write};

use control::{resetter, operation_manager};

use abstract_math_calculator::AbstractMathCalculatorAddGroup;
use crate::abstract_math_calculator::AbstractMathCalculatorGroup;
use crate::control::{OperationManager, Resetter};

use math_kernel::MathKernel;
use crate::math_kernel::AttributesDictionary;

use crate::algorithm_editor::AlgorithmEditorOutput;

use render_ui_manual_integration::Renderer;

use pollster::block_on;

use winit::event::{KeyEvent, ElementState};

use winit::keyboard::{KeyCode, PhysicalKey};

use error_logger::log_error;
fn main() /* -> io::Result<()> */  {
// ClU
/* 
    /// 初始化
    let kernel = MathKernel::new();
    let resetter = Resetter::new();
    let mut manager = OperationManager::new(kernel, resetter);
    
    
    manager.kernel.load_default_cube1();
    manager.kernel.sync_render_faces();

    println!("╔═══════════════════════════════════════╗");
    println!("║   silcL 引擎 v0.1.0                   ║");
    println!("║   输入 help 查看命令                  ║");
    println!("╚═══════════════════════════════════════╝");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0];

        match cmd {
            "exit" => break,
            "help" => print_help(),
            "translate" => handle_translate(&mut manager, &parts),
            "rotate" => handle_rotate(&mut manager, &parts),
            "scale" => handle_scale(&mut manager, &parts),
            "undo" => handle_undo(&mut manager),
            "redo" => handle_redo(&mut manager),
            "info" => handle_info(&mut manager),
            "export" => handle_export(&mut manager),
            _ => println!("❌ 未知命令: {}", cmd),
        }
    }

    println!("👋 引擎关闭了");
    Ok(())
*/ 

    // run_cli();
    env_logger::init();

    std::panic::set_hook(Box::new(|panic_info| {
        let msg = format!("Panic: {}", panic_info);
        log_error(&msg);
        eprintln!("程序崩溃，错误已记录到 error.log");
    }));

    // ---- 1. 初始化内核 ----
    let mut kernel = MathKernel::new();
    kernel.load_default_cube1();
    kernel.sync_render_faces();

    let resetter = Resetter::new();
    let mut manager = OperationManager::new(kernel, resetter);

    // ---- 2. 窗口 ----
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("SilcL 引擎")
            .with_inner_size(winit::dpi::PhysicalSize::new(1024, 768))
            .build(&event_loop)
            .unwrap(),
    );

    let mut renderer = block_on(Renderer::new(window.clone(), &manager.kernel));

    // ---- 3. 状态变量 ----
    let mut mode = String::new();
    let mut axis = String::new();
    let mut waiting_for_axis = false;
    let mut waiting_for_value = false;
    let mut input_buffer = String::new();

    println!("🎮 SilcL 引擎键盘控制");
    println!("  G → 平移 | R → 旋转 | S → 缩放");
    println!("  X/Y/Z → 选择轴 | Enter → 执行 | Esc → 取消");

    // ---- 4. 事件循环（合并版） ----
    event_loop.run(move |event, target| {
        // ---- egui 事件 ----
        if let Event::WindowEvent { event, .. } = &event {
            let _ = renderer.egui_state.on_window_event(&window, event);
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                // ===== 键盘事件 =====
                WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        physical_key: PhysicalKey::Code(keycode),
                        state,
                        ..
                    },
                    ..
                } => {
                    if state == ElementState::Pressed {
                        match keycode {
                            KeyCode::KeyG if !waiting_for_value => {
                                mode = "translate".to_string();
                                waiting_for_axis = true;
                                input_buffer.clear();
                                println!("平移模式，选择轴: X/Y/Z");
                            }
                            KeyCode::KeyR if !waiting_for_value => {
                                mode = "rotate".to_string();
                                waiting_for_axis = true;
                                input_buffer.clear();
                                println!("旋转模式，选择轴: X/Y/Z");
                            }
                            KeyCode::KeyS if !waiting_for_value => {
                                mode = "scale".to_string();
                                waiting_for_axis = true;
                                input_buffer.clear();
                                println!("缩放模式，选择轴: X/Y/Z/A");
                            }
                            KeyCode::KeyX if waiting_for_axis => {
                                axis = "x".to_string();
                                waiting_for_axis = false;
                                waiting_for_value = true;
                                input_buffer.clear();
                                println!("输入 {} 方向数值:", axis);
                            }
                            KeyCode::KeyY if waiting_for_axis => {
                                axis = "y".to_string();
                                waiting_for_axis = false;
                                waiting_for_value = true;
                                input_buffer.clear();
                                println!("输入 {} 方向数值:", axis);
                            }
                            KeyCode::KeyZ if waiting_for_axis => {
                                axis = "z".to_string();
                                waiting_for_axis = false;
                                waiting_for_value = true;
                                input_buffer.clear();
                                println!("输入 {} 方向数值:", axis);
                            }
                            KeyCode::KeyA if waiting_for_axis => {
                                axis = "a".to_string();
                                waiting_for_axis = false;
                                waiting_for_value = true;
                                input_buffer.clear();
                                println!("输入总分量数值:");
                            }
                            KeyCode::Enter if waiting_for_value => {
                                if !input_buffer.is_empty() {
                                    let val = input_buffer.parse::<f64>().unwrap_or(0.0);
                                    match mode.as_str() {
                                        "translate" => {
                                            match axis.as_str() {
                                                "x" => manager.translate(val, 0.0, 0.0),
                                                "y" => manager.translate(0.0, val, 0.0),
                                                "z" => manager.translate(0.0, 0.0, val),
                                                _ => {}
                                            }
                                        }
                                        "rotate" => {
                                            let angle = val.to_radians();
                                            match axis.as_str() {
                                                "x" => manager.rotate_x(angle),
                                                "y" => manager.rotate_y(angle),
                                                "z" => manager.rotate_z(angle),
                                                _ => {}
                                            }
                                        }
                                        "scale" => {
                                            match axis.as_str() {
                                                "x" => manager.scale_x(val),
                                                "y" => manager.scale_y(val),
                                                "z" => manager.scale_z(val),
                                                "a" => manager.scale(val),
                                                _ => {}        
                                            }
                                        }
                                        _ => {}
                                    }
                                    renderer.update_buffers(&manager.kernel);
                                    println!("✅ 操作完成");
                                    mode.clear();
                                    axis.clear();
                                    waiting_for_axis = false;
                                    waiting_for_value = false;
                                    input_buffer.clear();
                                }
                            }
                            KeyCode::Escape if waiting_for_value || waiting_for_axis => {
                                mode.clear();
                                axis.clear();
                                waiting_for_axis = false;
                                waiting_for_value = false;
                                input_buffer.clear();
                                println!("已取消");
                            }
                            KeyCode::KeyQ => {
                                manager.reset();   // 重置所有变换
                                manager.kernel.load_default_cube1();
                                renderer.update_buffers(&manager.kernel);
                                println!("🔄 已重置");
                            }
                            KeyCode::Digit0 => input_buffer.push('0'),
                            KeyCode::Digit1 => input_buffer.push('1'),
                            KeyCode::Digit2 => input_buffer.push('2'),
                            KeyCode::Digit3 => input_buffer.push('3'),
                            KeyCode::Digit4 => input_buffer.push('4'),
                            KeyCode::Digit5 => input_buffer.push('5'),
                            KeyCode::Digit6 => input_buffer.push('6'),
                            KeyCode::Digit7 => input_buffer.push('7'),
                            KeyCode::Digit8 => input_buffer.push('8'),
                            KeyCode::Digit9 => input_buffer.push('9'),
                            KeyCode::Period => input_buffer.push('.'),
                            KeyCode::Minus => input_buffer.push('-'),
                            _ => {}
                        }
                    }
                }
                WindowEvent::Resized(size) => renderer.resize(size),
                WindowEvent::CloseRequested => target.exit(),
                _ => {}
            },

            Event::AboutToWait => {
                if let Err(e) = renderer.render(&mut manager) {
                    match renderer.render(&mut manager) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            let _ = log_error("Surface Lost，正在重置");
                            renderer.resize(window.inner_size());
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            let _ = log_error("GPU 内存耗尽");
                            target.exit();
                        }
                        Err(e) => {
                            let _ = log_error(&format!("渲染失败: {}", e));
                        }
                    }
                }
            }
            _ => {}
        }
    }).unwrap();

/*    
    let kernel_display =&manager.kernel;
    println!("{:?}",kernel_display);
    let dictionary = AttributesDictionary::new();
    println!("{:?}",dictionary);
    let calculator = AbstractMathCalculatorAddGroup::<3>;
    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0, 6.0];
    let result = calculator.op(&a, &b);
    println!("Result: {:?}", result);

    // 导出tex文件PDF（指定路径为output/algebra_groups.pdf） 
*/

}



fn print_help() {
    println!("\n可用命令:");
    println!("  translate <dx> <dy> <dz>  - 平移");
    println!("  rotate <x|y|z> <角度>     - 绕轴旋转");
    println!("  scale <因子>             - 均匀缩放");
    println!("  undo                     - 撤销");
    println!("  redo                     - 重做");
    println!("  info                     - 显示当前状态");
    println!("  export                   - 导出代数结构为 LaTeX");
    println!("  exit                     - 退出\n");
}

fn handle_translate(manager: &mut OperationManager, parts: &[&str]) {
    if parts.len() != 4 {
        println!("❌ 用法: translate <dx> <dy> <dz>");
        return;
    }
    let dx: f64 = parts[1].parse().unwrap_or(0.0);
    let dy: f64 = parts[2].parse().unwrap_or(0.0);
    let dz: f64 = parts[3].parse().unwrap_or(0.0);
    manager.translate(dx, dy, dz);
    println!("✅ 已平移 ({:.2}, {:.2}, {:.2})", dx, dy, dz);
}

fn handle_rotate(manager: &mut OperationManager, parts: &[&str]) {
    if parts.len() != 3 {
        println!("❌ 用法: rotate <x|y|z> <角度>");
        return;
    }
    let axis = parts[1];
    let angle: f64 = parts[2].parse().unwrap_or(0.0);
    let angle_rad = angle.to_radians();

    match axis {
        "x" => manager.rotate_x(angle_rad),
        "y" => manager.rotate_y(angle_rad),
        "z" => manager.rotate_z(angle_rad),
        _ => {
            println!("❌ 未知轴: {}", axis);
            return;
        }
    }
    println!("✅ 已绕 {} 轴旋转 {:.2}°", axis, angle);
}

fn handle_scale(manager: &mut OperationManager, parts: &[&str]) {
    if parts.len() != 2 {
        println!("❌ 用法: scale <因子>");
        return;
    }
    let factor: f64 = parts[1].parse().unwrap_or(1.0);
    manager.scale(factor);
    println!("✅ 已缩放 {:.2}x", factor);
}

fn handle_undo(manager: &mut OperationManager) {
    manager.undo();
}

fn handle_redo(manager: &mut OperationManager) {
    manager.redo();
}

fn handle_info(manager: &mut OperationManager) {
    let v_count = manager.kernel.vertex_list.len();
    let f_count = manager.kernel.face_list.len();
    let e_count = manager.kernel.edge_list.len();
    let r_count = manager.kernel.render_faces_list.len();
    println!("📊 当前状态:");
    println!("   顶点数: {}", v_count);
    println!("   数学面数: {}", f_count);
    println!("   边数: {}", e_count);
    println!("   渲染面数: {}", r_count);
    println!("   脏标记: {}", manager.kernel.is_dirty);
}

fn handle_export(manager: &mut OperationManager) {
    let editor = AlgorithmEditorOutput::new();
    let latex = editor.generate_full_tex();
    match editor.export_tex(&latex, "output/algebra.tex") {
        Ok(_) => println!("✅ 已导出到 output/algebra.tex"),
        Err(e) => println!("❌ 导出失败: {}", e),
    }

}