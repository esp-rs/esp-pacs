///Register `APB_SARADC2_DATA_STATUS` reader
pub type R = crate::R<APB_SARADC2_DATA_STATUS_SPEC>;
///Field `SARADC2_DATA` reader - apb saradc2 sample data
pub type SARADC2_DATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:16 - apb saradc2 sample data
    #[inline(always)]
    pub fn saradc2_data(&self) -> SARADC2_DATA_R {
        SARADC2_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC2_DATA_STATUS")
            .field("saradc2_data", &self.saradc2_data())
            .finish()
    }
}
/**get apb saradc2 sample data

You can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc2_data_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB_SARADC2_DATA_STATUS_SPEC;
impl crate::RegisterSpec for APB_SARADC2_DATA_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb_saradc2_data_status::R`](R) reader structure
impl crate::Readable for APB_SARADC2_DATA_STATUS_SPEC {}
///`reset()` method sets APB_SARADC2_DATA_STATUS to value 0
impl crate::Resettable for APB_SARADC2_DATA_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
