#[doc = "Register `FSM` reader"]
pub struct R(crate::R<FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM` writer"]
pub struct W(crate::W<FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SPEC>;
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
impl From<crate::W<FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE_NUM` reader - sample number"]
pub type SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAMPLE_NUM` writer - sample number"]
pub type SAMPLE_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_SPEC, 8, O>;
#[doc = "Field `SAMPLE_CYCLE` reader - sample cycles"]
pub type SAMPLE_CYCLE_R = crate::FieldReader;
#[doc = "Field `SAMPLE_CYCLE` writer - sample cycles"]
pub type SAMPLE_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, FSM_SPEC, 8, O>;
impl R {
    #[doc = "Bits 16:23 - sample number"]
    #[inline(always)]
    pub fn sample_num(&self) -> SAMPLE_NUM_R {
        SAMPLE_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn sample_cycle(&self) -> SAMPLE_CYCLE_R {
        SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("sample_num", &format_args!("{}", self.sample_num().bits()))
            .field(
                "sample_cycle",
                &format_args!("{}", self.sample_cycle().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:23 - sample number"]
    #[inline(always)]
    #[must_use]
    pub fn sample_num(&mut self) -> SAMPLE_NUM_W<16> {
        SAMPLE_NUM_W::new(self)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    #[must_use]
    pub fn sample_cycle(&mut self) -> SAMPLE_CYCLE_W<24> {
        SAMPLE_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital adc control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm](index.html) module"]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm::R](R) reader structure"]
impl crate::Readable for FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm::W](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM to value 0x0200_0000"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
