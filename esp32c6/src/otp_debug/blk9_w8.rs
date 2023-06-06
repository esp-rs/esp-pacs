#[doc = "Register `BLK9_W8` reader"]
pub struct R(crate::R<BLK9_W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK9_W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK9_W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK9_W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK9_W8` reader - Otp block9 word8 data."]
pub type BLOCK9_W8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block9 word8 data."]
    #[inline(always)]
    pub fn block9_w8(&self) -> BLOCK9_W8_R {
        BLOCK9_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK9_W8")
            .field("block9_w8", &format_args!("{}", self.block9_w8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK9_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block9 data register8.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk9_w8](index.html) module"]
pub struct BLK9_W8_SPEC;
impl crate::RegisterSpec for BLK9_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk9_w8::R](R) reader structure"]
impl crate::Readable for BLK9_W8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK9_W8 to value 0"]
impl crate::Resettable for BLK9_W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
