#[doc = "Register `CPU_PWAIT_MODE` reader"]
pub type R = crate::R<CPU_PWAIT_MODE_SPEC>;
#[doc = "Field `CPU_PWAIT_MODE` reader - Indicate status of core0 and core1 pwait"]
pub type CPU_PWAIT_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Indicate status of core0 and core1 pwait"]
    #[inline(always)]
    pub fn cpu_pwait_mode(&self) -> CPU_PWAIT_MODE_R {
        CPU_PWAIT_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PWAIT_MODE")
            .field("cpu_pwait_mode", &self.cpu_pwait_mode())
            .finish()
    }
}
#[doc = "Indicate status of core0 and core1 pwait\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pwait_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PWAIT_MODE_SPEC;
impl crate::RegisterSpec for CPU_PWAIT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pwait_mode::R`](R) reader structure"]
impl crate::Readable for CPU_PWAIT_MODE_SPEC {}
#[doc = "`reset()` method sets CPU_PWAIT_MODE to value 0"]
impl crate::Resettable for CPU_PWAIT_MODE_SPEC {}
