#[doc = "Register `SAR1DATA_STATUS` reader"]
pub type R = crate::R<SAR1DATA_STATUS_SPEC>;
#[doc = "Field `APB_SARADC1_DATA` reader - saradc1 data"]
pub type APB_SARADC1_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - saradc1 data"]
    #[inline(always)]
    pub fn apb_saradc1_data(&self) -> APB_SARADC1_DATA_R {
        APB_SARADC1_DATA_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1DATA_STATUS")
            .field(
                "apb_saradc1_data",
                &format_args!("{}", self.apb_saradc1_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR1DATA_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1data_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1DATA_STATUS_SPEC;
impl crate::RegisterSpec for SAR1DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1data_status::R`](R) reader structure"]
impl crate::Readable for SAR1DATA_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR1DATA_STATUS to value 0"]
impl crate::Resettable for SAR1DATA_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
