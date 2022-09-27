#[doc = "Register `_1_TO_EOF_DES_ADDR` reader"]
pub struct R(crate::R<_1_TO_EOF_DES_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1_TO_EOF_DES_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1_TO_EOF_DES_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1_TO_EOF_DES_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC1_TO_EOF_DES_ADDR` reader - "]
pub type SLC1_TO_EOF_DES_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_to_eof_des_addr(&self) -> SLC1_TO_EOF_DES_ADDR_R {
        SLC1_TO_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_to_eof_des_addr](index.html) module"]
pub struct _1_TO_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for _1_TO_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1_to_eof_des_addr::R](R) reader structure"]
impl crate::Readable for _1_TO_EOF_DES_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _1_TO_EOF_DES_ADDR to value 0"]
impl crate::Resettable for _1_TO_EOF_DES_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
