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
                &self.hp_reg_core0_corestalled_st(),
            )
            .field(
                "hp_reg_core1_corestalled_st",
                &self.hp_reg_core1_corestalled_st(),
            )
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_cpu_corestalled_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CPU_CORESTALLED_ST_SPEC;
impl crate::RegisterSpec for HP_CPU_CORESTALLED_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cpu_corestalled_st::R`](R) reader structure"]
impl crate::Readable for HP_CPU_CORESTALLED_ST_SPEC {}
#[doc = "`reset()` method sets HP_CPU_CORESTALLED_ST to value 0"]
impl crate::Resettable for HP_CPU_CORESTALLED_ST_SPEC {}
