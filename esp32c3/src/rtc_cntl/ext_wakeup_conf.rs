#[doc = "Register `EXT_WAKEUP_CONF` reader"]
pub struct R(crate::R<EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP_CONF` writer"]
pub struct W(crate::W<EXT_WAKEUP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP_CONF_SPEC>;
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
impl From<crate::W<EXT_WAKEUP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_WAKEUP_FILTER` reader - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_R = crate::BitReader;
#[doc = "Field `GPIO_WAKEUP_FILTER` writer - enable filter for gpio wakeup event"]
pub type GPIO_WAKEUP_FILTER_W<'a, const O: u8> = crate::BitWriter<'a, EXT_WAKEUP_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 31 - enable filter for gpio wakeup event"]
    #[inline(always)]
    pub fn gpio_wakeup_filter(&self) -> GPIO_WAKEUP_FILTER_R {
        GPIO_WAKEUP_FILTER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CONF")
            .field(
                "gpio_wakeup_filter",
                &format_args!("{}", self.gpio_wakeup_filter().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - enable filter for gpio wakeup event"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_filter(&mut self) -> GPIO_WAKEUP_FILTER_W<31> {
        GPIO_WAKEUP_FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup_conf](index.html) module"]
pub struct EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup_conf::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup_conf::W](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for EXT_WAKEUP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
