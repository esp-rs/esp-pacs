#[doc = "Register `INT_ENA_TIMERS` reader"]
pub struct R(crate::R<INT_ENA_TIMERS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_TIMERS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_TIMERS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_TIMERS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_TIMERS` writer"]
pub struct W(crate::W<INT_ENA_TIMERS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_TIMERS_SPEC>;
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
impl From<crate::W<INT_ENA_TIMERS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_TIMERS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_INT_ENA` reader - t0_int_ena"]
pub type T0_INT_ENA_R = crate::BitReader;
#[doc = "Field `T0_INT_ENA` writer - t0_int_ena"]
pub type T0_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_TIMERS_SPEC, O>;
#[doc = "Field `WDT_INT_ENA` reader - wdt_int_ena"]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - wdt_int_ena"]
pub type WDT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_TIMERS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - t0_int_ena"]
    #[inline(always)]
    pub fn t0_int_ena(&self) -> T0_INT_ENA_R {
        T0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wdt_int_ena"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_TIMERS")
            .field("t0_int_ena", &format_args!("{}", self.t0_int_ena().bit()))
            .field("wdt_int_ena", &format_args!("{}", self.wdt_int_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_TIMERS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - t0_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn t0_int_ena(&mut self) -> T0_INT_ENA_W<0> {
        T0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - wdt_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<1> {
        WDT_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT_ENA_TIMG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_timers](index.html) module"]
pub struct INT_ENA_TIMERS_SPEC;
impl crate::RegisterSpec for INT_ENA_TIMERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_timers::R](R) reader structure"]
impl crate::Readable for INT_ENA_TIMERS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_timers::W](W) writer structure"]
impl crate::Writable for INT_ENA_TIMERS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA_TIMERS to value 0"]
impl crate::Resettable for INT_ENA_TIMERS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
