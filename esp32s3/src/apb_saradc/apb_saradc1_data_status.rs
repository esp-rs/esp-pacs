#[doc = "Register `APB_SARADC1_DATA_STATUS` reader"]
pub type R = crate::R<APB_SARADC1_DATA_STATUS_SPEC>;
#[doc = "Field `SARADC1_DATA` reader - apbsaradc sample data"]
pub type SARADC1_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - apbsaradc sample data"]
    #[inline(always)]
    pub fn saradc1_data(&self) -> SARADC1_DATA_R {
        SARADC1_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC1_DATA_STATUS")
            .field("saradc1_data", &self.saradc1_data())
            .finish()
    }
}
#[doc = "get apb saradc sample data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc1_data_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_SARADC1_DATA_STATUS_SPEC;
impl crate::RegisterSpec for APB_SARADC1_DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_saradc1_data_status::R`](R) reader structure"]
impl crate::Readable for APB_SARADC1_DATA_STATUS_SPEC {}
#[doc = "`reset()` method sets APB_SARADC1_DATA_STATUS to value 0"]
impl crate::Resettable for APB_SARADC1_DATA_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
