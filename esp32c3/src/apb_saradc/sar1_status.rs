#[doc = "Register `SAR1_STATUS` reader"]
pub type R = crate::R<SAR1_STATUS_SPEC>;
#[doc = "Field `SARADC_SAR1_STATUS` reader - saradc1 status about data and channel"]
pub type SARADC_SAR1_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - saradc1 status about data and channel"]
    #[inline(always)]
    pub fn saradc_sar1_status(&self) -> SARADC_SAR1_STATUS_R {
        SARADC_SAR1_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_STATUS")
            .field(
                "saradc_sar1_status",
                &format_args!("{}", self.saradc_sar1_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_STATUS_SPEC;
impl crate::RegisterSpec for SAR1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_status::R`](R) reader structure"]
impl crate::Readable for SAR1_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR1_STATUS to value 0"]
impl crate::Resettable for SAR1_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
