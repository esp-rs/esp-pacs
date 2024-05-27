#[doc = "Register `LP_CPU_PWR5` reader"]
pub type R = crate::R<LP_CPU_PWR5_SPEC>;
#[doc = "Field `LP_CPU_REJECT_CAUSE` reader - need_des"]
pub type LP_CPU_REJECT_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_reject_cause(&self) -> LP_CPU_REJECT_CAUSE_R {
        LP_CPU_REJECT_CAUSE_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CPU_PWR5")
            .field("lp_cpu_reject_cause", &self.lp_cpu_reject_cause())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CPU_PWR5_SPEC;
impl crate::RegisterSpec for LP_CPU_PWR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_pwr5::R`](R) reader structure"]
impl crate::Readable for LP_CPU_PWR5_SPEC {}
#[doc = "`reset()` method sets LP_CPU_PWR5 to value 0"]
impl crate::Resettable for LP_CPU_PWR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
