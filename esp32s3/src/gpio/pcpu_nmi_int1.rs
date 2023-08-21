#[doc = "Register `PCPU_NMI_INT1` reader"]
pub type R = crate::R<PCPU_NMI_INT1_SPEC>;
#[doc = "Field `PROCPU_NMI_INT1` reader - GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-53"]
pub type PROCPU_NMI_INT1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-53"]
    #[inline(always)]
    pub fn procpu_nmi_int1(&self) -> PROCPU_NMI_INT1_R {
        PROCPU_NMI_INT1_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCPU_NMI_INT1")
            .field(
                "procpu_nmi_int1",
                &format_args!("{}", self.procpu_nmi_int1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PCPU_NMI_INT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register for GPIO32-53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCPU_NMI_INT1_SPEC;
impl crate::RegisterSpec for PCPU_NMI_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcpu_nmi_int1::R`](R) reader structure"]
impl crate::Readable for PCPU_NMI_INT1_SPEC {}
#[doc = "`reset()` method sets PCPU_NMI_INT1 to value 0"]
impl crate::Resettable for PCPU_NMI_INT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
