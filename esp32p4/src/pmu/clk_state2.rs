#[doc = "Register `CLK_STATE2` reader"]
pub type R = crate::R<CLK_STATE2_SPEC>;
#[doc = "Field `PMU_ICG_APB_EN_STATE` reader - need_des"]
pub type PMU_ICG_APB_EN_STATE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_apb_en_state(&self) -> PMU_ICG_APB_EN_STATE_R {
        PMU_ICG_APB_EN_STATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_STATE2")
            .field(
                "pmu_icg_apb_en_state",
                &format_args!("{}", self.pmu_icg_apb_en_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_STATE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_state2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_STATE2_SPEC;
impl crate::RegisterSpec for CLK_STATE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_state2::R`](R) reader structure"]
impl crate::Readable for CLK_STATE2_SPEC {}
#[doc = "`reset()` method sets CLK_STATE2 to value 0xffff_ffff"]
impl crate::Resettable for CLK_STATE2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
