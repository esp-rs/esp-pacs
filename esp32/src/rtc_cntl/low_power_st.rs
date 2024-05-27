#[doc = "Register `LOW_POWER_ST` reader"]
pub type R = crate::R<LOW_POWER_ST_SPEC>;
#[doc = "Field `LOW_POWER_DIAG0` reader - "]
pub type LOW_POWER_DIAG0_R = crate::FieldReader<u32>;
#[doc = "Field `RDY_FOR_WAKEUP` reader - 1 if RTC controller is ready to execute WAKE instruction, 0 otherwise"]
pub type RDY_FOR_WAKEUP_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn low_power_diag0(&self) -> LOW_POWER_DIAG0_R {
        LOW_POWER_DIAG0_R::new(self.bits)
    }
    #[doc = "Bit 19 - 1 if RTC controller is ready to execute WAKE instruction, 0 otherwise"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&self) -> RDY_FOR_WAKEUP_R {
        RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOW_POWER_ST")
            .field("low_power_diag0", &self.low_power_diag0())
            .field("rdy_for_wakeup", &self.rdy_for_wakeup())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_power_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_power_st::R`](R) reader structure"]
impl crate::Readable for LOW_POWER_ST_SPEC {}
#[doc = "`reset()` method sets LOW_POWER_ST to value 0"]
impl crate::Resettable for LOW_POWER_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
