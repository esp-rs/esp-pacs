#[doc = "Register `TAR1_LOW` reader"]
pub struct R(crate::R<TAR1_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAR1_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAR1_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAR1_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAR1_LOW` writer"]
pub struct W(crate::W<TAR1_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAR1_LOW_SPEC>;
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
impl From<crate::W<TAR1_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAR1_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_TIMER_TAR_LOW1` reader - need_des"]
pub type MAIN_TIMER_TAR_LOW1_R = crate::FieldReader<u32>;
#[doc = "Field `MAIN_TIMER_TAR_LOW1` writer - need_des"]
pub type MAIN_TIMER_TAR_LOW1_W<'a, const O: u8> = crate::FieldWriter<'a, TAR1_LOW_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_low1(&self) -> MAIN_TIMER_TAR_LOW1_R {
        MAIN_TIMER_TAR_LOW1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAR1_LOW")
            .field(
                "main_timer_tar_low1",
                &format_args!("{}", self.main_timer_tar_low1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TAR1_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_tar_low1(&mut self) -> MAIN_TIMER_TAR_LOW1_W<0> {
        MAIN_TIMER_TAR_LOW1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tar1_low](index.html) module"]
pub struct TAR1_LOW_SPEC;
impl crate::RegisterSpec for TAR1_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tar1_low::R](R) reader structure"]
impl crate::Readable for TAR1_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tar1_low::W](W) writer structure"]
impl crate::Writable for TAR1_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAR1_LOW to value 0"]
impl crate::Resettable for TAR1_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
