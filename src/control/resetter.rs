use crate::types::VecN;
use crate::math_kernel::GroupAction;

/// 重置器：记录操作信息

/// 最大步数常量（一期默认50步）
pub const DEFAULT_MAX_STEPS: usize = 50;

#[derive(Debug)]
/// 重置器内涵
pub struct Resetter {
    pub history: Vec<OperationRecord>,        //历史记录
    pub current_index: usize,                 //当前所在位置
    pub max_step: usize,                      //最大步数（一期默认50步）
    pub save_already: bool,                   //是否已保存历史记录
    pub safe_lock: bool,                      //安全锁（防止误操作）
}


#[derive(Debug, Clone)]
/// 重置器操作记录（单次操作）
pub struct OperationRecord {
    pub operation_type: Operation,                    //操作类型
    pub timestamp: std::time::SystemTime,             //时间戳
    pub operation_description: String,                //操作描述
    pub operation_data: Option<String>,               //操作数据
}


#[derive(Debug, Clone)]
/// 操作类型枚举
pub enum Operation {
    Reset,

    // 变换
    Translate { dx: f64, dy: f64, dz: f64 },
    TranslateX { dx: f64 },
    TranslateY { dy: f64 },
    TranslateZ { dz: f64 },
    TranslateAbsolute { x: f64 , y: f64, z: f64 },
    RotateX { thetax: f64 },
    RotateY { thetay: f64 },
    RotateZ { thetaz: f64 },
    Scale { ratio: f64 },
    ScaleX { ratiox: f64 },
    ScaleY { ratioy: f64 },
    ScaleZ { ratioz: f64 },

    // 几何操作
    AddVertex { position: VecN<3> },
    AddFace { vertices: Vec<usize> },
    AddEdge { vertex1: usize, vertex2: usize },
    AddAll { vertices: Vec<VecN<3>>, faces: Vec<Vec<usize>> , edges: Vec<[usize; 2]> },
    AddAllDefaultCube1, 
    RemoveVertex { index: usize },
    RemoveFace { index: usize },
    RemoveEdge { index: usize },
    RemoveAll ,

    // 属性操作
    SetName { name: String },
    SetColor { hex: String },
    AddGroupAction { action: GroupAction },

    // 批量/自定义
    Batch { operations: Vec<Operation> },
    Custom { name: String, data: String },
}

/// 重置器方法
impl Resetter {

    /// 创建新的重置器
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: 0,
            max_step: DEFAULT_MAX_STEPS,
            save_already: true,
            safe_lock: false,
        }
    }

    /// 保存操作记录（单次操作）
    pub fn add_record(&mut self, operation: OperationRecord) {
        
        // 安全锁
        if self.safe_lock {
            println!("进程繁忙！请稍后再试 ^~^!");
            return;
        }
        
        // 保留整个历史列表；当前位置只表示当前指针，不会把后面的记录直接删掉
        self.history.push(operation);
        self.current_index = self.history.len();
        
        // 如果历史记录超过最大步数，则移除最旧的记录
        if self.history.len() > self.max_step {
                self.history.remove(0);             // 超过最大步数，移除最旧的记录
                self.current_index -= 1;
        }

        // 标记为未保存状态
        self.save_already = false;
    }


    /// 撤销一步，返回被撤销的操作记录
    pub fn undo(&mut self) -> Result<OperationRecord, &'static str> {
        if self.safe_lock {
            return Err("进程繁忙！请稍后再试 ^~^!");
        }
        if self.current_index == 0 {
            return Err("记忆不够用了 ^~^!");
        }

        self.current_index -= 1;
        self.save_already = false; // 状态变了，标记为未保存
        Ok(self.history[self.current_index].clone())
    }


    /// 重做一步，返回被重做的操作记录
    pub fn redo(&mut self) -> Result<OperationRecord, &'static str> {
        if self.safe_lock {
            return Err("进程繁忙！请稍后再试 ^~^!");
        }
        if self.current_index >= self.history.len() {
            return Err("记忆不够用了 ^~^!");
        }

        let record = self.history[self.current_index].clone();
        self.current_index += 1;
        self.save_already = false; // 状态变了，标记为未保存
        Ok(record)
    }


    /// 跳转到历史中的某一步（索引从 0 开始）
    pub fn jump_to(&mut self, index: usize) -> Result<(), &'static str> {
        if self.safe_lock {
            return Err("进程繁忙！请稍后再试 ^~^!");
        }
        if index >= self.history.len() {
            return Err("找不到记忆了 ^~^!");
        }

        // 注意：UI 里索引0对应第一步，但 current_index 指向的是“下一步要写入的位置”
        // 跳转到第 index 步，即current_index = index + 1
        self.current_index = index + 1;
        self.save_already = false;
        Ok(())
    }


    // 状态查询

    /// 是否可以撤销
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    /// 是否可以重做
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len()
    }


    // 数据获取（供 UI 列表显示）

    /// 获取整个历史列表的只读引用
    pub fn get_history(&self) -> &Vec<OperationRecord> {
        &self.history
    }

    /// 获取当前状态的描述（比如显示在状态栏）
    pub fn get_current_description(&self) -> String {
        if self.current_index == 0 {
            "初始记忆".to_string()
        } else {
            format!("当前记忆: {}", self.history[self.current_index - 1].operation_description)
        }
    }


    // 重置与保存确认

    /// 清空所有历史记录（不可逆）
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.current_index = 0;
        self.save_already = true; // 清空后算是已保存状态
    }

    /// 用户点击保存后调用，标记为已保存
    pub fn confirm_save(&mut self) {
        self.save_already = true;
    }

}