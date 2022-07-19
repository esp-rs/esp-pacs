#[doc = "Register `SPI_MEM_RD_STATUS` reader"]
pub struct R(crate::R<SPI_MEM_RD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_RD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_RD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_RD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_RD_STATUS` writer"]
pub struct W(crate::W<SPI_MEM_RD_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_RD_STATUS_SPEC>;
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
impl From<crate::W<SPI_MEM_RD_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_RD_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_STATUS` reader - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
pub type SPI_MEM_STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPI_MEM_STATUS` writer - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
pub type SPI_MEM_STATUS_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_RD_STATUS_SPEC, u16, u16, 16, 0>;
#[doc = "Field `SPI_MEM_WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_RD_STATUS_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:15 - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
    #[inline(always)]
    pub fn spi_mem_status(&self) -> SPI_MEM_STATUS_R {
        SPI_MEM_STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&self) -> SPI_MEM_WB_MODE_R {
        SPI_MEM_WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
    #[inline(always)]
    pub fn spi_mem_status(&mut self) -> SPI_MEM_STATUS_W {
        SPI_MEM_STATUS_W::new(self)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&mut self) -> SPI_MEM_WB_MODE_W {
        SPI_MEM_WB_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_rd_status](index.html) module"]
pub struct SPI_MEM_RD_STATUS_SPEC;
impl crate::RegisterSpec for SPI_MEM_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_rd_status::R](R) reader structure"]
impl crate::Readable for SPI_MEM_RD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_rd_status::W](W) writer structure"]
impl crate::Writable for SPI_MEM_RD_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_RD_STATUS to value 0"]
impl crate::Resettable for SPI_MEM_RD_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
