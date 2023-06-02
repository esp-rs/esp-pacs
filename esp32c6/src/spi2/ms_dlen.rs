#[doc = "Register `MS_DLEN` reader"]
pub struct R(crate::R<MS_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS_DLEN` writer"]
pub struct W(crate::W<MS_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS_DLEN_SPEC>;
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
impl From<crate::W<MS_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MS_DATA_BITLEN` reader - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type MS_DATA_BITLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MS_DATA_BITLEN` writer - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type MS_DATA_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, MS_DLEN_SPEC, 18, O, u32, u32>;
impl R {
    #[doc = "Bits 0:17 - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn ms_data_bitlen(&self) -> MS_DATA_BITLEN_R {
        MS_DATA_BITLEN_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MS_DLEN")
            .field(
                "ms_data_bitlen",
                &format_args!("{}", self.ms_data_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MS_DLEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:17 - The value of these bits is the configured SPI transmission data bit length in master mode DMA controlled transfer or CPU controlled transfer. The value is also the configured bit length in slave mode DMA RX controlled transfer. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ms_data_bitlen(&mut self) -> MS_DATA_BITLEN_W<0> {
        MS_DATA_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI data bit length control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_dlen](index.html) module"]
pub struct MS_DLEN_SPEC;
impl crate::RegisterSpec for MS_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_dlen::R](R) reader structure"]
impl crate::Readable for MS_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms_dlen::W](W) writer structure"]
impl crate::Writable for MS_DLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MS_DLEN to value 0"]
impl crate::Resettable for MS_DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
