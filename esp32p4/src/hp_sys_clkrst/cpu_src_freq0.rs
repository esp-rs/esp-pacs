#[doc = "Register `CPU_SRC_FREQ0` reader"]
pub type R = crate::R<CPU_SRC_FREQ0_SPEC>;
#[doc = "Field `CPU_SRC_FREQ` reader - cpu source clock frequency, step by 0.25MHz"]
pub type CPU_SRC_FREQ_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cpu source clock frequency, step by 0.25MHz"]
    #[inline(always)]
    pub fn cpu_src_freq(&self) -> CPU_SRC_FREQ_R {
        CPU_SRC_FREQ_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_SRC_FREQ0")
            .field("cpu_src_freq", &self.cpu_src_freq().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_SRC_FREQ0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "CPU Source Frequency\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_src_freq0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_SRC_FREQ0_SPEC;
impl crate::RegisterSpec for CPU_SRC_FREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_src_freq0::R`](R) reader structure"]
impl crate::Readable for CPU_SRC_FREQ0_SPEC {}
#[doc = "`reset()` method sets CPU_SRC_FREQ0 to value 0"]
impl crate::Resettable for CPU_SRC_FREQ0_SPEC {
    const RESET_VALUE: u32 = 0;
}
