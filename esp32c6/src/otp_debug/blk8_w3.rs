#[doc = "Register `BLK8_W3` reader"]
pub struct R(crate::R<BLK8_W3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK8_W3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK8_W3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK8_W3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK8_W3` reader - Otp block8 word3 data."]
pub type BLOCK8_W3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word3 data."]
    #[inline(always)]
    pub fn block8_w3(&self) -> BLOCK8_W3_R {
        BLOCK8_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK8_W3")
            .field("block8_w3", &format_args!("{}", self.block8_w3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK8_W3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block8 data register3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk8_w3](index.html) module"]
pub struct BLK8_W3_SPEC;
impl crate::RegisterSpec for BLK8_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk8_w3::R](R) reader structure"]
impl crate::Readable for BLK8_W3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK8_W3 to value 0"]
impl crate::Resettable for BLK8_W3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
