#[doc = "Register `ULP_CP_SLEEP_CYC3` reader"]
pub struct R(crate::R<ULP_CP_SLEEP_CYC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ULP_CP_SLEEP_CYC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ULP_CP_SLEEP_CYC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ULP_CP_SLEEP_CYC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ULP_CP_SLEEP_CYC3` writer"]
pub struct W(crate::W<ULP_CP_SLEEP_CYC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ULP_CP_SLEEP_CYC3_SPEC>;
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
impl From<crate::W<ULP_CP_SLEEP_CYC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ULP_CP_SLEEP_CYC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_CYCLES_S3` reader - "]
pub type SLEEP_CYCLES_S3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLEEP_CYCLES_S3` writer - "]
pub type SLEEP_CYCLES_S3_W<'a, const O: u8> =
    crate::FieldWriter<'a, ULP_CP_SLEEP_CYC3_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s3(&self) -> SLEEP_CYCLES_S3_R {
        SLEEP_CYCLES_S3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_SLEEP_CYC3")
            .field(
                "sleep_cycles_s3",
                &format_args!("{}", self.sleep_cycles_s3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_SLEEP_CYC3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_cycles_s3(&mut self) -> SLEEP_CYCLES_S3_W<0> {
        SLEEP_CYCLES_S3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulp_cp_sleep_cyc3](index.html) module"]
pub struct ULP_CP_SLEEP_CYC3_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ulp_cp_sleep_cyc3::R](R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ulp_cp_sleep_cyc3::W](W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ULP_CP_SLEEP_CYC3 to value 0x28"]
impl crate::Resettable for ULP_CP_SLEEP_CYC3_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
