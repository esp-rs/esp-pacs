#[doc = "Register `RTC_FASTMEM_CRC` reader"]
pub struct R(crate::R<RTC_FASTMEM_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_FASTMEM_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_FASTMEM_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_FASTMEM_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_MEM_CRC_RES` reader - reg_rtc_mem_crc_res"]
pub type RTC_MEM_CRC_RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_rtc_mem_crc_res"]
    #[inline(always)]
    pub fn rtc_mem_crc_res(&self) -> RTC_MEM_CRC_RES_R {
        RTC_MEM_CRC_RES_R::new(self.bits)
    }
}
#[doc = "reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_fastmem_crc](index.html) module"]
pub struct RTC_FASTMEM_CRC_SPEC;
impl crate::RegisterSpec for RTC_FASTMEM_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_fastmem_crc::R](R) reader structure"]
impl crate::Readable for RTC_FASTMEM_CRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTC_FASTMEM_CRC to value 0"]
impl crate::Resettable for RTC_FASTMEM_CRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
