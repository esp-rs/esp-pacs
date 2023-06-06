#[doc = "Register `TARGET0_HI` reader"]
pub struct R(crate::R<TARGET0_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET0_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET0_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET0_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET0_HI` writer"]
pub struct W(crate::W<TARGET0_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET0_HI_SPEC>;
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
impl From<crate::W<TARGET0_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET0_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TARGET0_HI` reader - System timer target 0, high 32 bits."]
pub type TIMER_TARGET0_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET0_HI` writer - System timer target 0, high 32 bits."]
pub type TIMER_TARGET0_HI_W<'a, const O: u8> = crate::FieldWriter<'a, TARGET0_HI_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - System timer target 0, high 32 bits."]
    #[inline(always)]
    pub fn timer_target0_hi(&self) -> TIMER_TARGET0_HI_R {
        TIMER_TARGET0_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET0_HI")
            .field(
                "timer_target0_hi",
                &format_args!("{}", self.timer_target0_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET0_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - System timer target 0, high 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn timer_target0_hi(&mut self) -> TIMER_TARGET0_HI_W<0> {
        TIMER_TARGET0_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System timer target 0, high 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target0_hi](index.html) module"]
pub struct TARGET0_HI_SPEC;
impl crate::RegisterSpec for TARGET0_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target0_hi::R](R) reader structure"]
impl crate::Readable for TARGET0_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target0_hi::W](W) writer structure"]
impl crate::Writable for TARGET0_HI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARGET0_HI to value 0"]
impl crate::Resettable for TARGET0_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
