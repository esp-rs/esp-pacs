///Register `PRO_CPU_RECORD_STATUS` reader
pub type R = crate::R<PRO_CPU_RECORD_STATUS_SPEC>;
///Field `PRO_CPU_RECORDING` reader -
pub type PRO_CPU_RECORDING_R = crate::BitReader;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn pro_cpu_recording(&self) -> PRO_CPU_RECORDING_R {
        PRO_CPU_RECORDING_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_STATUS")
            .field("pro_cpu_recording", &self.pro_cpu_recording())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_CPU_RECORD_STATUS_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_cpu_record_status::R`](R) reader structure
impl crate::Readable for PRO_CPU_RECORD_STATUS_SPEC {}
///`reset()` method sets PRO_CPU_RECORD_STATUS to value 0
impl crate::Resettable for PRO_CPU_RECORD_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
