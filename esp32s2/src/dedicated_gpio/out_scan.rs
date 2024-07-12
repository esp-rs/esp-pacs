#[doc = "Register `OUT_SCAN` reader"]
pub type R = crate::R<OUT_SCAN_SPEC>;
#[doc = "Field `OUT_STATUS` reader - GPIO out value configured by DEDIC_GPIO_OUT_DRT_REG, DEDIC_GPIO_OUT_MSK_REG, DEDIC_GPIO_OUT_IDV_REG."]
pub type OUT_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - GPIO out value configured by DEDIC_GPIO_OUT_DRT_REG, DEDIC_GPIO_OUT_MSK_REG, DEDIC_GPIO_OUT_IDV_REG."]
    #[inline(always)]
    pub fn out_status(&self) -> OUT_STATUS_R {
        OUT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_SCAN")
            .field("out_status", &self.out_status())
            .finish()
    }
}
#[doc = "Dedicated GPIO output status register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_scan::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SCAN_SPEC;
impl crate::RegisterSpec for OUT_SCAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_scan::R`](R) reader structure"]
impl crate::Readable for OUT_SCAN_SPEC {}
#[doc = "`reset()` method sets OUT_SCAN to value 0"]
impl crate::Resettable for OUT_SCAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
