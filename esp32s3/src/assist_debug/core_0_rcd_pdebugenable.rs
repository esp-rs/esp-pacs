#[doc = "Register `CORE_0_RCD_PDEBUGENABLE` reader"]
pub type R = crate::R<CORE_0_RCD_PDEBUGENABLE_SPEC>;
#[doc = "Register `CORE_0_RCD_PDEBUGENABLE` writer"]
pub type W = crate::W<CORE_0_RCD_PDEBUGENABLE_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGENABLE` reader - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
pub type CORE_0_RCD_PDEBUGENABLE_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_PDEBUGENABLE` writer - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
pub type CORE_0_RCD_PDEBUGENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugenable(&self) -> CORE_0_RCD_PDEBUGENABLE_R {
        CORE_0_RCD_PDEBUGENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_PDEBUGENABLE")
            .field(
                "core_0_rcd_pdebugenable",
                &format_args!("{}", self.core_0_rcd_pdebugenable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_PDEBUGENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Core0 Pdebugenable,set 1 to open core0 Pdebug interface,then can get core0 PC"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_rcd_pdebugenable(
        &mut self,
    ) -> CORE_0_RCD_PDEBUGENABLE_W<CORE_0_RCD_PDEBUGENABLE_SPEC, 0> {
        CORE_0_RCD_PDEBUGENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "core0 pdebug configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_pdebugenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_PDEBUGENABLE_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_pdebugenable::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_rcd_pdebugenable::W`](W) writer structure"]
impl crate::Writable for CORE_0_RCD_PDEBUGENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGENABLE to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
