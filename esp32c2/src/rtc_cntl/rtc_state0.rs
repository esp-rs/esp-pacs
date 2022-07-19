#[doc = "Register `RTC_STATE0` reader"]
pub struct R(crate::R<RTC_STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_STATE0` writer"]
pub struct W(crate::W<RTC_STATE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_STATE0_SPEC>;
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
impl From<crate::W<RTC_STATE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_STATE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_SW_CPU_INT` reader - rtc software interrupt to main cpu"]
pub type RTC_SW_CPU_INT_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SW_CPU_INT` writer - rtc software interrupt to main cpu"]
pub type RTC_SW_CPU_INT_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 0>;
#[doc = "Field `RTC_SLP_REJECT_CAUSE_CLR` reader - clear rtc sleep reject cause"]
pub type RTC_SLP_REJECT_CAUSE_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLP_REJECT_CAUSE_CLR` writer - clear rtc sleep reject cause"]
pub type RTC_SLP_REJECT_CAUSE_CLR_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 1>;
#[doc = "Field `APB2RTC_BRIDGE_SEL` reader - 1: APB to RTC using bridge"]
pub type APB2RTC_BRIDGE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `APB2RTC_BRIDGE_SEL` writer - 1: APB to RTC using bridge"]
pub type APB2RTC_BRIDGE_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 22>;
#[doc = "Field `SDIO_ACTIVE_IND` reader - SDIO active indication"]
pub type SDIO_ACTIVE_IND_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_ACTIVE_IND` writer - SDIO active indication"]
pub type SDIO_ACTIVE_IND_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 28>;
#[doc = "Field `SLP_WAKEUP` reader - leep wakeup bit"]
pub type SLP_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `SLP_WAKEUP` writer - leep wakeup bit"]
pub type SLP_WAKEUP_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 29>;
#[doc = "Field `SLP_REJECT` reader - leep reject bit"]
pub type SLP_REJECT_R = crate::BitReader<bool>;
#[doc = "Field `SLP_REJECT` writer - leep reject bit"]
pub type SLP_REJECT_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 30>;
#[doc = "Field `SLEEP_EN` reader - sleep enable bit"]
pub type SLEEP_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_EN` writer - sleep enable bit"]
pub type SLEEP_EN_W<'a> = crate::BitWriter<'a, u32, RTC_STATE0_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - rtc software interrupt to main cpu"]
    #[inline(always)]
    pub fn rtc_sw_cpu_int(&self) -> RTC_SW_CPU_INT_R {
        RTC_SW_CPU_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clear rtc sleep reject cause"]
    #[inline(always)]
    pub fn rtc_slp_reject_cause_clr(&self) -> RTC_SLP_REJECT_CAUSE_CLR_R {
        RTC_SLP_REJECT_CAUSE_CLR_R::new(((self.bits >> 1) & 1) != 0)
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
impl W {
    #[doc = "Bit 0 - rtc software interrupt to main cpu"]
    #[inline(always)]
    pub fn rtc_sw_cpu_int(&mut self) -> RTC_SW_CPU_INT_W {
        RTC_SW_CPU_INT_W::new(self)
    }
    #[doc = "Bit 1 - clear rtc sleep reject cause"]
    #[inline(always)]
    pub fn rtc_slp_reject_cause_clr(&mut self) -> RTC_SLP_REJECT_CAUSE_CLR_W {
        RTC_SLP_REJECT_CAUSE_CLR_W::new(self)
    }
    #[doc = "Bit 22 - 1: APB to RTC using bridge"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W {
        APB2RTC_BRIDGE_SEL_W::new(self)
    }
    #[doc = "Bit 28 - SDIO active indication"]
    #[inline(always)]
    pub fn sdio_active_ind(&mut self) -> SDIO_ACTIVE_IND_W {
        SDIO_ACTIVE_IND_W::new(self)
    }
    #[doc = "Bit 29 - leep wakeup bit"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W {
        SLP_WAKEUP_W::new(self)
    }
    #[doc = "Bit 30 - leep reject bit"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W {
        SLP_REJECT_W::new(self)
    }
    #[doc = "Bit 31 - sleep enable bit"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W {
        SLEEP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_state0](index.html) module"]
pub struct RTC_STATE0_SPEC;
impl crate::RegisterSpec for RTC_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_state0::R](R) reader structure"]
impl crate::Readable for RTC_STATE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_state0::W](W) writer structure"]
impl crate::Writable for RTC_STATE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_STATE0 to value 0"]
impl crate::Resettable for RTC_STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
