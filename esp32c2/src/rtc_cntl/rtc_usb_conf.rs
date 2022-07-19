#[doc = "Register `RTC_USB_CONF` reader"]
pub struct R(crate::R<RTC_USB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_USB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_USB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_USB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_USB_CONF` writer"]
pub struct W(crate::W<RTC_USB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_USB_CONF_SPEC>;
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
impl From<crate::W<RTC_USB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_USB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - Need add desc"]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - Need add desc"]
pub type IO_MUX_RESET_DISABLE_W<'a> = crate::BitWriter<'a, u32, RTC_USB_CONF_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W {
        IO_MUX_RESET_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_usb_conf](index.html) module"]
pub struct RTC_USB_CONF_SPEC;
impl crate::RegisterSpec for RTC_USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_usb_conf::R](R) reader structure"]
impl crate::Readable for RTC_USB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_usb_conf::W](W) writer structure"]
impl crate::Writable for RTC_USB_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_USB_CONF to value 0"]
impl crate::Resettable for RTC_USB_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
