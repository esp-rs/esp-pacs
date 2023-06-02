#[doc = "Register `UNIT1_OP` reader"]
pub struct R(crate::R<UNIT1_OP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT1_OP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT1_OP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT1_OP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNIT1_OP` writer"]
pub struct W(crate::W<UNIT1_OP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNIT1_OP_SPEC>;
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
impl From<crate::W<UNIT1_OP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNIT1_OP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT1_VALUE_VALID` reader - timer value is sync and valid"]
pub type TIMER_UNIT1_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_UPDATE` writer - update timer unit1"]
pub type TIMER_UNIT1_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, UNIT1_OP_SPEC, O>;
impl R {
    #[doc = "Bit 29 - timer value is sync and valid"]
    #[inline(always)]
    pub fn timer_unit1_value_valid(&self) -> TIMER_UNIT1_VALUE_VALID_R {
        TIMER_UNIT1_VALUE_VALID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_OP")
            .field(
                "timer_unit1_value_valid",
                &format_args!("{}", self.timer_unit1_value_valid().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_OP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - update timer unit1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_update(&mut self) -> TIMER_UNIT1_UPDATE_W<30> {
        TIMER_UNIT1_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTIMER_UNIT1_OP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit1_op](index.html) module"]
pub struct UNIT1_OP_SPEC;
impl crate::RegisterSpec for UNIT1_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit1_op::R](R) reader structure"]
impl crate::Readable for UNIT1_OP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unit1_op::W](W) writer structure"]
impl crate::Writable for UNIT1_OP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNIT1_OP to value 0"]
impl crate::Resettable for UNIT1_OP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
