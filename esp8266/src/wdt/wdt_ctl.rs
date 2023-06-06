#[doc = "Register `WDT_CTL` reader"]
pub struct R(crate::R<WDT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CTL` writer"]
pub struct W(crate::W<WDT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CTL_SPEC>;
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
impl From<crate::W<WDT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> = crate::FieldWriter<'a, WDT_CTL_SPEC, 32, O, u32>;
#[doc = "Field `enable` reader - Enable the watchdog timer."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `enable` writer - Enable the watchdog timer."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CTL_SPEC, O>;
#[doc = "Field `stage_1_no_reset` reader - When set to 1, and running in two-stage mode, it turns the watchdog into a single shot timer that doesn't reset the device."]
pub type STAGE_1_NO_RESET_R = crate::BitReader;
#[doc = "Field `stage_1_no_reset` writer - When set to 1, and running in two-stage mode, it turns the watchdog into a single shot timer that doesn't reset the device."]
pub type STAGE_1_NO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CTL_SPEC, O>;
#[doc = "Field `stage_1_disable` reader - Set to 1 to disable the stage 1 of the watchdog timer"]
pub type STAGE_1_DISABLE_R = crate::BitReader;
#[doc = "Field `stage_1_disable` writer - Set to 1 to disable the stage 1 of the watchdog timer"]
pub type STAGE_1_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CTL_SPEC, O>;
#[doc = "Field `unknown_3` reader - "]
pub type UNKNOWN_3_R = crate::BitReader;
#[doc = "Field `unknown_3` writer - "]
pub type UNKNOWN_3_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CTL_SPEC, O>;
#[doc = "Field `unknown_4` reader - "]
pub type UNKNOWN_4_R = crate::BitReader;
#[doc = "Field `unknown_4` writer - "]
pub type UNKNOWN_4_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CTL_SPEC, O>;
#[doc = "Field `unknown_5` reader - "]
pub type UNKNOWN_5_R = crate::BitReader;
#[doc = "Field `unknown_5` writer - "]
pub type UNKNOWN_5_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CTL_SPEC, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
    #[doc = "Bit 0 - Enable the watchdog timer."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, and running in two-stage mode, it turns the watchdog into a single shot timer that doesn't reset the device."]
    #[inline(always)]
    pub fn stage_1_no_reset(&self) -> STAGE_1_NO_RESET_R {
        STAGE_1_NO_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to 1 to disable the stage 1 of the watchdog timer"]
    #[inline(always)]
    pub fn stage_1_disable(&self) -> STAGE_1_DISABLE_R {
        STAGE_1_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn unknown_3(&self) -> UNKNOWN_3_R {
        UNKNOWN_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn unknown_4(&self) -> UNKNOWN_4_R {
        UNKNOWN_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn unknown_5(&self) -> UNKNOWN_5_R {
        UNKNOWN_5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_CTL")
            .field("register", &format_args!("{}", self.register().bits()))
            .field("enable", &format_args!("{}", self.enable().bit()))
            .field(
                "stage_1_no_reset",
                &format_args!("{}", self.stage_1_no_reset().bit()),
            )
            .field(
                "stage_1_disable",
                &format_args!("{}", self.stage_1_disable().bit()),
            )
            .field("unknown_3", &format_args!("{}", self.unknown_3().bit()))
            .field("unknown_4", &format_args!("{}", self.unknown_4().bit()))
            .field("unknown_5", &format_args!("{}", self.unknown_5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDT_CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<0> {
        REGISTER_W::new(self)
    }
    #[doc = "Bit 0 - Enable the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - When set to 1, and running in two-stage mode, it turns the watchdog into a single shot timer that doesn't reset the device."]
    #[inline(always)]
    #[must_use]
    pub fn stage_1_no_reset(&mut self) -> STAGE_1_NO_RESET_W<1> {
        STAGE_1_NO_RESET_W::new(self)
    }
    #[doc = "Bit 2 - Set to 1 to disable the stage 1 of the watchdog timer"]
    #[inline(always)]
    #[must_use]
    pub fn stage_1_disable(&mut self) -> STAGE_1_DISABLE_W<2> {
        STAGE_1_DISABLE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn unknown_3(&mut self) -> UNKNOWN_3_W<3> {
        UNKNOWN_3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn unknown_4(&mut self) -> UNKNOWN_4_W<4> {
        UNKNOWN_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn unknown_5(&mut self) -> UNKNOWN_5_W<5> {
        UNKNOWN_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT_CTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_ctl](index.html) module"]
pub struct WDT_CTL_SPEC;
impl crate::RegisterSpec for WDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_ctl::R](R) reader structure"]
impl crate::Readable for WDT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_ctl::W](W) writer structure"]
impl crate::Writable for WDT_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_CTL to value 0"]
impl crate::Resettable for WDT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
