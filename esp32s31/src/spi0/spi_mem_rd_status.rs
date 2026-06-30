#[doc = "Register `SPI_MEM_RD_STATUS` reader"]
pub type R = crate::R<SPI_MEM_RD_STATUS_SPEC>;
#[doc = "Register `SPI_MEM_RD_STATUS` writer"]
pub type W = crate::W<SPI_MEM_RD_STATUS_SPEC>;
#[doc = "Field `SPI_MEM_WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_MEM_WB_MODE_BITLEN` reader - Mode bits length for flash fast read mode."]
pub type SPI_MEM_WB_MODE_BITLEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_WB_MODE_BITLEN` writer - Mode bits length for flash fast read mode."]
pub type SPI_MEM_WB_MODE_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_MEM_WB_MODE_EN` reader - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
pub type SPI_MEM_WB_MODE_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WB_MODE_EN` writer - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
pub type SPI_MEM_WB_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&self) -> SPI_MEM_WB_MODE_R {
        SPI_MEM_WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Mode bits length for flash fast read mode."]
    #[inline(always)]
    pub fn spi_mem_wb_mode_bitlen(&self) -> SPI_MEM_WB_MODE_BITLEN_R {
        SPI_MEM_WB_MODE_BITLEN_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_wb_mode_en(&self) -> SPI_MEM_WB_MODE_EN_R {
        SPI_MEM_WB_MODE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_RD_STATUS")
            .field("spi_mem_wb_mode", &self.spi_mem_wb_mode())
            .field("spi_mem_wb_mode_bitlen", &self.spi_mem_wb_mode_bitlen())
            .field("spi_mem_wb_mode_en", &self.spi_mem_wb_mode_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_mem_wb_mode(&mut self) -> SPI_MEM_WB_MODE_W<'_, SPI_MEM_RD_STATUS_SPEC> {
        SPI_MEM_WB_MODE_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Mode bits length for flash fast read mode."]
    #[inline(always)]
    pub fn spi_mem_wb_mode_bitlen(
        &mut self,
    ) -> SPI_MEM_WB_MODE_BITLEN_W<'_, SPI_MEM_RD_STATUS_SPEC> {
        SPI_MEM_WB_MODE_BITLEN_W::new(self, 24)
    }
    #[doc = "Bit 27 - Mode bits is valid while this bit is enable. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_wb_mode_en(&mut self) -> SPI_MEM_WB_MODE_EN_W<'_, SPI_MEM_RD_STATUS_SPEC> {
        SPI_MEM_WB_MODE_EN_W::new(self, 27)
    }
}
#[doc = "SPI0 read control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_rd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_rd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_RD_STATUS_SPEC;
impl crate::RegisterSpec for SPI_MEM_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_rd_status::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_rd_status::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_RD_STATUS to value 0"]
impl crate::Resettable for SPI_MEM_RD_STATUS_SPEC {}
