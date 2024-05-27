///Register `APP_CPU_RECORD_PID` reader
pub type R = crate::R<APP_CPU_RECORD_PID_SPEC>;
///Field `RECORD_APP_PID` reader -
pub type RECORD_APP_PID_R = crate::FieldReader;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn record_app_pid(&self) -> RECORD_APP_PID_R {
        RECORD_APP_PID_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CPU_RECORD_PID")
            .field("record_app_pid", &self.record_app_pid())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_cpu_record_pid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_CPU_RECORD_PID_SPEC;
impl crate::RegisterSpec for APP_CPU_RECORD_PID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_cpu_record_pid::R`](R) reader structure
impl crate::Readable for APP_CPU_RECORD_PID_SPEC {}
///`reset()` method sets APP_CPU_RECORD_PID to value 0
impl crate::Resettable for APP_CPU_RECORD_PID_SPEC {
    const RESET_VALUE: u32 = 0;
}
