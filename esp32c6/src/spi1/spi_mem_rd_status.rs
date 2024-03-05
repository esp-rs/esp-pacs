#[doc = "Register `SPI_MEM_RD_STATUS` reader"]
pub type R = crate::R<SPI_MEM_RD_STATUS_SPEC>;
#[doc = "Register `SPI_MEM_RD_STATUS` writer"]
pub type W = crate::W<SPI_MEM_RD_STATUS_SPEC>;
#[doc = "Field `SPI_MEM_STATUS` reader - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
pub type SPI_MEM_STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_STATUS` writer - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
pub type SPI_MEM_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPI_MEM_WB_MODE` reader - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_WB_MODE` writer - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_RD_STATUS")
            .field(
                "spi_mem_status",
                &format_args!("{}", self.spi_mem_status().bits()),
            )
            .field(
                "spi_mem_wb_mode",
                &format_args!("{}", self.spi_mem_wb_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_RD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value is stored when set spi_mem_flash_rdsr bit and spi_mem_flash_res bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_status(&mut self) -> SPI_MEM_STATUS_W<SPI_MEM_RD_STATUS_SPEC> {
        SPI_MEM_STATUS_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Mode bits in the flash fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wb_mode(&mut self) -> SPI_MEM_WB_MODE_W<SPI_MEM_RD_STATUS_SPEC> {
        SPI_MEM_WB_MODE_W::new(self, 16)
    }
}
#[doc = "SPI1 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_rd_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_rd_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_RD_STATUS_SPEC;
impl crate::RegisterSpec for SPI_MEM_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_rd_status::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_rd_status::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_RD_STATUS to value 0"]
impl crate::Resettable for SPI_MEM_RD_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
