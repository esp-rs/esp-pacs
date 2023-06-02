#[doc = "Register `EDGE_INT_ENABLE` reader"]
pub struct R(crate::R<EDGE_INT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDGE_INT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDGE_INT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDGE_INT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDGE_INT_ENABLE` writer"]
pub struct W(crate::W<EDGE_INT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDGE_INT_ENABLE_SPEC>;
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
impl From<crate::W<EDGE_INT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDGE_INT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, EDGE_INT_ENABLE_SPEC, 32, O, u32, u32>;
#[doc = "Field `wdt_edge_int_enable` reader - Enable the watchdog timer edge interrupt"]
pub type WDT_EDGE_INT_ENABLE_R = crate::BitReader;
#[doc = "Field `wdt_edge_int_enable` writer - Enable the watchdog timer edge interrupt"]
pub type WDT_EDGE_INT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, EDGE_INT_ENABLE_SPEC, O>;
#[doc = "Field `timer1_edge_int_enable` reader - Enable the timer1 edge interrupt"]
pub type TIMER1_EDGE_INT_ENABLE_R = crate::BitReader;
#[doc = "Field `timer1_edge_int_enable` writer - Enable the timer1 edge interrupt"]
pub type TIMER1_EDGE_INT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, EDGE_INT_ENABLE_SPEC, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
    #[doc = "Bit 0 - Enable the watchdog timer edge interrupt"]
    #[inline(always)]
    pub fn wdt_edge_int_enable(&self) -> WDT_EDGE_INT_ENABLE_R {
        WDT_EDGE_INT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the timer1 edge interrupt"]
    #[inline(always)]
    pub fn timer1_edge_int_enable(&self) -> TIMER1_EDGE_INT_ENABLE_R {
        TIMER1_EDGE_INT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDGE_INT_ENABLE")
            .field("register", &format_args!("{}", self.register().bits()))
            .field(
                "wdt_edge_int_enable",
                &format_args!("{}", self.wdt_edge_int_enable().bit()),
            )
            .field(
                "timer1_edge_int_enable",
                &format_args!("{}", self.timer1_edge_int_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDGE_INT_ENABLE_SPEC> {
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
    #[doc = "Bit 0 - Enable the watchdog timer edge interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_edge_int_enable(&mut self) -> WDT_EDGE_INT_ENABLE_W<0> {
        WDT_EDGE_INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable the timer1 edge interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_edge_int_enable(&mut self) -> TIMER1_EDGE_INT_ENABLE_W<1> {
        TIMER1_EDGE_INT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDGE_INT_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edge_int_enable](index.html) module"]
pub struct EDGE_INT_ENABLE_SPEC;
impl crate::RegisterSpec for EDGE_INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edge_int_enable::R](R) reader structure"]
impl crate::Readable for EDGE_INT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edge_int_enable::W](W) writer structure"]
impl crate::Writable for EDGE_INT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDGE_INT_ENABLE to value 0"]
impl crate::Resettable for EDGE_INT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
