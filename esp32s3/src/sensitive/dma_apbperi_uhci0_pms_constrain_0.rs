#[doc = "Register `DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0` reader"]
pub struct R(crate::R<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0` writer"]
pub struct W(crate::W<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>;
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
impl From<crate::W<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK` reader - Set 1 to lock uhci0 dma permission Configuration Register."]
pub type DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK_R = crate::BitReader;
#[doc = "Field `DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK` writer - Set 1 to lock uhci0 dma permission Configuration Register."]
pub type DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock uhci0 dma permission Configuration Register."]
    #[inline(always)]
    pub fn dma_apbperi_uhci0_pms_constrain_lock(&self) -> DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK_R {
        DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0")
            .field(
                "dma_apbperi_uhci0_pms_constrain_lock",
                &format_args!("{}", self.dma_apbperi_uhci0_pms_constrain_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock uhci0 dma permission Configuration Register."]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_uhci0_pms_constrain_lock(
        &mut self,
    ) -> DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK_W<0> {
        DMA_APBPERI_UHCI0_PMS_CONSTRAIN_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uhci0 dma permission configuration register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_uhci0_pms_constrain_0](index.html) module"]
pub struct DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_uhci0_pms_constrain_0::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apbperi_uhci0_pms_constrain_0::W](W) writer structure"]
impl crate::Writable for DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0 to value 0"]
impl crate::Resettable for DMA_APBPERI_UHCI0_PMS_CONSTRAIN_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
