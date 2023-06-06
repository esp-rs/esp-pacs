#[doc = "Register `RD_KEY0_DATA7` reader"]
pub struct R(crate::R<RD_KEY0_DATA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY0_DATA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY0_DATA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY0_DATA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY0_DATA7` reader - Stores the seventh 32 bits of KEY0."]
pub type KEY0_DATA7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the seventh 32 bits of KEY0."]
    #[inline(always)]
    pub fn key0_data7(&self) -> KEY0_DATA7_R {
        KEY0_DATA7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY0_DATA7")
            .field("key0_data7", &format_args!("{}", self.key0_data7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY0_DATA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 7 of BLOCK4 (KEY0).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data7](index.html) module"]
pub struct RD_KEY0_DATA7_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key0_data7::R](R) reader structure"]
impl crate::Readable for RD_KEY0_DATA7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY0_DATA7 to value 0"]
impl crate::Resettable for RD_KEY0_DATA7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
