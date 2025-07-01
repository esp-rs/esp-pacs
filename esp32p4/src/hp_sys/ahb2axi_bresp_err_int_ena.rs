#[doc = "Register `AHB2AXI_BRESP_ERR_INT_ENA` reader"]
pub type R = crate::R<AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "Register `AHB2AXI_BRESP_ERR_INT_ENA` writer"]
pub type W = crate::W<AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_ENA` reader - Write 1 to enable cpu_icm_h2x_bresp_err int"]
pub type CPU_ICM_H2X_BRESP_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_ENA` writer - Write 1 to enable cpu_icm_h2x_bresp_err int"]
pub type CPU_ICM_H2X_BRESP_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_ena(&self) -> CPU_ICM_H2X_BRESP_ERR_INT_ENA_R {
        CPU_ICM_H2X_BRESP_ERR_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2AXI_BRESP_ERR_INT_ENA")
            .field(
                "cpu_icm_h2x_bresp_err_int_ena",
                &self.cpu_icm_h2x_bresp_err_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_ena(
        &mut self,
    ) -> CPU_ICM_H2X_BRESP_ERR_INT_ENA_W<AHB2AXI_BRESP_ERR_INT_ENA_SPEC> {
        CPU_ICM_H2X_BRESP_ERR_INT_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2axi_bresp_err_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2AXI_BRESP_ERR_INT_ENA_SPEC;
impl crate::RegisterSpec for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2axi_bresp_err_int_ena::R`](R) reader structure"]
impl crate::Readable for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2axi_bresp_err_int_ena::W`](W) writer structure"]
impl crate::Writable for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2AXI_BRESP_ERR_INT_ENA to value 0"]
impl crate::Resettable for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {}
