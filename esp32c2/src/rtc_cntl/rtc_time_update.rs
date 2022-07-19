#[doc = "Register `RTC_TIME_UPDATE` reader"]
pub struct R(crate::R<RTC_TIME_UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIME_UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIME_UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME_UPDATE` writer"]
pub struct W(crate::W<RTC_TIME_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_UPDATE_SPEC>;
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
impl From<crate::W<RTC_TIME_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIME_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SYS_STALL` reader - Enable to record system stall time"]
pub type TIMER_SYS_STALL_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_SYS_STALL` writer - Enable to record system stall time"]
pub type TIMER_SYS_STALL_W<'a> = crate::BitWriter<'a, u32, RTC_TIME_UPDATE_SPEC, bool, 27>;
#[doc = "Field `TIMER_XTL_OFF` reader - Enable to record 40M XTAL OFF time"]
pub type TIMER_XTL_OFF_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_XTL_OFF` writer - Enable to record 40M XTAL OFF time"]
pub type TIMER_XTL_OFF_W<'a> = crate::BitWriter<'a, u32, RTC_TIME_UPDATE_SPEC, bool, 28>;
#[doc = "Field `TIMER_SYS_RST` reader - enable to record system reset time"]
pub type TIMER_SYS_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_SYS_RST` writer - enable to record system reset time"]
pub type TIMER_SYS_RST_W<'a> = crate::BitWriter<'a, u32, RTC_TIME_UPDATE_SPEC, bool, 29>;
#[doc = "Field `RTC_TIME_UPDATE` reader - Set 1: to update register with RTC timer"]
pub type RTC_TIME_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TIME_UPDATE` writer - Set 1: to update register with RTC timer"]
pub type RTC_TIME_UPDATE_W<'a> = crate::BitWriter<'a, u32, RTC_TIME_UPDATE_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 27 - Enable to record system stall time"]
    #[inline(always)]
    pub fn timer_sys_stall(&self) -> TIMER_SYS_STALL_R {
        TIMER_SYS_STALL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable to record 40M XTAL OFF time"]
    #[inline(always)]
    pub fn timer_xtl_off(&self) -> TIMER_XTL_OFF_R {
        TIMER_XTL_OFF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - enable to record system reset time"]
    #[inline(always)]
    pub fn timer_sys_rst(&self) -> TIMER_SYS_RST_R {
        TIMER_SYS_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    pub fn rtc_time_update(&self) -> RTC_TIME_UPDATE_R {
        RTC_TIME_UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Enable to record system stall time"]
    #[inline(always)]
    pub fn timer_sys_stall(&mut self) -> TIMER_SYS_STALL_W {
        TIMER_SYS_STALL_W::new(self)
    }
    #[doc = "Bit 28 - Enable to record 40M XTAL OFF time"]
    #[inline(always)]
    pub fn timer_xtl_off(&mut self) -> TIMER_XTL_OFF_W {
        TIMER_XTL_OFF_W::new(self)
    }
    #[doc = "Bit 29 - enable to record system reset time"]
    #[inline(always)]
    pub fn timer_sys_rst(&mut self) -> TIMER_SYS_RST_W {
        TIMER_SYS_RST_W::new(self)
    }
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    pub fn rtc_time_update(&mut self) -> RTC_TIME_UPDATE_W {
        RTC_TIME_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_update](index.html) module"]
pub struct RTC_TIME_UPDATE_SPEC;
impl crate::RegisterSpec for RTC_TIME_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_update::R](R) reader structure"]
impl crate::Readable for RTC_TIME_UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time_update::W](W) writer structure"]
impl crate::Writable for RTC_TIME_UPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIME_UPDATE to value 0"]
impl crate::Resettable for RTC_TIME_UPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
