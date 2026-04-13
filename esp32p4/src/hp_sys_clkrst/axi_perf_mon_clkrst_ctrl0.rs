#[doc = "Register `AXI_PERF_MON_CLKRST_CTRL0` reader"]
pub type R = crate::R<AXI_PERF_MON_CLKRST_CTRL0_SPEC>;
#[doc = "Register `AXI_PERF_MON_CLKRST_CTRL0` writer"]
pub type W = crate::W<AXI_PERF_MON_CLKRST_CTRL0_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_PERF_MON_CLKRST_CTRL0")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, AXI_PERF_MON_CLKRST_CTRL0_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_perf_mon_clkrst_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_perf_mon_clkrst_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_PERF_MON_CLKRST_CTRL0_SPEC;
impl crate::RegisterSpec for AXI_PERF_MON_CLKRST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_perf_mon_clkrst_ctrl0::R`](R) reader structure"]
impl crate::Readable for AXI_PERF_MON_CLKRST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_perf_mon_clkrst_ctrl0::W`](W) writer structure"]
impl crate::Writable for AXI_PERF_MON_CLKRST_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_PERF_MON_CLKRST_CTRL0 to value 0"]
impl crate::Resettable for AXI_PERF_MON_CLKRST_CTRL0_SPEC {}
