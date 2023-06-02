#[doc = "Register `DMA_RX_I_0` reader"]
pub struct R(crate::R<DMA_RX_I_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_I_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_I_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_I_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_I_0` writer"]
pub struct W(crate::W<DMA_RX_I_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_I_0_SPEC>;
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
impl From<crate::W<DMA_RX_I_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_I_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RX_I_LOCK` reader - Lock register. Setting to 1 locks RX Copy DMA permission control registers."]
pub type DMA_RX_I_LOCK_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_LOCK` writer - Lock register. Setting to 1 locks RX Copy DMA permission control registers."]
pub type DMA_RX_I_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks RX Copy DMA permission control registers."]
    #[inline(always)]
    pub fn dma_rx_i_lock(&self) -> DMA_RX_I_LOCK_R {
        DMA_RX_I_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RX_I_0")
            .field(
                "dma_rx_i_lock",
                &format_args!("{}", self.dma_rx_i_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_RX_I_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks RX Copy DMA permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_lock(&mut self) -> DMA_RX_I_LOCK_W<0> {
        DMA_RX_I_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Copy DMA permission control register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_i_0](index.html) module"]
pub struct DMA_RX_I_0_SPEC;
impl crate::RegisterSpec for DMA_RX_I_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_i_0::R](R) reader structure"]
impl crate::Readable for DMA_RX_I_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_i_0::W](W) writer structure"]
impl crate::Writable for DMA_RX_I_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RX_I_0 to value 0"]
impl crate::Resettable for DMA_RX_I_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
