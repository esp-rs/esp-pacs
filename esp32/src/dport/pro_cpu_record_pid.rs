#[doc = "Register `PRO_CPU_RECORD_PID` reader"]
pub type R = crate::R<PRO_CPU_RECORD_PID_SPEC>;
#[doc = "Field `RECORD_PRO_PID` reader - "]
pub type RECORD_PRO_PID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn record_pro_pid(&self) -> RECORD_PRO_PID_R {
        RECORD_PRO_PID_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PID")
            .field(
                "record_pro_pid",
                &format_args!("{}", self.record_pro_pid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CPU_RECORD_PID_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cpu_record_pid::R`](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PID_SPEC {}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PID to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
