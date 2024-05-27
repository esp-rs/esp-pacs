#[doc = "Register `PCPU_NMI_INT1` reader"]
pub type R = crate::R<PCPU_NMI_INT1_SPEC>;
#[doc = "Field `PROCPU_NMI_INT_H` reader - GPIO32~39 PRO CPU non-maskable interrupt status"]
pub type PROCPU_NMI_INT_H_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 PRO CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn procpu_nmi_int_h(&self) -> PROCPU_NMI_INT_H_R {
        PROCPU_NMI_INT_H_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCPU_NMI_INT1")
            .field("procpu_nmi_int_h", &self.procpu_nmi_int_h())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCPU_NMI_INT1_SPEC;
impl crate::RegisterSpec for PCPU_NMI_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcpu_nmi_int1::R`](R) reader structure"]
impl crate::Readable for PCPU_NMI_INT1_SPEC {}
#[doc = "`reset()` method sets PCPU_NMI_INT1 to value 0"]
impl crate::Resettable for PCPU_NMI_INT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
