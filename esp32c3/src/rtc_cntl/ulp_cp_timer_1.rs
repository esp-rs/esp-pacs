#[doc = "Register `ULP_CP_TIMER_1` reader"]
pub struct R(crate::R<ULP_CP_TIMER_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_TIMER_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_TIMER_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_TIMER_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_TIMER_1` writer"]
pub struct W(crate::W<ULP_CP_TIMER_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_TIMER_1_SPEC>;
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
impl From<crate::W<ULP_CP_TIMER_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_TIMER_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` reader - sleep cycles for ULP-coprocessor timer"]
pub type ULP_CP_TIMER_SLP_CYCLE_R = crate::FieldReader<u32>;
#[doc = "Field `ULP_CP_TIMER_SLP_CYCLE` writer - sleep cycles for ULP-coprocessor timer"]
pub type ULP_CP_TIMER_SLP_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, ULP_CP_TIMER_1_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    pub fn ulp_cp_timer_slp_cycle(&self) -> ULP_CP_TIMER_SLP_CYCLE_R {
        ULP_CP_TIMER_SLP_CYCLE_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_TIMER_1")
            .field(
                "ulp_cp_timer_slp_cycle",
                &format_args!("{}", self.ulp_cp_timer_slp_cycle().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_TIMER_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:31 - sleep cycles for ULP-coprocessor timer"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_timer_slp_cycle(&mut self) -> ULP_CP_TIMER_SLP_CYCLE_W<8> {
        ULP_CP_TIMER_SLP_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_timer_1](index.html) module"]
pub struct ULP_CP_TIMER_1_SPEC;
impl crate::RegisterSpec for ULP_CP_TIMER_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_timer_1::R](R) reader structure"]
impl crate::Readable for ULP_CP_TIMER_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_timer_1::W](W) writer structure"]
impl crate::Writable for ULP_CP_TIMER_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ULP_CP_TIMER_1 to value 0xc800"]
impl crate::Resettable for ULP_CP_TIMER_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xc800;
}
