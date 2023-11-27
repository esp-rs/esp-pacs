#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_ST` reader"]
pub type R = crate::R<HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC>;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_ST` reader - the masked interrupt status of cpu_icm_h2x_bresp_err"]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - the masked interrupt status of cpu_icm_h2x_bresp_err"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bresp_err_int_st(&self) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_ST_R {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_AHB2AXI_BRESP_ERR_INT_ST")
            .field(
                "hp_cpu_icm_h2x_bresp_err_int_st",
                &format_args!("{}", self.hp_cpu_icm_h2x_bresp_err_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC;
impl crate::RegisterSpec for HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_ahb2axi_bresp_err_int_st::R`](R) reader structure"]
impl crate::Readable for HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC {}
#[doc = "`reset()` method sets HP_AHB2AXI_BRESP_ERR_INT_ST to value 0"]
impl crate::Resettable for HP_AHB2AXI_BRESP_ERR_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
