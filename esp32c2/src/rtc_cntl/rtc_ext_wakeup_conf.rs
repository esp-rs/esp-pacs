#[doc = "Register `RTC_EXT_WAKEUP_CONF` reader"]
pub struct R(crate::R<RTC_EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_EXT_WAKEUP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_EXT_WAKEUP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_EXT_WAKEUP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_EXT_WAKEUP_CONF` writer"]
pub struct W(crate::W<RTC_EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_EXT_WAKEUP_CONF_SPEC>;
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
impl From<crate::W<RTC_EXT_WAKEUP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_EXT_WAKEUP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_W<'a> = crate::BitWriter<'a, u32, RTC_EXT_WAKEUP_CONF_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 31 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W {
        GPIO_WAKEUP_FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ext_wakeup_conf](index.html) module"]
pub struct RTC_EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for RTC_EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ext_wakeup_conf::R](R) reader structure"]
impl crate::Readable for RTC_EXT_WAKEUP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ext_wakeup_conf::W](W) writer structure"]
impl crate::Writable for RTC_EXT_WAKEUP_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for RTC_EXT_WAKEUP_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
