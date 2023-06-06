#[doc = "Register `SPI_SMEM_PMS%s_SIZE` reader"]
pub struct R(crate::R<SPI_SMEM_PMS_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SMEM_PMS_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SMEM_PMS_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SMEM_PMS_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SMEM_PMS%s_SIZE` writer"]
pub struct W(crate::W<SPI_SMEM_PMS_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SMEM_PMS_SIZE_SPEC>;
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
impl From<crate::W<SPI_SMEM_PMS_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SMEM_PMS_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SMEM_PMS_SIZE` reader - SPI1 external RAM ACE section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
pub type SPI_SMEM_PMS_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_SMEM_PMS_SIZE` writer - SPI1 external RAM ACE section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
pub type SPI_SMEM_PMS_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_SMEM_PMS_SIZE_SPEC, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - SPI1 external RAM ACE section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
    #[inline(always)]
    pub fn spi_smem_pms_size(&self) -> SPI_SMEM_PMS_SIZE_R {
        SPI_SMEM_PMS_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_PMS_SIZE")
            .field(
                "spi_smem_pms_size",
                &format_args!("{}", self.spi_smem_pms_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_PMS_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - SPI1 external RAM ACE section %s address region is (SPI_SMEM_PMS%s_ADDR_S, SPI_SMEM_PMS%s_ADDR_S + SPI_SMEM_PMS%s_SIZE)"]
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_pms_size(&mut self) -> SPI_SMEM_PMS_SIZE_W<0> {
        SPI_SMEM_PMS_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 external RAM ACE section %s start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_smem_pms_size](index.html) module"]
pub struct SPI_SMEM_PMS_SIZE_SPEC;
impl crate::RegisterSpec for SPI_SMEM_PMS_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_smem_pms_size::R](R) reader structure"]
impl crate::Readable for SPI_SMEM_PMS_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_smem_pms_size::W](W) writer structure"]
impl crate::Writable for SPI_SMEM_PMS_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SMEM_PMS%s_SIZE to value 0x1000"]
impl crate::Resettable for SPI_SMEM_PMS_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
