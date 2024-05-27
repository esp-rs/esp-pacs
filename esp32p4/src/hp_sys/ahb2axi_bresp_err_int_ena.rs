///Register `AHB2AXI_BRESP_ERR_INT_ENA` reader
pub type R = crate::R<AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
///Register `AHB2AXI_BRESP_ERR_INT_ENA` writer
pub type W = crate::W<AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
///Field `CPU_ICM_H2X_BRESP_ERR_INT_ENA` reader - Write 1 to enable cpu_icm_h2x_bresp_err int
pub type CPU_ICM_H2X_BRESP_ERR_INT_ENA_R = crate::BitReader;
///Field `CPU_ICM_H2X_BRESP_ERR_INT_ENA` writer - Write 1 to enable cpu_icm_h2x_bresp_err int
pub type CPU_ICM_H2X_BRESP_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int
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
    ///Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int
    #[inline(always)]
    #[must_use]
    pub fn cpu_icm_h2x_bresp_err_int_ena(
        &mut self,
    ) -> CPU_ICM_H2X_BRESP_ERR_INT_ENA_W<AHB2AXI_BRESP_ERR_INT_ENA_SPEC> {
        CPU_ICM_H2X_BRESP_ERR_INT_ENA_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ahb2axi_bresp_err_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AHB2AXI_BRESP_ERR_INT_ENA_SPEC;
impl crate::RegisterSpec for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ahb2axi_bresp_err_int_ena::R`](R) reader structure
impl crate::Readable for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`ahb2axi_bresp_err_int_ena::W`](W) writer structure
impl crate::Writable for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB2AXI_BRESP_ERR_INT_ENA to value 0
impl crate::Resettable for AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
