#[doc = "Register `Core_0_NMI_MASK` reader"]
pub struct R(crate::R<CORE_0_NMI_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_NMI_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_NMI_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_NMI_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_NMI_MASK` writer"]
pub struct W(crate::W<CORE_0_NMI_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_NMI_MASK_SPEC>;
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
impl From<crate::W<CORE_0_NMI_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_NMI_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_NMI_MASK` reader - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type CORE_0_NMI_MASK_R = crate::BitReader;
#[doc = "Field `CORE_0_NMI_MASK` writer - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type CORE_0_NMI_MASK_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_NMI_MASK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    pub fn core_0_nmi_mask(&self) -> CORE_0_NMI_MASK_R {
        CORE_0_NMI_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_NMI_MASK")
            .field(
                "core_0_nmi_mask",
                &format_args!("{}", self.core_0_nmi_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_NMI_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_nmi_mask(&mut self) -> CORE_0_NMI_MASK_W<0> {
        CORE_0_NMI_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_0 NMI mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_nmi_mask](index.html) module"]
pub struct CORE_0_NMI_MASK_SPEC;
impl crate::RegisterSpec for CORE_0_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_nmi_mask::R](R) reader structure"]
impl crate::Readable for CORE_0_NMI_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_nmi_mask::W](W) writer structure"]
impl crate::Writable for CORE_0_NMI_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_NMI_MASK to value 0"]
impl crate::Resettable for CORE_0_NMI_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
