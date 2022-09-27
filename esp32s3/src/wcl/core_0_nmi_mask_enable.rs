#[doc = "Register `Core_0_NMI_MASK_ENABLE` writer"]
pub struct W(crate::W<CORE_0_NMI_MASK_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_NMI_MASK_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_NMI_MASK_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_NMI_MASK_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_NMI_MASK_ENABLE` writer - this field is used to set NMI mask,it can write any value,when write this register,the hardware start masking NMI interrupt"]
pub type CORE_0_NMI_MASK_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_NMI_MASK_ENABLE_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - this field is used to set NMI mask,it can write any value,when write this register,the hardware start masking NMI interrupt"]
    #[inline(always)]
    pub fn core_0_nmi_mask_enable(&mut self) -> CORE_0_NMI_MASK_ENABLE_W<0> {
        CORE_0_NMI_MASK_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 NMI mask enable register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_nmi_mask_enable](index.html) module"]
pub struct CORE_0_NMI_MASK_ENABLE_SPEC;
impl crate::RegisterSpec for CORE_0_NMI_MASK_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core_0_nmi_mask_enable::W](W) writer structure"]
impl crate::Writable for CORE_0_NMI_MASK_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Core_0_NMI_MASK_ENABLE to value 0"]
impl crate::Resettable for CORE_0_NMI_MASK_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
