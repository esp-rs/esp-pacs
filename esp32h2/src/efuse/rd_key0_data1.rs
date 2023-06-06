#[doc = "Register `RD_KEY0_DATA1` reader"]
pub struct R(crate::R<RD_KEY0_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY0_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY0_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY0_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY0_DATA1` reader - Stores the first 32 bits of KEY0."]
pub type KEY0_DATA1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the first 32 bits of KEY0."]
    #[inline(always)]
    pub fn key0_data1(&self) -> KEY0_DATA1_R {
        KEY0_DATA1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY0_DATA1")
            .field("key0_data1", &format_args!("{}", self.key0_data1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_KEY0_DATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register $n of BLOCK4 (KEY0).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data1](index.html) module"]
pub struct RD_KEY0_DATA1_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key0_data1::R](R) reader structure"]
impl crate::Readable for RD_KEY0_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY0_DATA1 to value 0"]
impl crate::Resettable for RD_KEY0_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
