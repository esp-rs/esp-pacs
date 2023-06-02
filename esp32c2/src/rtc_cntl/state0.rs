#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE0` writer"]
pub struct W(crate::W<STATE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE0_SPEC>;
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
impl From<crate::W<STATE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_CPU_INT` reader - rtc software interrupt to main cpu"]
pub type SW_CPU_INT_R = crate::BitReader;
#[doc = "Field `SW_CPU_INT` writer - rtc software interrupt to main cpu"]
pub type SW_CPU_INT_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
#[doc = "Field `SLP_REJECT_CAUSE_CLR` reader - clear rtc sleep reject cause"]
pub type SLP_REJECT_CAUSE_CLR_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_CAUSE_CLR` writer - clear rtc sleep reject cause"]
pub type SLP_REJECT_CAUSE_CLR_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
#[doc = "Field `APB2RTC_BRIDGE_SEL` reader - 1: APB to RTC using bridge"]
pub type APB2RTC_BRIDGE_SEL_R = crate::BitReader;
#[doc = "Field `APB2RTC_BRIDGE_SEL` writer - 1: APB to RTC using bridge"]
pub type APB2RTC_BRIDGE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
#[doc = "Field `SDIO_ACTIVE_IND` reader - SDIO active indication"]
pub type SDIO_ACTIVE_IND_R = crate::BitReader;
#[doc = "Field `SDIO_ACTIVE_IND` writer - SDIO active indication"]
pub type SDIO_ACTIVE_IND_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
#[doc = "Field `SLP_WAKEUP` reader - leep wakeup bit"]
pub type SLP_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP` writer - leep wakeup bit"]
pub type SLP_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
#[doc = "Field `SLP_REJECT` reader - leep reject bit"]
pub type SLP_REJECT_R = crate::BitReader;
#[doc = "Field `SLP_REJECT` writer - leep reject bit"]
pub type SLP_REJECT_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
#[doc = "Field `SLEEP_EN` reader - sleep enable bit"]
pub type SLEEP_EN_R = crate::BitReader;
#[doc = "Field `SLEEP_EN` writer - sleep enable bit"]
pub type SLEEP_EN_W<'a, const O: u8> = crate::BitWriter<'a, STATE0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - rtc software interrupt to main cpu"]
    #[inline(always)]
    pub fn sw_cpu_int(&self) -> SW_CPU_INT_R {
        SW_CPU_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clear rtc sleep reject cause"]
    #[inline(always)]
    pub fn slp_reject_cause_clr(&self) -> SLP_REJECT_CAUSE_CLR_R {
        SLP_REJECT_CAUSE_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - leep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - leep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE0")
            .field("sw_cpu_int", &format_args!("{}", self.sw_cpu_int().bit()))
            .field(
                "slp_reject_cause_clr",
                &format_args!("{}", self.slp_reject_cause_clr().bit()),
            )
            .field(
                "apb2rtc_bridge_sel",
                &format_args!("{}", self.apb2rtc_bridge_sel().bit()),
            )
            .field(
                "sdio_active_ind",
                &format_args!("{}", self.sdio_active_ind().bit()),
            )
            .field("slp_wakeup", &format_args!("{}", self.slp_wakeup().bit()))
            .field("slp_reject", &format_args!("{}", self.slp_reject().bit()))
            .field("sleep_en", &format_args!("{}", self.sleep_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - rtc software interrupt to main cpu"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cpu_int(&mut self) -> SW_CPU_INT_W<0> {
        SW_CPU_INT_W::new(self)
    }
    #[doc = "Bit 1 - clear rtc sleep reject cause"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_cause_clr(&mut self) -> SLP_REJECT_CAUSE_CLR_W<1> {
        SLP_REJECT_CAUSE_CLR_W::new(self)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge"]
    #[inline(always)]
    #[must_use]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W<22> {
        APB2RTC_BRIDGE_SEL_W::new(self)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_active_ind(&mut self) -> SDIO_ACTIVE_IND_W<28> {
        SDIO_ACTIVE_IND_W::new(self)
    }
    #[doc = "Bit 29 - leep wakeup bit"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W<29> {
        SLP_WAKEUP_W::new(self)
    }
    #[doc = "Bit 30 - leep reject bit"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W<30> {
        SLP_REJECT_W::new(self)
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W<31> {
        SLEEP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state0::W](W) writer structure"]
impl crate::Writable for STATE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
