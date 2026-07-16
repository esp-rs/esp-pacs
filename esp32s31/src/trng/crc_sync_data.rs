#[doc = "Register `CRC_SYNC_DATA` reader"]
pub type R = crate::R<CRC_SYNC_DATA_SPEC>;
#[doc = "Field `SW_CRC_RANDOM_DATA_SYNC` reader - sw read crc random sync data"]
pub type SW_CRC_RANDOM_DATA_SYNC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sw read crc random sync data"]
    #[inline(always)]
    pub fn sw_crc_random_data_sync(&self) -> SW_CRC_RANDOM_DATA_SYNC_R {
        SW_CRC_RANDOM_DATA_SYNC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC_SYNC_DATA")
            .field("sw_crc_random_data_sync", &self.sw_crc_random_data_sync())
            .finish()
    }
}
#[doc = "sw read data sync\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_sync_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_SYNC_DATA_SPEC;
impl crate::RegisterSpec for CRC_SYNC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_sync_data::R`](R) reader structure"]
impl crate::Readable for CRC_SYNC_DATA_SPEC {}
#[doc = "`reset()` method sets CRC_SYNC_DATA to value 0"]
impl crate::Resettable for CRC_SYNC_DATA_SPEC {}
