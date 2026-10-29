use crate::{math_kernel::OperationTarget::Face, types::{Float, VecN}};

#[derive(Debug,Clone)]

///数学内核：管理存储所有几何数据与代数数据

/// 基础数学内核
pub struct MathKernel {
    pub vertex_list: Vec<VecN<3>>,           //顶点列表（三维坐标点）
    pub face_list: Vec<Vec<usize>>,            //面列表（存储顶点索引）
    pub render_faces_list: Vec<[usize; 3]>,    //面列表（存储顶点索引）
    pub edge_list: Vec<[usize; 2]>,            //边列表（存储顶点索引）
    pub is_dirty: bool,                        //数据是否被修改过
    
    pub attributes: AttributesDictionary,      //属性字典嵌套
}


/// 基础权重内核
///pub struct RatioKernel {
///    pub general_weighted_center: VecN<3>,   //通用权重中心（三维坐标点）
///}


/// X1Y0Z2

#[derive(Debug,Clone)]
/// 基础属性字典
pub struct AttributesDictionary {
    // 状态属性
    pub is_dirty: bool,                             //引用MathKernel的 is_dirty 

    // 显示属性
    pub color_display: Option<String>,              //显示颜色#FF0000格式
    pub label: Option<String>,                      //标题/标签
    pub font_size: Option<f64>,                     //字体大小

    // 用户属性
    pub name: Option<String>,                       //名称
    pub note: Option<String>,                       //备注

    // 数学属性
    pub group_action: Vec<GroupAction>,             //群运算列表
}

#[derive(Debug, Clone)]

/// 群运算列表
pub struct GroupAction {
    pub name: String,                                //群名称
    pub action_type: ActionType,                     //操作类型
    pub parameters: Vec<f64>,                        //参数
    pub action_object: OperationTarget               //操作对象

}

///
#[derive(Debug, Clone)]

pub enum OperationTarget {
    // 全部 
    AllVertices,            //所有顶点
    AllEdges,               //所有边
    AllFaces,               //所有面
    All,                    //所有（顶点+边+面）

    // 部分（索引）
    Vertex(usize),          //单个顶点
    Vertices(Vec<usize>),   //多个顶点
    Edge(usize),            //单条边
    Edges(Vec<usize>),      //多条边
    Face(usize),            //单个面
    Faces(Vec<usize>),      //多个面

    // 其他
    Selection,              //当前选中的部分
    None,
}

#[derive(Debug, Clone)]

// 群运算名称
pub enum ActionType {
    Rotation,           // 旋转
    Reflection,         // 反射
    Translation,        // 平移
    Scaling,            // 缩放
    Custom(String),     // 自定义
}


// 基础属性字典方法
impl AttributesDictionary {
   
    /// 创建一个空属性字典
    pub fn new() -> Self {
        Self {
            group_action: vec![],
            color_display: None,
            label: None,
            font_size: None,
            name: None,
            note: None,
            is_dirty: false,
        }
    }

    /// 添加群操作
    pub fn add_group_action(&mut self, action: GroupAction) {
        self.group_action.push(action);
        self.is_dirty = true;
    }

    /// 设置颜色
    pub fn set_color_display(&mut self, color: String) {
        self.color_display = Some(color);
        self.is_dirty = true;
    }

    /// 设置名称
    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
        self.is_dirty = true;
    }
}


/// 基础数学内核方法
impl MathKernel {

    /// 创建一个空的数学内核
    pub fn new() -> Self {
        Self {
            vertex_list: Vec::new(),
            face_list: Vec::new(),
            render_faces_list: Vec::new(),
            edge_list: Vec::new(),
            is_dirty: false,
            attributes: AttributesDictionary::new(),
        }
    }


    // 读取接口 

    /// 读取所有顶点
    pub fn read_vertices(&self) -> &Vec<VecN<3>> {
        &self.vertex_list
    }

    /// 读取所有面
    pub fn read_faces(&self) -> &Vec<Vec<usize>> {
        &self.face_list
    }
    
    /// 读取所有渲染面
    pub fn read_render_faces(&self) -> &Vec<[usize; 3]> {
        &self.render_faces_list
    }


    /// 读取所有边
    pub fn read_edges(&self) -> &Vec<[usize; 2]> {
        &self.edge_list
    }


    // 写入接口

    /// 写入顶点列表
    pub fn write_vertices(&mut self, new_vertices: Vec<VecN<3>>) {
        self.vertex_list = new_vertices;
        self.is_dirty = true;
    }

    /// 写入面列表
    pub fn write_faces(&mut self, new_faces: Vec<Vec<usize>>) {
        self.face_list = new_faces;
        self.is_dirty = true;
    }

    /// 写入所有渲染面
    pub fn write_render_faces(&mut self, new_render_faces: Vec<[usize; 3]>) {
        self.render_faces_list = new_render_faces;
        self.is_dirty = true;
    }

    /// 写入边列表
    pub fn write_edges(&mut self, new_edges: Vec<[usize; 2]>) {
        self.edge_list = new_edges;
        self.is_dirty = true;
    }

    //所有读写方法都是复数形式


    /// 重置所有数据（清空）
    pub fn reset(&mut self) {
        self.vertex_list.clear();
        self.face_list.clear();
        self.edge_list.clear();
        self.is_dirty = false;
    }


    // 加载默认

    /// 加载第一默认正方体（8个顶点与12个三角形）
    pub fn load_default_cube1(& mut self) {
        // 边长等于 1
        let h = 0.5;

        // 8个顶点
        self.vertex_list = vec![
            [-h,-h,-h],     //0
            [h,-h,-h],      //1
            [h,h,-h],       //2
            [-h,h,-h],      //3
            [-h,-h,h],      //4
            [h,-h,h],       //5
            [h,h,h],        //6
            [-h,h,h],       //7
        ];

        // 数学面：六个四边形（每个面4个顶点索引）
        self.face_list = vec![
            vec![4, 5, 6, 7],       // Z+方向面
            vec![0, 1, 2, 3],       // Z-方向面
            vec![1, 5, 6, 2],       // X+方向面
            vec![0, 3, 7, 4],       // X-方向面
            vec![3, 2, 6, 7],       // Y+方向面
            vec![0, 1, 5, 4],       // Y-方向面
        ];

        // 渲染面： 12个三角形（每个四边形拆成两个三角形顶点索引）
        self.render_faces_list = vec![
            // Z+方向面
            [4, 5, 6],
            [4, 6, 7],
            // Z-方向面
            [0, 1, 2],
            [0, 2, 3],
            // X+方向面
            [1, 5, 6],
            [1, 6, 2],
            // X-方向面
            [0, 3, 7],
            [0, 7, 4],
            // Y+方向面
            [3, 2, 6],
            [3, 6, 7],
            // Y-方向面
            [0, 1, 5],
            [0, 5, 4],
        ];     

        // 边： 12条边（ 12条棱的顶点索引）
        self.edge_list = vec![
            // 底面 (z = -h)
            [0, 1], // 底后
            [1, 2], // 底右
            [2, 3], // 底前
            [3, 0], // 底左

            // 顶面 (z = +h)
            [4, 5], // 顶后
            [5, 6], // 顶右
            [6, 7], // 顶前
            [7, 4], // 顶左

            // 连接上下
            [0, 4], // 左后竖
            [1, 5], // 右后竖
            [2, 6], // 右前竖
            [3, 7], // 左前竖
        ];

        //更改脏标记
        self.is_dirty = true;
    }


    /// 加载第二默认正方体（8个顶点与12个三角形）
    pub fn load_default_cube2(& mut self) {
        // 边长等于 1
        let d = 1.0;

        // 8个顶点
        self.vertex_list = vec![
            [0.0,0.0,0.0],     //0
            [d,0.0,0.0],       //1
            [d,d,0.0],         //2
            [0.0,d,0.0],       //3
            [0.0,0.0,d],       //4
            [d,0.0,d],         //5
            [d,d,d],           //6
            [0.0,d,d],         //7
        ];

        // 数学面：六个四边形（每个面4个顶点索引）
        self.face_list = vec![
            vec![4, 5, 6, 7],       // Z+方向面
            vec![0, 1, 2, 3],       // Z-方向面
            vec![1, 5, 6, 2],       // X+方向面
            vec![0, 3, 7, 4],       // X-方向面
            vec![3, 2, 6, 7],       // Y+方向面
            vec![0, 1, 5, 4],       // Y-方向面
        ];

        // 渲染面： 12个三角形（每个四边形拆成两个三角形顶点索引）
        self.render_faces_list = vec![
            // Z+方向面
            [4, 5, 6],
            [4, 6, 7],
            // Z-方向面
            [0, 1, 2],
            [0, 2, 3],
            // X+方向面
            [1, 5, 6],
            [1, 6, 2],
            // X-方向面
            [0, 3, 7],
            [0, 7, 4],
            // Y+方向面
            [3, 2, 6],
            [3, 6, 7],
            // Y-方向面
            [0, 1, 5],
            [0, 5, 4],
        ];     

        // 边： 12条边（ 12条棱的顶点索引）
        self.edge_list = vec![
            // 底面 (z = -h)
            [0, 1], // 底后
            [1, 2], // 底右
            [2, 3], // 底前
            [3, 0], // 底左

            // 顶面 (z = +h)
            [4, 5], // 顶后
            [5, 6], // 顶右
            [6, 7], // 顶前
            [7, 4], // 顶左

            // 连接上下
            [0, 4], // 左后竖
            [1, 5], // 右后竖
            [2, 6], // 右前竖
            [3, 7], // 左前竖
        ];

        //更改脏标记
        self.is_dirty = true;
    }

    
    // 加载其他

    ///加载几何体圆（待办）
    pub fn load_default_sphere(& mut self) {
        todo!("这个功能还在开发喵！");
    }

    ///加载几何体正方形（待办）
    pub fn load_default_square(& mut self) {
        todo!("这个功能还在开发喵！");
    }

    ///加载几何体圆柱（待办）
    pub fn load_default_cylinder(& mut self) {
        todo!("这个功能还在开发喵！");
    }

    ///加载几何体圆锥（待办）
    pub fn load_default_cone(& mut self) {
        todo!("这个功能还在开发喵！");
    }

    ///加载几何体平面（待办）
    pub fn load_default_plane(& mut self) {
        todo!("这个功能还在开发喵！");
    }

    ///加载几何体圆环（待办）
    pub fn load_default_annulus(& mut self) {
        todo!("这个功能还在开发喵！");
    }


    // 基础操作


    // 旋转操作

    /// 绕X轴的旋转操作（逆时针为正，角度：弧度/角度（判断在抽象数学计算器里））
    pub fn transformation_rotate_x(& mut self , mut theta: Float) {
        let cos = theta.cos();
        let sin = theta.sin();
        for v in & mut self.vertex_list {
            let y = v[1];
            let z = v[0];
            v[1] = y * cos - z * sin;
            v[0] = y * sin + z * cos;
        }
        self.is_dirty = true;
    }

    /// 绕y轴的旋转操作（逆时针为正，角度：弧度/角度（判断在抽象数学计算器里））
    pub fn transformation_rotate_y(& mut self , mut theta: Float) {
        let cos = theta.cos();
        let sin = theta.sin();
        for v in & mut self.vertex_list {
            let x = v[1];
            let z = v[2];
            v[1] = x * cos + z * sin;
            v[2] = -x * sin + z * cos;
        }
        self.is_dirty = true;
    }

    /// 绕z轴的旋转操作（逆时针为正，角度：弧度/角度（判断在抽象数学计算器里））
    pub fn transformation_rotate_z(& mut self , mut theta: Float) {
        let cos = theta.cos();
        let sin = theta.sin();
        for v in & mut self.vertex_list {
            let x = v[0];
            let y = v[2];
            v[0] = x * cos - y * sin;
            v[2] = x * sin + y * cos;
        }
        self.is_dirty = true;
    }

    /// 绕任意轴旋转操作（逆时针为正，角度：弧度/角度（判断在抽象数学计算器里））
    pub fn transformation_rotate_any(& mut self , axis_vector: VecN<3> , mut theta: Float) {
        todo!("这个功能还在开发喵！")
    }

    // 缩放操作

    /// 全局等比例缩放操作
    pub fn transformation_scale_global(& mut self , ratio: Float) {
        let mut sum = [0.0,0.0,0.0];                                    //第一期的临时计算
        for v in &self.vertex_list {                                   //第一期的临时计算
            sum[0] += v[0];                                                       //第一期的临时计算
            sum[1] += v[1];                                                       //第一期的临时计算
            sum[2] += v[2];                                                       //第一期的临时计算
        }                                                                         //第一期的临时计算
        let n = self.vertex_list.len() as f64;                               //第一期的临时计算
        let the_general_weighted_center = [sum[0]/n,sum[1]/n,sum[2]/n]; //第一期的临时计算
        for v in &mut self.vertex_list {
            v[0] = the_general_weighted_center[0] + (v[0] - the_general_weighted_center[0]) * ratio;
            v[1] = the_general_weighted_center[1] + (v[1] - the_general_weighted_center[1]) * ratio;
            v[2] = the_general_weighted_center[2] + (v[2] - the_general_weighted_center[2]) * ratio;
        }
        self.is_dirty = true;
    }

    /// 沿x轴等比例缩放操作
    pub fn transformation_scale_x(& mut self , ratio: Float) {
        let mut sum = [0.0,0.0,0.0];                                    //第一期的临时计算
        for v in &self.vertex_list {                                   //第一期的临时计算
            sum[0] += v[0];                                                       //第一期的临时计算
            sum[1] += v[1];                                                       //第一期的临时计算
            sum[2] += v[2];                                                       //第一期的临时计算
        }                                                                         //第一期的临时计算
        let n = self.vertex_list.len() as f64;                               //第一期的临时计算
        let the_general_weighted_center = [sum[0]/n,sum[1]/n,sum[2]/n]; //第一期的临时计算
        for v in &mut self.vertex_list {
            v[2] = the_general_weighted_center[2] + (v[2] - the_general_weighted_center[2]) * ratio;
        }
        self.is_dirty = true;
    }

    /// 沿y轴等比例缩放操作
    pub fn transformation_scale_y(& mut self , ratio: Float) {
        let mut sum = [0.0,0.0,0.0];                                    //第一期的临时计算
        for v in &self.vertex_list {                                   //第一期的临时计算
            sum[0] += v[0];                                                       //第一期的临时计算
            sum[1] += v[1];                                                       //第一期的临时计算
            sum[2] += v[2];                                                       //第一期的临时计算
        }                                                                         //第一期的临时计算
        let n = self.vertex_list.len() as f64;                               //第一期的临时计算
        let the_general_weighted_center = [sum[0]/n,sum[1]/n,sum[2]/n]; //第一期的临时计算
        for v in &mut self.vertex_list {
            v[0] = the_general_weighted_center[0] + (v[0] - the_general_weighted_center[0]) * ratio;
        }
        self.is_dirty = true;
    }

    /// 沿z轴等比例缩放操作
    pub fn transformation_scale_z(& mut self , ratio: Float) {
        let mut sum = [0.0,0.0,0.0];                                    //第一期的临时计算
        for v in &self.vertex_list {                                   //第一期的临时计算
            sum[0] += v[0];                                                       //第一期的临时计算
            sum[1] += v[1];                                                       //第一期的临时计算
            sum[2] += v[2];                                                       //第一期的临时计算
        }                                                                         //第一期的临时计算
        let n = self.vertex_list.len() as f64;                               //第一期的临时计算
        let the_general_weighted_center = [sum[0]/n,sum[1]/n,sum[2]/n]; //第一期的临时计算
        for v in &mut self.vertex_list {
            v[1] = the_general_weighted_center[1] + (v[1] - the_general_weighted_center[1]) * ratio;
        }
        self.is_dirty = true;
    }

    /// 全局按权重中心等比例缩放操作
    pub fn transformation_weighted_scale_global(& mut self , mut ratio: Float) {
        todo!("这个功能还在开发喵！")
    }


    // 平移操作

    /// 沿x轴平移dx个单位操作
    pub fn transformation_translation_x(& mut self , dx: Float) {
        for v in & mut self.vertex_list {
            v[2] += dx;
        }
        self.is_dirty = true;
    }

    /// 沿y轴平移dy个单位操作
    pub fn transformation_translation_y(& mut self , dy: Float) {
        for v in & mut self.vertex_list {
            v[0] += dy;
        }
        self.is_dirty = true;
    }

    /// 沿z轴平移dz个单位操作
    pub fn transformation_translation_z(& mut self , dz: Float) {
        for v in & mut self.vertex_list {
            v[1] += dz;
        }
        self.is_dirty = true;
    }

    /// 整体移动到（x，y，z）位置操作
    pub fn transformation_moveto_global(& mut self ,target_x: Float ,target_y: Float ,target_z: Float) {
        let mut sum = [0.0,0.0,0.0];                                              //第一期的临时计算
        for v in &self.vertex_list {                                              //第一期的临时计算
            sum[0] += v[0];                                                       //第一期的临时计算
            sum[1] += v[1];                                                       //第一期的临时计算
            sum[2] += v[2];                                                       //第一期的临时计算
        }                                                                         //第一期的临时计算
        let n = self.vertex_list.len() as f64;                                    //第一期的临时计算
        let the_general_weighted_center = [sum[0]/n,sum[1]/n,sum[2]/n];           //第一期的临时计算
        self.transformation_translation_x(target_x - the_general_weighted_center[0]);
        self.transformation_translation_y(target_y - the_general_weighted_center[1]);
        self.transformation_translation_z(target_z - the_general_weighted_center[2]);
    }


    // 自动校验

    /// 自动更新脏标记
    pub fn sync_dirty(&mut self){
        if self.is_dirty || self.attributes.is_dirty {
            self.is_dirty = true;
            self.attributes.is_dirty = true;
        }
    }

    /// 自动同步渲染面
    pub fn sync_render_faces(&mut self) {
        self.render_faces_list.clear();

        for face in &self.face_list {
            if face.len() < 3 {
                continue;
            }
            let first = face[0];
            for i in 1..face.len()-1 {
                self.render_faces_list.push([first,face[i],face[i+1]]);
            }
        }

        self.is_dirty = true;
    }


    // 添加
    
    /// 添加顶点
    pub fn add_vertex(&mut self, position: VecN<3>) {
        self.vertex_list.push(position);
        self.is_dirty = true;
    }

    /// 添加面（顶点索引）
    pub fn add_face(&mut self, vertices: Vec<usize>) {
        self.face_list.push(vertices);
        self.is_dirty = true;
    }

    /// 添加边（顶点索引）
    pub fn add_edge(&mut self, vertex1: usize, vertex2: usize) {
        self.edge_list.push([vertex1, vertex2]);
        self.is_dirty = true;
    }

    /// 添加所有（顶点、面、边）
    pub fn add_all(&mut self, vertices: Vec<VecN<3>>, faces: Vec<Vec<usize>>, edges: Vec<[usize; 2]>) {
        self.vertex_list.extend(vertices);
        self.face_list.extend(faces);
        self.edge_list.extend(edges);
        self.is_dirty = true;
    }


    // 删除

    /// 删除顶点（索引）
    pub fn remove_vertex(&mut self, index: usize) {
        self.vertex_list.remove(index);
        self.is_dirty = true;
    }

    /// 删除面（索引）
    pub fn remove_face(&mut self, index: usize) {
        self.face_list.remove(index);
        self.is_dirty = true;
    }

    /// 删除边（索引）
    pub fn remove_edge(&mut self, index: usize) {
        self.edge_list.remove(index);
        self.is_dirty = true;
    }

    /// 删除所有（索引）
    pub fn remove_all(&mut self) {
        self.vertex_list.clear();
        self.face_list.clear();
        self.edge_list.clear();
        self.is_dirty = true;
    }

}