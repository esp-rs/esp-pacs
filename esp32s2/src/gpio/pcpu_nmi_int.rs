///Register `PCPU_NMI_INT` reader
pub type R = crate::R<PCPU_NMI_INT_SPEC>;
///Field `PROCPU_NMI_INT` reader - GPIO0 ~ 31 PRO_CPU non-maskable interrupt status. This interrupt sta- tus is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit 14 of GPIO_PINn_REG).
pub type PROCPU_NMI_INT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - GPIO0 ~ 31 PRO_CPU non-maskable interrupt status. This interrupt sta- tus is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit 14 of GPIO_PINn_REG).
    #[inline(always)]
    pub fn procpu_nmi_int(&self) -> PROCPU_NMI_INT_R {
        PROCPU_NMI_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCPU_NMI_INT")
            .field("procpu_nmi_int", &self.procpu_nmi_int())
            .finish()
    }
}
/**GPIO0 ~ 31 PRO_CPU non-maskable interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PCPU_NMI_INT_SPEC;
impl crate::RegisterSpec for PCPU_NMI_INT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pcpu_nmi_int::R`](R) reader structure
impl crate::Readable for PCPU_NMI_INT_SPEC {}
///`reset()` method sets PCPU_NMI_INT to value 0
impl crate::Resettable for PCPU_NMI_INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
