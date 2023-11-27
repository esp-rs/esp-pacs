#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_CLR` writer"]
pub type W = crate::W<HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC>;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR` writer - Write 1 to clear cpu_icm_h2x_bresp_err int"]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - Write 1 to clear cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    #[must_use]
    pub fn hp_cpu_icm_h2x_bresp_err_int_clr(
        &mut self,
    ) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR_W<HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_ahb2axi_bresp_err_int_clr::W`](W) writer structure"]
impl crate::Writable for HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_AHB2AXI_BRESP_ERR_INT_CLR to value 0"]
impl crate::Resettable for HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
