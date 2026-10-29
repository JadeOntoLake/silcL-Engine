use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct AlgorithmEditorInput;        // 输入器，用于输入算法
pub struct AlgorithmEditor;             // 算法编辑器，用于编辑算法
pub struct AlgorithmEditorRecorder;     // 记录器，用于记录算法
pub struct AlgorithmEditorOutput;       // 输出器，用于导出记录算法


impl AlgorithmEditorOutput {
    pub fn new() -> Self {
        Self
    }

    /// 导出代数结构为 LaTeX 文件（可编译为 PDF）
    pub fn export_tex(&self, content: &str, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// 生成默认的代数结构 LaTeX 文档
    pub fn generate_algebra_group_tex(&self) -> String {
        let mut content = String::new();

        /// 文档头 
        content.push_str("\\documentclass{article}\n");
        content.push_str("\\usepackage{amsmath,amssymb,amsthm}\n");
        content.push_str("\\usepackage{geometry}\n");
        content.push_str("\\geometry{a4paper, margin=2.5cm}\n");
        content.push_str("\\begin{document}\n");
        content.push_str("\\title{抽象数学计算器 - 群结构列表}\n");
        content.push_str("\\author{SilcL Engine}\n");
        content.push_str("\\date{\\today}\n");
        content.push_str("\\maketitle\n");

        // ============================================================
        /// 加法群 Z^N
        // ============================================================
        content.push_str("\\section{加法群 $\\mathbb{Z}^N$}\n");
        content.push_str("\\begin{itemize}\n");
        content.push_str("\\item 群类型: 加法群（自由阿贝尔群）\n");
        content.push_str("\\item 元素: $N$ 维向量 $\\mathbf{v} = (v_1, \\dots, v_N)$\n");
        content.push_str("\\item 单位元: $\\mathbf{0} = (0, \\dots, 0)$\n");
        content.push_str("\\item 二元运算: $\\mathbf{a} + \\mathbf{b} = (a_1 + b_1, \\dots, a_N + b_N)$\n");
        content.push_str("\\item 逆元: $-\\mathbf{v} = (-v_1, \\dots, -v_N)$\n");
        content.push_str("\\item 性质: 阿贝尔群，无挠\n");
        content.push_str("\\end{itemize}\n");

        // ============================================================
        /// 乘法群 R^N
        // ============================================================
        content.push_str("\\section{乘法群 $\\mathbb{R}^N$}\n");
        content.push_str("\\begin{itemize}\n");
        content.push_str("\\item 群类型: 逐分量乘法群\n");
        content.push_str("\\item 元素: $N$ 维向量 $\\mathbf{v} = (v_1, \\dots, v_N)$，$v_i \\neq 0$\n");
        content.push_str("\\item 单位元: $\\mathbf{1} = (1, \\dots, 1)$\n");
        content.push_str("\\item 二元运算: $\\mathbf{a} \\cdot \\mathbf{b} = (a_1 b_1, \\dots, a_N b_N)$\n");
        content.push_str("\\item 逆元: $\\mathbf{v}^{-1} = (1/v_1, \\dots, 1/v_N)$\n");
        content.push_str("\\item 性质: 阿贝尔群，无挠\n");
        content.push_str("\\end{itemize}\n");

        // ============================================================
        /// 挠群 SO(3)
        // ============================================================
        content.push_str("\\section{挠群 $SO(3)$}\n");
        content.push_str("\\begin{itemize}\n");
        content.push_str("\\item 群类型: 三维旋转群（挠群）\n");
        content.push_str("\\item 元素: 轴角表示 $(\\mathbf{n}, \\theta)$，$\\mathbf{n}$ 为单位向量\n");
        content.push_str("\\item 单位元: $(\\mathbf{n}, 0)$\n");
        content.push_str("\\item 二元运算: 四元数乘法（复合旋转）\n");
        content.push_str("\\item 逆元: $(\\mathbf{n}, -\\theta)$\n");
        content.push_str("\\item 挠元判定: $\\theta / 2\\pi \\in \\mathbb{Q}$ 时为挠元\n");
        content.push_str("\\item 性质: 非阿贝尔，紧致李群\n");
        content.push_str("\\end{itemize}\n");

        // ============================================================
        /// 直和 G ⊕ H
        // ============================================================
        content.push_str("\\section{直和 $G \\oplus H$}\n");
        content.push_str("\\begin{itemize}\n");
        content.push_str("\\item 群类型: 两个群的直和\n");
        content.push_str("\\item 元素: $(g, h)$，$g \\in G, h \\in H$\n");
        content.push_str("\\item 单位元: $(e_G, e_H)$\n");
        content.push_str("\\item 二元运算: $(g_1, h_1) \\cdot (g_2, h_2) = (g_1 \\cdot g_2, h_1 \\cdot h_2)$\n");
        content.push_str("\\item 逆元: $(g, h)^{-1} = (g^{-1}, h^{-1})$\n");
        content.push_str("\\item 性质: 逐分量独立运算\n");
        content.push_str("\\end{itemize}\n");

        /// 文档尾 
        content.push_str("\\end{document}\n");

        content
    }

    /// 生成默认的 LaTeX 文档
    pub fn generate_full_tex(&self) -> String {
        self.generate_algebra_group_tex()
    }

    /// 编译 LaTeX 文件为 PDF
    pub fn compile_pdf(&self, tex_path: &str) -> Result<(), String> {
        let output = Command::new("pdflatex")
            .arg("-interaction=nonstopmode")
            .arg(tex_path)
            .output()
            .map_err(|e| format!("未找到 pdflatex，请安装 LaTeX: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("LaTeX 编译失败:\n{}", stderr));
        }

        Ok(())
    }

}