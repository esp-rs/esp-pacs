#[doc = "Register `FPGA_DEBUG_PROBE` reader"]
pub type R = crate::R<FPGA_DEBUG_PROBE_SPEC>;
#[doc = "Register `FPGA_DEBUG_PROBE` writer"]
pub type W = crate::W<FPGA_DEBUG_PROBE_SPEC>;
#[doc = "Field `REG_I3C_MST_FPGA_DEBUG_PROBE` reader - For Debug Probe Test on FPGA"]
pub type REG_I3C_MST_FPGA_DEBUG_PROBE_R = crate::FieldReader<u32>;
#[doc = "Field `REG_I3C_MST_FPGA_DEBUG_PROBE` writer - For Debug Probe Test on FPGA"]
pub type REG_I3C_MST_FPGA_DEBUG_PROBE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For Debug Probe Test on FPGA"]
    #[inline(always)]
    pub fn reg_i3c_mst_fpga_debug_probe(&self) -> REG_I3C_MST_FPGA_DEBUG_PROBE_R {
        REG_I3C_MST_FPGA_DEBUG_PROBE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPGA_DEBUG_PROBE")
            .field(
                "reg_i3c_mst_fpga_debug_probe",
                &self.reg_i3c_mst_fpga_debug_probe(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - For Debug Probe Test on FPGA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i3c_mst_fpga_debug_probe(
        &mut self,
    ) -> REG_I3C_MST_FPGA_DEBUG_PROBE_W<FPGA_DEBUG_PROBE_SPEC> {
        REG_I3C_MST_FPGA_DEBUG_PROBE_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpga_debug_probe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpga_debug_probe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPGA_DEBUG_PROBE_SPEC;
impl crate::RegisterSpec for FPGA_DEBUG_PROBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpga_debug_probe::R`](R) reader structure"]
impl crate::Readable for FPGA_DEBUG_PROBE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpga_debug_probe::W`](W) writer structure"]
impl crate::Writable for FPGA_DEBUG_PROBE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPGA_DEBUG_PROBE to value 0x01"]
impl crate::Resettable for FPGA_DEBUG_PROBE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
