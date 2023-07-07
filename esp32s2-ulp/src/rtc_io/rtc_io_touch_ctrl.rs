#[doc = "Register `RTC_IO_TOUCH_CTRL` reader"]
pub struct R(crate::R<RTC_IO_TOUCH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_IO_TOUCH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_IO_TOUCH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_IO_TOUCH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_IO_TOUCH_CTRL` writer"]
pub struct W(crate::W<RTC_IO_TOUCH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_IO_TOUCH_CTRL_SPEC>;
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
impl From<crate::W<RTC_IO_TOUCH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_IO_TOUCH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_TOUCH_BUFSEL` reader - "]
pub type IO_TOUCH_BUFSEL_R = crate::FieldReader;
#[doc = "Field `IO_TOUCH_BUFSEL` writer - "]
pub type IO_TOUCH_BUFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_IO_TOUCH_CTRL_SPEC, 4, O>;
#[doc = "Field `IO_TOUCH_BUFMODE` reader - "]
pub type IO_TOUCH_BUFMODE_R = crate::BitReader;
#[doc = "Field `IO_TOUCH_BUFMODE` writer - "]
pub type IO_TOUCH_BUFMODE_W<'a, const O: u8> = crate::BitWriter<'a, RTC_IO_TOUCH_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io_touch_bufsel(&self) -> IO_TOUCH_BUFSEL_R {
        IO_TOUCH_BUFSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn io_touch_bufmode(&self) -> IO_TOUCH_BUFMODE_R {
        IO_TOUCH_BUFMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO_TOUCH_CTRL")
            .field(
                "io_touch_bufsel",
                &format_args!("{}", self.io_touch_bufsel().bits()),
            )
            .field(
                "io_touch_bufmode",
                &format_args!("{}", self.io_touch_bufmode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_IO_TOUCH_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn io_touch_bufsel(&mut self) -> IO_TOUCH_BUFSEL_W<0> {
        IO_TOUCH_BUFSEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn io_touch_bufmode(&mut self) -> IO_TOUCH_BUFMODE_W<4> {
        IO_TOUCH_BUFMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Touch control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_io_touch_ctrl](index.html) module"]
pub struct RTC_IO_TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for RTC_IO_TOUCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_io_touch_ctrl::R](R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_io_touch_ctrl::W](W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_IO_TOUCH_CTRL to value 0"]
impl crate::Resettable for RTC_IO_TOUCH_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
