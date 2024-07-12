#[doc = "Register `IN_SCAN` reader"]
pub type R = crate::R<IN_SCAN_SPEC>;
#[doc = "Field `IN_STATUS` reader - GPIO input value after configured by DEDIC_GPIO_IN_DLY_REG."]
pub type IN_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - GPIO input value after configured by DEDIC_GPIO_IN_DLY_REG."]
    #[inline(always)]
    pub fn in_status(&self) -> IN_STATUS_R {
        IN_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SCAN")
            .field("in_status", &self.in_status())
            .finish()
    }
}
#[doc = "Dedicated GPIO input status register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_scan::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SCAN_SPEC;
impl crate::RegisterSpec for IN_SCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_scan::R`](R) reader structure"]
impl crate::Readable for IN_SCAN_SPEC {}
#[doc = "`reset()` method sets IN_SCAN to value 0"]
impl crate::Resettable for IN_SCAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
