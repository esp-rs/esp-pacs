#[doc = "Register `CORE_0_RCD_EN` reader"]
pub type R = crate::R<CORE_0_RCD_EN_SPEC>;
#[doc = "Register `CORE_0_RCD_EN` writer"]
pub type W = crate::W<CORE_0_RCD_EN_SPEC>;
#[doc = "Field `CORE_0_RCD_RECORDEN` reader - Configures whether to enable PC logging.\\\\ 0: Disable\\\\ 1: ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG starts to record PC in real time\\\\"]
pub type CORE_0_RCD_RECORDEN_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_RECORDEN` writer - Configures whether to enable PC logging.\\\\ 0: Disable\\\\ 1: ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG starts to record PC in real time\\\\"]
pub type CORE_0_RCD_RECORDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` reader - Configures whether to enable HP CPU debugging.\\\\ 0: Disable\\\\ 1: HP CPU outputs PC\\\\"]
pub type CORE_0_RCD_PDEBUGEN_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` writer - Configures whether to enable HP CPU debugging.\\\\ 0: Disable\\\\ 1: HP CPU outputs PC\\\\"]
pub type CORE_0_RCD_PDEBUGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to enable PC logging.\\\\ 0: Disable\\\\ 1: ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG starts to record PC in real time\\\\"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&self) -> CORE_0_RCD_RECORDEN_R {
        CORE_0_RCD_RECORDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether to enable HP CPU debugging.\\\\ 0: Disable\\\\ 1: HP CPU outputs PC\\\\"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&self) -> CORE_0_RCD_PDEBUGEN_R {
        CORE_0_RCD_PDEBUGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_EN")
            .field("core_0_rcd_recorden", &self.core_0_rcd_recorden())
            .field("core_0_rcd_pdebugen", &self.core_0_rcd_pdebugen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable PC logging.\\\\ 0: Disable\\\\ 1: ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC_REG starts to record PC in real time\\\\"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&mut self) -> CORE_0_RCD_RECORDEN_W<CORE_0_RCD_EN_SPEC> {
        CORE_0_RCD_RECORDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to enable HP CPU debugging.\\\\ 0: Disable\\\\ 1: HP CPU outputs PC\\\\"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&mut self) -> CORE_0_RCD_PDEBUGEN_W<CORE_0_RCD_EN_SPEC> {
        CORE_0_RCD_PDEBUGEN_W::new(self, 1)
    }
}
#[doc = "HP CPU PC logging enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_rcd_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_rcd_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_EN_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_en::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_rcd_en::W`](W) writer structure"]
impl crate::Writable for CORE_0_RCD_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_RCD_EN to value 0"]
impl crate::Resettable for CORE_0_RCD_EN_SPEC {}
