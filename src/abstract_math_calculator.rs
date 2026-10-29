use crate::types::VecN;


/// 抽象数学计算器

/// 一期：实现 Z³、Z、R 等的代数运算

/// 群计算器（底层抽象）
// ==================================================================
pub trait AbstractMathCalculatorGroup {
    type Element;
    fn op(&self, a: &Self::Element, b: &Self::Element) -> Self::Element;
    fn inv(&self, a: &Self::Element) -> Self::Element;
    fn identity(&self) -> Self::Element;

    /// 判断元素是否为挠元
    fn is_torsion(&self, e: &Self::Element) -> bool {
        false
    }
}
// ==================================================================

/// 加法群Z^N（代数层）
// ==================================================================
#[derive(Debug, Clone, Copy)]
pub struct AbstractMathCalculatorAddGroup<const N: usize>;

impl<const N: usize> AbstractMathCalculatorGroup for AbstractMathCalculatorAddGroup<N> {
    type Element = VecN<N>;

    /// 加法运算（Z 群运算）
    fn op(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = a[i] + b[i];
        }
        result
    }

    /// 逆元运算（Z 群逆元）
    fn inv(&self, a: &Self::Element) -> Self::Element {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = -a[i];
        }
        result
    }

    /// 单位元运算（Z 群单位元）
    fn identity(&self) -> Self::Element {
        [0.0; N]
    }
}
// ==================================================================

/// 乘法群 R⁺（代数层）
// ==================================================================
#[derive(Debug, Clone, Copy)]
pub struct AbstractMathCalculatorMulGroup<const N: usize>;

impl<const N: usize> AbstractMathCalculatorGroup for AbstractMathCalculatorMulGroup<N> {
    type Element = VecN<N>;

    /// 逐分量乘法
    fn op(&self, v1: &Self::Element, v2: &Self::Element) -> Self::Element {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = v1[i] * v2[i];
        }
        result
    }

    /// 逐分量逆元
    fn inv(&self, v: &Self::Element) -> Self::Element {
        let mut result = [0.0; N];
        for i in 0..N {
            result[i] = 1.0 / v[i];
        }
        result
    }

    /// 单位元
    fn identity(&self) -> Self::Element {
        [1.0; N]
    }

}
// ==================================================================

/// 挠群 T（代数层）
// ==================================================================
#[derive(Debug, Clone, Copy)]
pub struct Rotation{
    pub axis: VecN<3>, // 旋转轴
    pub angle: f64,    // 旋转角度（弧度）  
}

#[derive(Debug, Clone, Copy)]
pub struct AbstractMathCalculatorTwistGroup;

impl AbstractMathCalculatorGroup for AbstractMathCalculatorTwistGroup {
    type Element = Rotation;

    fn is_torsion(&self, e: &Self::Element) -> bool {
        // 判断旋转角度是否为 2π 的整数倍
        let ratio = e.angle / (2.0 * std::f64::consts::PI);
        (ratio - ratio.round()).abs() < 1e-10
    }

    /// 旋转复合运算
    fn op(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        unimplemented!("旋转复合运算，二期实现");
    }

    /// 旋转逆元运算
    fn inv(&self, a: &Self::Element) -> Self::Element {
        unimplemented!("旋转逆元运算，二期实现");
    }

    /// 单位元运算
    fn identity(&self) -> Self::Element {
        Rotation {
            axis: [0.0, 0.0, 1.0], // 默认单位旋转轴
            angle: 0.0,           // 默认角度为 0
        }
    }
    
}

/// ==================================================================

/// 平移群（Z^N）计算器（实现层）
// ==================================================================
pub type AbstractMathCalculatorTranslationAddGroup<const N: usize> = AbstractMathCalculatorAddGroup<N>;
// ==================================================================

/// 缩放群（R⁺）计算器（实现层）
// ==================================================================
pub type AbstractMathCalculatorScaleMulGroup<const N: usize> = AbstractMathCalculatorMulGroup<N>;
// ==================================================================

/// 旋转群（SO(3)）计算器（实现层）
// 一期只占位，二期实现四元数或罗德里格公式
// ==================================================================
pub type AbstractMathCalculatorRotationTwistGroup = AbstractMathCalculatorTwistGroup;
// ==================================================================

/// 群的直和运算（G ⊕ H）
// ==================================================================

#[derive(Debug, Clone, Copy)]
pub struct DirectSum<G1, G2>(G1, G2);

impl<G1, G2> AbstractMathCalculatorGroup for DirectSum<G1, G2>
where
    G1: AbstractMathCalculatorGroup,
    G2: AbstractMathCalculatorGroup,
{
    type Element = (G1::Element, G2::Element);

    fn identity(&self) -> Self::Element {
        (self.0.identity(), self.1.identity())
    }

    fn op(&self, a: &Self::Element, b: &Self::Element) -> Self::Element {
        (self.0.op(&a.0, &b.0), self.1.op(&a.1, &b.1))
    }

    fn inv(&self, a: &Self::Element) -> Self::Element {
        (self.0.inv(&a.0), self.1.inv(&a.1))
    }
}

// ==================================================================