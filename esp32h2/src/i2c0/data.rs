#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_RDATA` reader - The value of rx FIFO read data."]
pub type FIFO_RDATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value of rx FIFO read data."]
    #[inline(always)]
    pub fn fifo_rdata(&self) -> FIFO_RDATA_R {
        FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Rx FIFO read data.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
