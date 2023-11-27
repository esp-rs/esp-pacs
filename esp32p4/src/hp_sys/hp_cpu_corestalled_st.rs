#[doc = "Register `HP_CPU_CORESTALLED_ST` reader"]
pub type R = crate::R<HP_CPU_CORESTALLED_ST_SPEC>;
#[doc = "Field `HP_REG_CORE0_CORESTALLED_ST` reader - hp core0 corestalled status"]
pub type HP_REG_CORE0_CORESTALLED_ST_R = crate::BitReader;
#[doc = "Field `HP_REG_CORE1_CORESTALLED_ST` reader - hp core1 corestalled status"]
pub type HP_REG_CORE1_CORESTALLED_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - hp core0 corestalled status"]
    #[inline(always)]
    pub fn hp_reg_core0_corestalled_st(&self) -> HP_REG_CORE0_CORESTALLED_ST_R {
        HP_REG_CORE0_CORESTALLED_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hp core1 corestalled status"]
    #[inline(always)]
    pub fn hp_reg_core1_corestalled_st(&self) -> HP_REG_CORE1_CORESTALLED_ST_R {
        HP_REG_CORE1_CORESTALLED_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CPU_CORESTALLED_ST")
            .field(
                "hp_reg_core0_corestalled_st",
                &format_args!("{}", self.hp_reg_core0_corestalled_st().bit()),
            )
            .field(
                "hp_reg_core1_corestalled_st",
                &format_args!("{}", self.hp_reg_core1_corestalled_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CPU_CORESTALLED_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_corestalled_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CPU_CORESTALLED_ST_SPEC;
impl crate::RegisterSpec for HP_CPU_CORESTALLED_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cpu_corestalled_st::R`](R) reader structure"]
impl crate::Readable for HP_CPU_CORESTALLED_ST_SPEC {}
#[doc = "`reset()` method sets HP_CPU_CORESTALLED_ST to value 0"]
impl crate::Resettable for HP_CPU_CORESTALLED_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
