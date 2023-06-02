#[doc = "Register `HP_INT_CLR` writer"]
pub struct W(crate::W<HP_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HP_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_CPU_EXC_INT_CLR` writer - need_des"]
pub type LP_CPU_EXC_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, HP_INT_CLR_SPEC, O>;
#[doc = "Field `SDIO_IDLE_INT_CLR` writer - need_des"]
pub type SDIO_IDLE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, HP_INT_CLR_SPEC, O>;
#[doc = "Field `SW_INT_CLR` writer - need_des"]
pub type SW_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, HP_INT_CLR_SPEC, O>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_CLR` writer - need_des"]
pub type SOC_SLEEP_REJECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, HP_INT_CLR_SPEC, O>;
#[doc = "Field `SOC_WAKEUP_INT_CLR` writer - need_des"]
pub type SOC_WAKEUP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, HP_INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_exc_int_clr(&mut self) -> LP_CPU_EXC_INT_CLR_W<27> {
        LP_CPU_EXC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_clr(&mut self) -> SDIO_IDLE_INT_CLR_W<28> {
        SDIO_IDLE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sw_int_clr(&mut self) -> SW_INT_CLR_W<29> {
        SW_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_sleep_reject_int_clr(&mut self) -> SOC_SLEEP_REJECT_INT_CLR_W<30> {
        SOC_SLEEP_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup_int_clr(&mut self) -> SOC_WAKEUP_INT_CLR_W<31> {
        SOC_WAKEUP_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_int_clr](index.html) module"]
pub struct HP_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hp_int_clr::W](W) writer structure"]
impl crate::Writable for HP_INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_INT_CLR to value 0"]
impl crate::Resettable for HP_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
