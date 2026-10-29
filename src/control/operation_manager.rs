use std::fmt::format;

use crate::math_kernel::MathKernel;
use crate::control::resetter::{Operation, OperationRecord, Resetter};
use crate::math_kernel::GroupAction;


/// 胶水层：操作管理器

pub struct OperationManager {
    pub kernel: MathKernel,
    pub resetter: Resetter,
}

impl OperationManager {

    /// 创建新的操作管理器
    pub fn new(kernel: MathKernel, resetter: Resetter) -> Self {
        Self { kernel, resetter }
    }

    /// 反向应用操作记录
    pub fn reverse_apply(&mut self, record: &OperationRecord) {
        // 反向应用操作
        match record.operation_type {
            Operation::AddAllDefaultCube1 => {
                self.kernel.remove_all();
            }
            Operation::Reset => {
                // 重置操作无法撤销，可能需要特殊处理
                println!("无法撤销重置操作");
            }
            Operation::TranslateX { dx } => {
                self.kernel.transformation_translation_x(-dx);
            }
            Operation::TranslateY { dy } => {
                self.kernel.transformation_translation_y(-dy);
            }
            Operation::TranslateZ { dz } => {
                self.kernel.transformation_translation_z(-dz);
            }
            Operation::Translate { dx, dy, dz } => {
                self.kernel.transformation_translation_x(-dx);
                self.kernel.transformation_translation_y(-dy);
                self.kernel.transformation_translation_z(-dz);
            }
            Operation::TranslateAbsolute { x, y, z } => {
                // 绝对移动的撤销需要记录之前的位置，这里暂时不处理
                println!("无法撤销绝对移动操作");
            }
            Operation::RotateX { thetax } => {
                self.kernel.transformation_rotate_x(-thetax);
            }
            Operation::RotateY { thetay } => {
                self.kernel.transformation_rotate_y(-thetay);
            }
            Operation::RotateZ { thetaz } => {
                self.kernel.transformation_rotate_z(-thetaz);
            }
            Operation::Scale { ratio } => {
                if ratio != 0.0 {
                    self.kernel.transformation_scale_global(1.0 / ratio);
                }
            }
            Operation::ScaleX { ratiox } => {
                if ratiox != 0.0 {
                    self.kernel.transformation_scale_x(1.0 / ratiox);
                }
            }
            Operation::ScaleY { ratioy } => {
                if ratioy != 0.0 {
                    self.kernel.transformation_scale_y(1.0 / ratioy);
                }
            }
            Operation::ScaleZ { ratioz } => {
                if ratioz != 0.0 {
                    self.kernel.transformation_scale_z(1.0 / ratioz);
                }
            }
            _ => {
                // 对于其他操作类型，暂时不执行任何操作
            }
        }
    }

    /// 执行操作
    pub fn execute_internal(&mut self, record: &OperationRecord) {
        // 执行操作
        match &record.operation_type {
            Operation::AddAllDefaultCube1 => {
                self.kernel.load_default_cube1();
            }
            Operation::Reset => {
                self.kernel.reset();
            }
            Operation::TranslateX { dx } => {
                self.kernel.transformation_translation_x(*dx);
            }
            Operation::TranslateY { dy } => {
                self.kernel.transformation_translation_y(*dy);
            }
            Operation::TranslateZ { dz } => {
                self.kernel.transformation_translation_z(*dz);
            }
            Operation::Translate { dx, dy, dz } => {
                self.kernel.transformation_translation_x(*dx);
                self.kernel.transformation_translation_y(*dy);
                self.kernel.transformation_translation_z(*dz);
            }
            Operation::TranslateAbsolute { x, y, z } => {
                self.kernel.transformation_moveto_global(*x, *y, *z);
            }
            Operation::RotateX { thetax } => {
                self.kernel.transformation_rotate_x(*thetax);
            }
            Operation::RotateY { thetay } => {
                self.kernel.transformation_rotate_y(*thetay);
            }
            Operation::RotateZ { thetaz } => {
                self.kernel.transformation_rotate_z(*thetaz);
            }
            Operation::Scale { ratio } => {
                self.kernel.transformation_scale_global(*ratio);
            }
            Operation::ScaleX { ratiox } => {
                self.kernel.transformation_scale_x(*ratiox);
            }
            Operation::ScaleY { ratioy } => {
                self.kernel.transformation_scale_y(*ratioy);
            }
            Operation::ScaleZ { ratioz } => {
                self.kernel.transformation_scale_z(*ratioz);
            }
            Operation::AddVertex { position } => {
                self.kernel.add_vertex(*position);
            }
            Operation::AddFace { vertices } => {
                self.kernel.add_face(vertices.clone());
            }
            Operation::AddEdge { vertex1, vertex2 } => {
                self.kernel.add_edge(*vertex1, *vertex2);
            }
            Operation::AddAll { vertices, faces, edges } => {
                self.kernel.add_all(vertices.clone(), faces.clone(), edges.clone());
            }
            Operation::RemoveVertex { index } => {
                self.kernel.remove_vertex(*index);
            }
            Operation::RemoveFace { index } => {
                self.kernel.remove_face(*index);
            }
            Operation::RemoveEdge { index } => {
                self.kernel.remove_edge(*index);
            }
            Operation::RemoveAll  => {
                self.kernel.remove_all()
            }
            _ => {
                // 对于其他操作类型，暂时不执行任何操作
            }
        }

        // 同步渲染面
        self.kernel.sync_render_faces();

        // 同步脏标记
        self.kernel.sync_dirty();

    }

    /// 执行操作并记录
    pub fn execute(&mut self, record: &OperationRecord) {
        
        // 执行操作
        match &record.operation_type {
            Operation::AddAllDefaultCube1 => {
                self.kernel.load_default_cube1();
            }
            Operation::Reset => {
                self.kernel.reset();
            }
            Operation::TranslateX { dx } => {
                self.kernel.transformation_translation_x(*dx);
            }
            Operation::TranslateY { dy } => {
                self.kernel.transformation_translation_y(*dy);
            }
            Operation::TranslateZ { dz } => {
                self.kernel.transformation_translation_z(*dz);
            }
            Operation::Translate { dx, dy, dz } => {
                self.kernel.transformation_translation_x(*dx);
                self.kernel.transformation_translation_y(*dy);
                self.kernel.transformation_translation_z(*dz);
            }
            Operation::TranslateAbsolute { x, y, z } => {
                self.kernel.transformation_moveto_global(*x, *y, *z);
            }
            Operation::RotateX { thetax } => {
                self.kernel.transformation_rotate_x(*thetax);
            }
            Operation::RotateY { thetay } => {
                self.kernel.transformation_rotate_y(*thetay);
            }
            Operation::RotateZ { thetaz } => {
                self.kernel.transformation_rotate_z(*thetaz);
            }
            Operation::Scale { ratio } => {
                self.kernel.transformation_scale_global(*ratio);
            }
            Operation::ScaleX { ratiox } => {
                self.kernel.transformation_scale_x(*ratiox);
            }
            Operation::ScaleY { ratioy } => {
                self.kernel.transformation_scale_y(*ratioy);
            }
            Operation::ScaleZ { ratioz } => {
                self.kernel.transformation_scale_z(*ratioz);
            }
            Operation::AddVertex { position } => {
                self.kernel.add_vertex(*position);
            }
            Operation::AddFace { vertices } => {
                self.kernel.add_face(vertices.clone());
            }
            Operation::AddEdge { vertex1, vertex2 } => {
                self.kernel.add_edge(*vertex1, *vertex2);
            }
            Operation::AddAll { vertices, faces, edges } => {
                self.kernel.add_all(vertices.clone(), faces.clone(), edges.clone());
            }
            Operation::RemoveVertex { index } => {
                self.kernel.remove_vertex(*index);
            }
            Operation::RemoveFace { index } => {
                self.kernel.remove_face(*index);
            }
            Operation::RemoveEdge { index } => {
                self.kernel.remove_edge(*index);
            }
            Operation::RemoveAll  => {
                self.kernel.remove_all()
            }
            Operation::SetName { name } => {
                self.kernel.attributes.set_name(&name);
            }
            Operation::SetColor { hex } => {
                self.kernel.attributes.set_color_display(hex.clone());
            }
            Operation::AddGroupAction { action } => {
                self.kernel.attributes.add_group_action(action.clone());
            }
            Operation::Batch { operations } => {
                for op in operations {
                    let record = OperationRecord {
                        operation_type: op.clone(),
                        operation_data: None,
                        operation_description: "批量操作此步骤".to_string(),
                        timestamp: record.timestamp,
                    };
                    self.execute_internal(&record);

                    self.kernel.sync_render_faces();
                    self.kernel.sync_dirty();
                }
            }
            Operation::Custom { name, data } => {
                println!("执行自定义操作: {}, 数据: {}", name, data);
                // 在这里可以根据自定义操作的名称和数据来执行相应的逻辑
            }
        }

        // 同步渲染面
        self.kernel.sync_render_faces();

        // 同步脏标记
        self.kernel.sync_dirty();

        // 记录到重置器
        self.resetter.add_record(record.clone());
    }


    // 便捷操作

    ///  重置 
    pub fn reset(&mut self) {
        self.execute(&OperationRecord {
            operation_type: Operation::Reset,
            timestamp: std::time::SystemTime::now(),
            operation_description: "重置".to_string(),
            operation_data: None,
        });
    }

    /// 加载默认立方体 
    pub fn load_default_cube(&mut self) {
        self.kernel.load_default_cube1();
        self.kernel.sync_render_faces();
        self.resetter.add_record(OperationRecord {
            operation_type: Operation::AddAllDefaultCube1,
            timestamp: std::time::SystemTime::now(),
            operation_description: "加载默认立方体".to_string(),
            operation_data: None,
        });
    }

    /// 平移 
    pub fn translate_x(&mut self, dx: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::TranslateX { dx },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("平移 X {:.2}", dx),
            operation_data: None,
        });
    }

    pub fn translate_y(&mut self, dy: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::TranslateY { dy },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("平移 Y {:.2}", dy),
            operation_data: None,
        });
    }

    pub fn translate_z(&mut self, dz: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::TranslateZ { dz },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("平移 Z {:.2}", dz),
            operation_data: None,
        });
    }

    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::Translate { dx, dy, dz },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("平移 ({:.2}, {:.2}, {:.2})", dx, dy, dz),
            operation_data: None,
        });
    }

    pub fn translate_absolute(&mut self, x: f64, y: f64, z: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::TranslateAbsolute { x, y, z },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("绝对平移 ({:.2}, {:.2}, {:.2})", x, y, z),
            operation_data: None,
        });
    }

    /// 旋转 
    pub fn rotate_x(&mut self, thetax: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::RotateX { thetax },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("绕 X 轴旋转 {:.2}°", thetax.to_degrees()),
            operation_data: None,
        });
    }

    pub fn rotate_y(&mut self, thetay: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::RotateY { thetay },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("绕 Y 轴旋转 {:.2}°", thetay.to_degrees()),
            operation_data: None,
        });
    }

    pub fn rotate_z(&mut self, thetaz: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::RotateZ { thetaz },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("绕 Z 轴旋转 {:.2}°", thetaz.to_degrees()),
            operation_data: None,
        });
    }

    /// 缩放 
    pub fn scale(&mut self, ratio: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::Scale { ratio },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("缩放 {:.2}x", ratio),
            operation_data: None,
        });
    }

    pub fn scale_x(&mut self, ratiox: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::ScaleX { ratiox },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("缩放 X {:.2}x", ratiox),
            operation_data: None,
        });
    }

    pub fn scale_y(&mut self, ratioy: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::ScaleY { ratioy },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("缩放 Y {:.2}x", ratioy),
            operation_data: None,
        });
    }

    pub fn scale_z(&mut self, ratioz: f64) {
        self.execute(&OperationRecord {
            operation_type: Operation::ScaleZ { ratioz },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("缩放 Z {:.2}x", ratioz),
            operation_data: None,
        });
    }

    /// 添加
    pub fn add_vertex(&mut self, position:[f64; 3] ) {
        self.execute(&OperationRecord {
            operation_type: Operation::AddVertex { position },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("添加顶点 ({:.2}, {:.2}, {:.2})", position[0], position[1], position[2]),
            operation_data: None,
        });
    }

    pub fn add_face(&mut self, vertices: Vec<usize>) {
        self.execute(&OperationRecord {
            operation_type: Operation::AddFace { vertices: vertices.clone() },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("添加面 {:?}", vertices),
            operation_data: None,
        });
    }

    pub fn add_edge(&mut self, v1: usize, v2: usize) {
        self.execute(&OperationRecord {
            operation_type: Operation::AddEdge { vertex1: v1, vertex2: v2 },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("添加边 ({}, {})", v1, v2),
            operation_data: None,
        });
    }

    pub fn add_all(&mut self, vertices: Vec<[f64;3]>, faces: Vec<Vec<usize>>, edges: Vec<[usize; 2]>) {
        self.execute(&OperationRecord {
            operation_type: Operation::AddAll {
                vertices: vertices.clone(),
                faces: faces.clone(),
                edges: edges.clone(),
            },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("添加全部 ({}顶点, {}面, {}边)", vertices.len(), faces.len(), edges.len()),
            operation_data: None,
        });
    }

    /// 移除 
    pub fn remove_vertex(&mut self, index: usize) {
        self.execute(&OperationRecord {
            operation_type: Operation::RemoveVertex { index },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("移除顶点 {}", index),
            operation_data: None,
        });
    }

    pub fn remove_face(&mut self, index: usize) {
        self.execute(&OperationRecord {
            operation_type: Operation::RemoveFace { index },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("移除面 {}", index),
            operation_data: None,
        });
    }

    pub fn remove_edge(&mut self, index: usize) {
        self.execute(&OperationRecord {
            operation_type: Operation::RemoveEdge { index },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("移除边 {}", index),
            operation_data: None,
        });
    }

    pub fn remove_all(&mut self) {
        self.execute(&OperationRecord {
            operation_type: Operation::RemoveAll,
            timestamp: std::time::SystemTime::now(),
            operation_description: "移除全部".to_string(),
            operation_data: None,
        });
    }

    /// 设置名称
    pub fn set_name(&mut self, name: String) {
        self.execute(&OperationRecord {
            operation_type: Operation::SetName { name: name.clone() },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("设置名称: {}", name),
            operation_data: None,
        });
    }

    /// 设置颜色
    pub fn set_color(&mut self, hex: String) {
        self.execute(&OperationRecord {
            operation_type: Operation::SetColor { hex: hex.clone() },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("设置颜色: {}", hex),
            operation_data: None,
        });
    }

    /// 添加群操作
    pub fn add_group_action(&mut self, action: GroupAction) {
        self.execute(&OperationRecord {
            operation_type: Operation::AddGroupAction { action: action.clone() },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("添加群操作: {}", action.name),
            operation_data: None,
        });
    }

    /// 批量 
    pub fn batch(&mut self, operations: Vec<Operation>) {
        let record = OperationRecord { 
            operation_type: Operation::Batch { operations: operations.clone() }, 
            timestamp: std::time::SystemTime::now(), 
            operation_description: format!("批量操作 {} 个子操作",operations.len()), 
            operation_data: None,
        };
        self.execute(&record)
    }

    /// 自定义 
    pub fn custom(&mut self, name: String, data: String) {
        self.execute(&OperationRecord {
            operation_type: Operation::Custom { name: name.clone(), data: data.clone() },
            timestamp: std::time::SystemTime::now(),
            operation_description: format!("自定义操作: {}", name),
            operation_data: None,
        });
    }

    /// 撤销操作
    pub fn undo(&mut self) {
        match self.resetter.undo() {
            Ok(record) => {
                self.reverse_apply(&record);
            }
            Err(err) => {
                println!("撤销未响应: {}", err);
            }
        }
    }

    /// 重做操作
    pub fn redo(&mut self) {
        match self.resetter.redo() {
            Ok(record) => {
                self.execute(&record);
            }
            Err(err) => {
                println!("重做未响应: {}", err);
            }
        }
    }

}