#[doc = "Register `CRC_DATA` reader"]
pub type R = crate::R<CRC_DATA_SPEC>;
#[doc = "Field `SW_CRC_RANDOM_DATA` reader - sw read crc random data"]
pub type SW_CRC_RANDOM_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sw read crc random data"]
    #[inline(always)]
    pub fn sw_crc_random_data(&self) -> SW_CRC_RANDOM_DATA_R {
        SW_CRC_RANDOM_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_DATA")
            .field("sw_crc_random_data", &self.sw_crc_random_data())
            .finish()
    }
}
#[doc = "sw read data\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_DATA_SPEC;
impl crate::RegisterSpec for CRC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_data::R`](R) reader structure"]
impl crate::Readable for CRC_DATA_SPEC {}
#[doc = "`reset()` method sets CRC_DATA to value 0"]
impl crate::Resettable for CRC_DATA_SPEC {}
