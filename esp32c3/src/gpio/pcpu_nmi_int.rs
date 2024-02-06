#[doc = "Register `PCPU_NMI_INT` reader"]
pub type R = crate::R<PCPU_NMI_INT_SPEC>;
#[doc = "Field `PROCPU_NMI_INT` reader - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-25"]
pub type PROCPU_NMI_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:25 - GPIO PRO_CPU(not shielded) interrupt status register for GPIO0-25"]
    #[inline(always)]
    pub fn procpu_nmi_int(&self) -> PROCPU_NMI_INT_R {
        PROCPU_NMI_INT_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCPU_NMI_INT")
            .field(
                "procpu_nmi_int",
                &format_args!("{}", self.procpu_nmi_int().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PCPU_NMI_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCPU_NMI_INT_SPEC;
impl crate::RegisterSpec for PCPU_NMI_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcpu_nmi_int::R`](R) reader structure"]
impl crate::Readable for PCPU_NMI_INT_SPEC {}
#[doc = "`reset()` method sets PCPU_NMI_INT to value 0"]
impl crate::Resettable for PCPU_NMI_INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
