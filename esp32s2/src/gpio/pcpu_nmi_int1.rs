#[doc = "Register `PCPU_NMI_INT1` reader"]
pub struct R(crate::R<PCPU_NMI_INT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPU_NMI_INT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPU_NMI_INT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPU_NMI_INT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCPU_NMI1_INT` reader - GPIO32 ~ 53 PRO_CPU non-maskable interrupt status. This interrupt status is corresponding to bit in GPIO_STATUS1_REG when assert (high) enable signal (bit 14 of GPIO_PINn_REG)."]
pub type PROCPU_NMI1_INT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO32 ~ 53 PRO_CPU non-maskable interrupt status. This interrupt status is corresponding to bit in GPIO_STATUS1_REG when assert (high) enable signal (bit 14 of GPIO_PINn_REG)."]
    #[inline(always)]
    pub fn procpu_nmi1_int(&self) -> PROCPU_NMI1_INT_R {
        PROCPU_NMI1_INT_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "GPIO32 ~ 53 PRO_CPU non-maskable interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_nmi_int1](index.html) module"]
pub struct PCPU_NMI_INT1_SPEC;
impl crate::RegisterSpec for PCPU_NMI_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpu_nmi_int1::R](R) reader structure"]
impl crate::Readable for PCPU_NMI_INT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCPU_NMI_INT1 to value 0"]
impl crate::Resettable for PCPU_NMI_INT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
