#[doc = "Register `CPU_CLK_STATUS0` reader"]
pub type R = crate::R<CPU_CLK_STATUS0_SPEC>;
#[doc = "Field `REG_ASIC_OR_FPGA` reader - 0: ASIC mode, 1: FPGA mode"]
pub type REG_ASIC_OR_FPGA_R = crate::BitReader;
#[doc = "Field `REG_CPU_DIV_EFFECT` reader - 0: Divider bypass, 1: Divider takes effect"]
pub type REG_CPU_DIV_EFFECT_R = crate::BitReader;
#[doc = "Field `REG_CPU_SRC_IS_CPLL` reader - 0: CPU source isn't cpll_400m, 1: CPU Source is cll_400m"]
pub type REG_CPU_SRC_IS_CPLL_R = crate::BitReader;
#[doc = "Field `REG_CPU_DIV_NUM_CUR` reader - cpu current div number"]
pub type REG_CPU_DIV_NUM_CUR_R = crate::FieldReader;
#[doc = "Field `REG_CPU_DIV_NUMERATOR_CUR` reader - cpu current div numerator"]
pub type REG_CPU_DIV_NUMERATOR_CUR_R = crate::FieldReader;
#[doc = "Field `REG_CPU_DIV_DENOMINATOR_CUR` reader - cpu current div denominator"]
pub type REG_CPU_DIV_DENOMINATOR_CUR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0: ASIC mode, 1: FPGA mode"]
    #[inline(always)]
    pub fn reg_asic_or_fpga(&self) -> REG_ASIC_OR_FPGA_R {
        REG_ASIC_OR_FPGA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Divider bypass, 1: Divider takes effect"]
    #[inline(always)]
    pub fn reg_cpu_div_effect(&self) -> REG_CPU_DIV_EFFECT_R {
        REG_CPU_DIV_EFFECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: CPU source isn't cpll_400m, 1: CPU Source is cll_400m"]
    #[inline(always)]
    pub fn reg_cpu_src_is_cpll(&self) -> REG_CPU_SRC_IS_CPLL_R {
        REG_CPU_SRC_IS_CPLL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - cpu current div number"]
    #[inline(always)]
    pub fn reg_cpu_div_num_cur(&self) -> REG_CPU_DIV_NUM_CUR_R {
        REG_CPU_DIV_NUM_CUR_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 11:18 - cpu current div numerator"]
    #[inline(always)]
    pub fn reg_cpu_div_numerator_cur(&self) -> REG_CPU_DIV_NUMERATOR_CUR_R {
        REG_CPU_DIV_NUMERATOR_CUR_R::new(((self.bits >> 11) & 0xff) as u8)
    }
    #[doc = "Bits 19:26 - cpu current div denominator"]
    #[inline(always)]
    pub fn reg_cpu_div_denominator_cur(&self) -> REG_CPU_DIV_DENOMINATOR_CUR_R {
        REG_CPU_DIV_DENOMINATOR_CUR_R::new(((self.bits >> 19) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_CLK_STATUS0")
            .field(
                "reg_asic_or_fpga",
                &format_args!("{}", self.reg_asic_or_fpga().bit()),
            )
            .field(
                "reg_cpu_div_effect",
                &format_args!("{}", self.reg_cpu_div_effect().bit()),
            )
            .field(
                "reg_cpu_src_is_cpll",
                &format_args!("{}", self.reg_cpu_src_is_cpll().bit()),
            )
            .field(
                "reg_cpu_div_num_cur",
                &format_args!("{}", self.reg_cpu_div_num_cur().bits()),
            )
            .field(
                "reg_cpu_div_numerator_cur",
                &format_args!("{}", self.reg_cpu_div_numerator_cur().bits()),
            )
            .field(
                "reg_cpu_div_denominator_cur",
                &format_args!("{}", self.reg_cpu_div_denominator_cur().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_CLK_STATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "CPU Clock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_clk_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_CLK_STATUS0_SPEC;
impl crate::RegisterSpec for CPU_CLK_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_clk_status0::R`](R) reader structure"]
impl crate::Readable for CPU_CLK_STATUS0_SPEC {}
#[doc = "`reset()` method sets CPU_CLK_STATUS0 to value 0"]
impl crate::Resettable for CPU_CLK_STATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
