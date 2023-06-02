#[doc = "Register `BLK7_W4` reader"]
pub struct R(crate::R<BLK7_W4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK7_W4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK7_W4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK7_W4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK7_W4` reader - Otp block7 word4 data."]
pub type BLOCK7_W4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block7 word4 data."]
    #[inline(always)]
    pub fn block7_w4(&self) -> BLOCK7_W4_R {
        BLOCK7_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W4")
            .field("block7_w4", &format_args!("{}", self.block7_w4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK7_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block7 data register4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk7_w4](index.html) module"]
pub struct BLK7_W4_SPEC;
impl crate::RegisterSpec for BLK7_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk7_w4::R](R) reader structure"]
impl crate::Readable for BLK7_W4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK7_W4 to value 0"]
impl crate::Resettable for BLK7_W4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
