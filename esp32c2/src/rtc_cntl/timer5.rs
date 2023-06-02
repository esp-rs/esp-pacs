#[doc = "Register `TIMER5` reader"]
pub struct R(crate::R<TIMER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER5` writer"]
pub struct W(crate::W<TIMER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER5_SPEC>;
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
impl From<crate::W<TIMER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_SLP_VAL` reader - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `MIN_SLP_VAL` writer - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER5_SPEC, 8, O>;
impl R {
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER5")
            .field(
                "min_slp_val",
                &format_args!("{}", self.min_slp_val().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W<8> {
        MIN_SLP_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer5](index.html) module"]
pub struct TIMER5_SPEC;
impl crate::RegisterSpec for TIMER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer5::R](R) reader structure"]
impl crate::Readable for TIMER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer5::W](W) writer structure"]
impl crate::Writable for TIMER5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER5 to value 0x8000"]
impl crate::Resettable for TIMER5_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
