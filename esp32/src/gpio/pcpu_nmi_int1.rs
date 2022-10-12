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
#[doc = "Field `PROCPU_NMI_INT_H` reader - GPIO32~39 PRO CPU non-maskable interrupt status"]
pub type PROCPU_NMI_INT_H_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 PRO CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn procpu_nmi_int_h(&self) -> PROCPU_NMI_INT_H_R {
        PROCPU_NMI_INT_H_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpu_nmi_int1](index.html) module"]
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