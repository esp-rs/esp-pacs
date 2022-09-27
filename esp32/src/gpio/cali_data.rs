#[doc = "Register `cali_data` reader"]
pub struct R(crate::R<CALI_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALI_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALI_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALI_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALI_VALUE_SYNC2` reader - "]
pub type CALI_VALUE_SYNC2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CALI_RDY_REAL` reader - "]
pub type CALI_RDY_REAL_R = crate::BitReader<bool>;
#[doc = "Field `CALI_RDY_SYNC2` reader - "]
pub type CALI_RDY_SYNC2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn cali_value_sync2(&self) -> CALI_VALUE_SYNC2_R {
        CALI_VALUE_SYNC2_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cali_rdy_real(&self) -> CALI_RDY_REAL_R {
        CALI_RDY_REAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_rdy_sync2(&self) -> CALI_RDY_SYNC2_R {
        CALI_RDY_SYNC2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cali_data](index.html) module"]
pub struct CALI_DATA_SPEC;
impl crate::RegisterSpec for CALI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cali_data::R](R) reader structure"]
impl crate::Readable for CALI_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cali_data to value 0"]
impl crate::Resettable for CALI_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
