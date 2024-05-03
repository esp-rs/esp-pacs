#[doc = "Register `AHB2AXI_BRESP_ERR_INT_RAW` reader"]
pub type R = crate::R<AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "Register `AHB2AXI_BRESP_ERR_INT_RAW` writer"]
pub type W = crate::W<AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_RAW` reader - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
pub type CPU_ICM_H2X_BRESP_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_RAW` writer - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
pub type CPU_ICM_H2X_BRESP_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_raw(&self) -> CPU_ICM_H2X_BRESP_ERR_INT_RAW_R {
        CPU_ICM_H2X_BRESP_ERR_INT_RAW_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2AXI_BRESP_ERR_INT_RAW")
            .field(
                "cpu_icm_h2x_bresp_err_int_raw",
                &self.cpu_icm_h2x_bresp_err_int_raw().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB2AXI_BRESP_ERR_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_icm_h2x_bresp_err_int_raw(
        &mut self,
    ) -> CPU_ICM_H2X_BRESP_ERR_INT_RAW_W<AHB2AXI_BRESP_ERR_INT_RAW_SPEC> {
        CPU_ICM_H2X_BRESP_ERR_INT_RAW_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2axi_bresp_err_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2AXI_BRESP_ERR_INT_RAW_SPEC;
impl crate::RegisterSpec for AHB2AXI_BRESP_ERR_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2axi_bresp_err_int_raw::R`](R) reader structure"]
impl crate::Readable for AHB2AXI_BRESP_ERR_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2axi_bresp_err_int_raw::W`](W) writer structure"]
impl crate::Writable for AHB2AXI_BRESP_ERR_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2AXI_BRESP_ERR_INT_RAW to value 0"]
impl crate::Resettable for AHB2AXI_BRESP_ERR_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
