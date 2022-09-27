#[doc = "Register `EXT2` reader"]
pub struct R(crate::R<EXT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ST` reader - The status of spi state machine ."]
pub type ST_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - The status of spi state machine ."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 7) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext2](index.html) module"]
pub struct EXT2_SPEC;
impl crate::RegisterSpec for EXT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext2::R](R) reader structure"]
impl crate::Readable for EXT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXT2 to value 0"]
impl crate::Resettable for EXT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
