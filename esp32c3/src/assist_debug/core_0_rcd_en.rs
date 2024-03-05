#[doc = "Register `CORE_0_RCD_EN` reader"]
pub type R = crate::R<CORE_0_RCD_EN_SPEC>;
#[doc = "Register `CORE_0_RCD_EN` writer"]
pub type W = crate::W<CORE_0_RCD_EN_SPEC>;
#[doc = "Field `CORE_0_RCD_RECORDEN` reader - reg_core_0_rcd_recorden"]
pub type CORE_0_RCD_RECORDEN_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_RECORDEN` writer - reg_core_0_rcd_recorden"]
pub type CORE_0_RCD_RECORDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` reader - reg_core_0_rcd_pdebugen"]
pub type CORE_0_RCD_PDEBUGEN_R = crate::BitReader;
#[doc = "Field `CORE_0_RCD_PDEBUGEN` writer - reg_core_0_rcd_pdebugen"]
pub type CORE_0_RCD_PDEBUGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_core_0_rcd_recorden"]
    #[inline(always)]
    pub fn core_0_rcd_recorden(&self) -> CORE_0_RCD_RECORDEN_R {
        CORE_0_RCD_RECORDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_core_0_rcd_pdebugen"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugen(&self) -> CORE_0_RCD_PDEBUGEN_R {
        CORE_0_RCD_PDEBUGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_EN")
            .field(
                "core_0_rcd_recorden",
                &format_args!("{}", self.core_0_rcd_recorden().bit()),
            )
            .field(
                "core_0_rcd_pdebugen",
                &format_args!("{}", self.core_0_rcd_pdebugen().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_core_0_rcd_recorden"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_rcd_recorden(&mut self) -> CORE_0_RCD_RECORDEN_W<CORE_0_RCD_EN_SPEC> {
        CORE_0_RCD_RECORDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_core_0_rcd_pdebugen"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_rcd_pdebugen(&mut self) -> CORE_0_RCD_PDEBUGEN_W<CORE_0_RCD_EN_SPEC> {
        CORE_0_RCD_PDEBUGEN_W::new(self, 1)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_EN_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_en::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_rcd_en::W`](W) writer structure"]
impl crate::Writable for CORE_0_RCD_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_RCD_EN to value 0"]
impl crate::Resettable for CORE_0_RCD_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
