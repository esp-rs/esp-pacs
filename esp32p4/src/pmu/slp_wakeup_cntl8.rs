#[doc = "Register `SLP_WAKEUP_CNTL8` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL8` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "Field `LP_LITE_WAKEUP_ENA` reader - need_des"]
pub type LP_LITE_WAKEUP_ENA_R = crate::BitReader;
#[doc = "Field `LP_LITE_WAKEUP_ENA` writer - need_des"]
pub type LP_LITE_WAKEUP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_lite_wakeup_ena(&self) -> LP_LITE_WAKEUP_ENA_R {
        LP_LITE_WAKEUP_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL8")
            .field(
                "lp_lite_wakeup_ena",
                &format_args!("{}", self.lp_lite_wakeup_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_lite_wakeup_ena(&mut self) -> LP_LITE_WAKEUP_ENA_W<SLP_WAKEUP_CNTL8_SPEC> {
        LP_LITE_WAKEUP_ENA_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL8_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl8::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl8::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL8 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
