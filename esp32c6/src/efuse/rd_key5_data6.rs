#[doc = "Register `RD_KEY5_DATA6` reader"]
pub struct R(crate::R<RD_KEY5_DATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY5_DATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY5_DATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY5_DATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY5_DATA6` reader - Stores the sixth 32 bits of KEY5."]
pub type KEY5_DATA6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the sixth 32 bits of KEY5."]
    #[inline(always)]
    pub fn key5_data6(&self) -> KEY5_DATA6_R {
        KEY5_DATA6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY5_DATA6")
            .field("key5_data6", &format_args!("{}", self.key5_data6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY5_DATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register $n of BLOCK9 (KEY5).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data6](index.html) module"]
pub struct RD_KEY5_DATA6_SPEC;
impl crate::RegisterSpec for RD_KEY5_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key5_data6::R](R) reader structure"]
impl crate::Readable for RD_KEY5_DATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY5_DATA6 to value 0"]
impl crate::Resettable for RD_KEY5_DATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
