#[doc = "Register `CAP_TIMER_PHASE` reader"]
pub struct R(crate::R<CAP_TIMER_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_TIMER_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_TIMER_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_TIMER_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_TIMER_PHASE` writer"]
pub struct W(crate::W<CAP_TIMER_PHASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_TIMER_PHASE_SPEC>;
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
impl From<crate::W<CAP_TIMER_PHASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_TIMER_PHASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_PHASE` reader - Phase value for capture timer sync operation."]
pub type CAP_PHASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAP_PHASE` writer - Phase value for capture timer sync operation."]
pub type CAP_PHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CAP_TIMER_PHASE_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    pub fn cap_phase(&self) -> CAP_PHASE_R {
        CAP_PHASE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_PHASE")
            .field("cap_phase", &format_args!("{}", self.cap_phase().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_TIMER_PHASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Phase value for capture timer sync operation."]
    #[inline(always)]
    #[must_use]
    pub fn cap_phase(&mut self) -> CAP_PHASE_W<0> {
        CAP_PHASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase for capture timer sync\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_timer_phase](index.html) module"]
pub struct CAP_TIMER_PHASE_SPEC;
impl crate::RegisterSpec for CAP_TIMER_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_timer_phase::R](R) reader structure"]
impl crate::Readable for CAP_TIMER_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_timer_phase::W](W) writer structure"]
impl crate::Writable for CAP_TIMER_PHASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_TIMER_PHASE to value 0"]
impl crate::Resettable for CAP_TIMER_PHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
