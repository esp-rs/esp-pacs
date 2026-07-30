#[doc = "Register `CORE0_CPU_INT_EIP_STATUS` reader"]
pub type R = crate::R<CORE0_CPU_INT_EIP_STATUS_SPEC>;
#[doc = "Field `CORE0_CPU_INT_EIP_STATUS` reader - Need add description"]
pub type CORE0_CPU_INT_EIP_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_int_eip_status(&self) -> CORE0_CPU_INT_EIP_STATUS_R {
        CORE0_CPU_INT_EIP_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_CPU_INT_EIP_STATUS")
            .field("core0_cpu_int_eip_status", &self.core0_cpu_int_eip_status())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`core0_cpu_int_eip_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_CPU_INT_EIP_STATUS_SPEC;
impl crate::RegisterSpec for CORE0_CPU_INT_EIP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_cpu_int_eip_status::R`](R) reader structure"]
impl crate::Readable for CORE0_CPU_INT_EIP_STATUS_SPEC {}
#[doc = "`reset()` method sets CORE0_CPU_INT_EIP_STATUS to value 0"]
impl crate::Resettable for CORE0_CPU_INT_EIP_STATUS_SPEC {}
