#[doc = "Register `DEBNCE` reader"]
pub struct R(crate::R<DEBNCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBNCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBNCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBNCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBNCE` writer"]
pub struct W(crate::W<DEBNCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBNCE_SPEC>;
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
impl From<crate::W<DEBNCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBNCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBOUNCE_COUNT` reader - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
pub type DEBOUNCE_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DEBOUNCE_COUNT` writer - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
pub type DEBOUNCE_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, DEBNCE_SPEC, 24, O, u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
    #[inline(always)]
    pub fn debounce_count(&self) -> DEBOUNCE_COUNT_R {
        DEBOUNCE_COUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBNCE")
            .field(
                "debounce_count",
                &format_args!("{}", self.debounce_count().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEBNCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
    #[inline(always)]
    #[must_use]
    pub fn debounce_count(&mut self) -> DEBOUNCE_COUNT_W<0> {
        DEBOUNCE_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debounce filter time configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debnce](index.html) module"]
pub struct DEBNCE_SPEC;
impl crate::RegisterSpec for DEBNCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debnce::R](R) reader structure"]
impl crate::Readable for DEBNCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debnce::W](W) writer structure"]
impl crate::Writable for DEBNCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBNCE to value 0"]
impl crate::Resettable for DEBNCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
