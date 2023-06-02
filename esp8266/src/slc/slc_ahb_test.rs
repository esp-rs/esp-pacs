#[doc = "Register `SLC_AHB_TEST` reader"]
pub struct R(crate::R<SLC_AHB_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_AHB_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_AHB_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_AHB_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_AHB_TEST` writer"]
pub struct W(crate::W<SLC_AHB_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_AHB_TEST_SPEC>;
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
impl From<crate::W<SLC_AHB_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_AHB_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_AHB_TESTMODE` reader - "]
pub type SLC_AHB_TESTMODE_R = crate::FieldReader;
#[doc = "Field `SLC_AHB_TESTMODE` writer - "]
pub type SLC_AHB_TESTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_AHB_TEST_SPEC, 3, O>;
#[doc = "Field `SLC_AHB_TESTADDR` reader - "]
pub type SLC_AHB_TESTADDR_R = crate::FieldReader;
#[doc = "Field `SLC_AHB_TESTADDR` writer - "]
pub type SLC_AHB_TESTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_AHB_TEST_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn slc_ahb_testmode(&self) -> SLC_AHB_TESTMODE_R {
        SLC_AHB_TESTMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn slc_ahb_testaddr(&self) -> SLC_AHB_TESTADDR_R {
        SLC_AHB_TESTADDR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_AHB_TEST")
            .field(
                "slc_ahb_testaddr",
                &format_args!("{}", self.slc_ahb_testaddr().bits()),
            )
            .field(
                "slc_ahb_testmode",
                &format_args!("{}", self.slc_ahb_testmode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_AHB_TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn slc_ahb_testmode(&mut self) -> SLC_AHB_TESTMODE_W<0> {
        SLC_AHB_TESTMODE_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn slc_ahb_testaddr(&mut self) -> SLC_AHB_TESTADDR_W<4> {
        SLC_AHB_TESTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_AHB_TEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_ahb_test](index.html) module"]
pub struct SLC_AHB_TEST_SPEC;
impl crate::RegisterSpec for SLC_AHB_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_ahb_test::R](R) reader structure"]
impl crate::Readable for SLC_AHB_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_ahb_test::W](W) writer structure"]
impl crate::Writable for SLC_AHB_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_AHB_TEST to value 0"]
impl crate::Resettable for SLC_AHB_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
