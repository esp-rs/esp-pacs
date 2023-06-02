#[doc = "Register `AHB_TEST` reader"]
pub struct R(crate::R<AHB_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_TEST` writer"]
pub struct W(crate::W<AHB_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_TEST_SPEC>;
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
impl From<crate::W<AHB_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHB_TESTMODE` reader - "]
pub type AHB_TESTMODE_R = crate::FieldReader;
#[doc = "Field `AHB_TESTMODE` writer - "]
pub type AHB_TESTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, AHB_TEST_SPEC, 3, O>;
#[doc = "Field `AHB_TESTADDR` reader - "]
pub type AHB_TESTADDR_R = crate::FieldReader;
#[doc = "Field `AHB_TESTADDR` writer - "]
pub type AHB_TESTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, AHB_TEST_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn ahb_testmode(&self) -> AHB_TESTMODE_R {
        AHB_TESTMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ahb_testaddr(&self) -> AHB_TESTADDR_R {
        AHB_TESTADDR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_TEST")
            .field(
                "ahb_testmode",
                &format_args!("{}", self.ahb_testmode().bits()),
            )
            .field(
                "ahb_testaddr",
                &format_args!("{}", self.ahb_testaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_testmode(&mut self) -> AHB_TESTMODE_W<0> {
        AHB_TESTMODE_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_testaddr(&mut self) -> AHB_TESTADDR_W<4> {
        AHB_TESTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_test](index.html) module"]
pub struct AHB_TEST_SPEC;
impl crate::RegisterSpec for AHB_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_test::R](R) reader structure"]
impl crate::Readable for AHB_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_test::W](W) writer structure"]
impl crate::Writable for AHB_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_TEST to value 0"]
impl crate::Resettable for AHB_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
