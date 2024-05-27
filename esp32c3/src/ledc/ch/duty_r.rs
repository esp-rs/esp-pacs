///Register `DUTY_R` reader
pub type R = crate::R<DUTY_R_SPEC>;
///Field `DUTY_R` reader - reg_duty_lsch0_r.
pub type DUTY_R_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:18 - reg_duty_lsch0_r.
    #[inline(always)]
    pub fn duty_r(&self) -> DUTY_R_R {
        DUTY_R_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DUTY_R")
            .field("duty_r", &self.duty_r())
            .finish()
    }
}
/**LEDC_LSCH0_DUTY_R.

You can [`read`](crate::generic::Reg::read) this register and get [`duty_r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DUTY_R_SPEC;
impl crate::RegisterSpec for DUTY_R_SPEC {
    type Ux = u32;
}
///`read()` method returns [`duty_r::R`](R) reader structure
impl crate::Readable for DUTY_R_SPEC {}
///`reset()` method sets DUTY_R to value 0
impl crate::Resettable for DUTY_R_SPEC {
    const RESET_VALUE: u32 = 0;
}
