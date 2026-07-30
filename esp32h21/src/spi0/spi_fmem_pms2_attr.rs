#[doc = "Register `SPI_FMEM_PMS2_ATTR` reader"]
pub type R = crate::R<SPI_FMEM_PMS2_ATTR_SPEC>;
#[doc = "Register `SPI_FMEM_PMS2_ATTR` writer"]
pub type W = crate::W<SPI_FMEM_PMS2_ATTR_SPEC>;
#[doc = "Field `SPI_FMEM_PMS2_RD_ATTR` reader - 1: SPI1 flash PMS section 2 read accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS2_RD_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PMS2_RD_ATTR` writer - 1: SPI1 flash PMS section 2 read accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS2_RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_PMS2_WR_ATTR` reader - 1: SPI1 flash PMS section 2 write accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS2_WR_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PMS2_WR_ATTR` writer - 1: SPI1 flash PMS section 2 write accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS2_WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_PMS2_ECC` reader - SPI1 flash PMS section 2 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 2 is configured by registers SPI_FMEM_PMS2_ADDR_REG and SPI_FMEM_PMS2_SIZE_REG."]
pub type SPI_FMEM_PMS2_ECC_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PMS2_ECC` writer - SPI1 flash PMS section 2 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 2 is configured by registers SPI_FMEM_PMS2_ADDR_REG and SPI_FMEM_PMS2_SIZE_REG."]
pub type SPI_FMEM_PMS2_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: SPI1 flash PMS section 2 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_fmem_pms2_rd_attr(&self) -> SPI_FMEM_PMS2_RD_ATTR_R {
        SPI_FMEM_PMS2_RD_ATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 flash PMS section 2 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_fmem_pms2_wr_attr(&self) -> SPI_FMEM_PMS2_WR_ATTR_R {
        SPI_FMEM_PMS2_WR_ATTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 flash PMS section 2 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 2 is configured by registers SPI_FMEM_PMS2_ADDR_REG and SPI_FMEM_PMS2_SIZE_REG."]
    #[inline(always)]
    pub fn spi_fmem_pms2_ecc(&self) -> SPI_FMEM_PMS2_ECC_R {
        SPI_FMEM_PMS2_ECC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FMEM_PMS2_ATTR")
            .field("spi_fmem_pms2_rd_attr", &self.spi_fmem_pms2_rd_attr())
            .field("spi_fmem_pms2_wr_attr", &self.spi_fmem_pms2_wr_attr())
            .field("spi_fmem_pms2_ecc", &self.spi_fmem_pms2_ecc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: SPI1 flash PMS section 2 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_fmem_pms2_rd_attr(
        &mut self,
    ) -> SPI_FMEM_PMS2_RD_ATTR_W<'_, SPI_FMEM_PMS2_ATTR_SPEC> {
        SPI_FMEM_PMS2_RD_ATTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: SPI1 flash PMS section 2 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_fmem_pms2_wr_attr(
        &mut self,
    ) -> SPI_FMEM_PMS2_WR_ATTR_W<'_, SPI_FMEM_PMS2_ATTR_SPEC> {
        SPI_FMEM_PMS2_WR_ATTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI1 flash PMS section 2 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 2 is configured by registers SPI_FMEM_PMS2_ADDR_REG and SPI_FMEM_PMS2_SIZE_REG."]
    #[inline(always)]
    pub fn spi_fmem_pms2_ecc(&mut self) -> SPI_FMEM_PMS2_ECC_W<'_, SPI_FMEM_PMS2_ATTR_SPEC> {
        SPI_FMEM_PMS2_ECC_W::new(self, 2)
    }
}
#[doc = "SPI1 flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fmem_pms2_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fmem_pms2_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_FMEM_PMS2_ATTR_SPEC;
impl crate::RegisterSpec for SPI_FMEM_PMS2_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_fmem_pms2_attr::R`](R) reader structure"]
impl crate::Readable for SPI_FMEM_PMS2_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_fmem_pms2_attr::W`](W) writer structure"]
impl crate::Writable for SPI_FMEM_PMS2_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_FMEM_PMS2_ATTR to value 0x03"]
impl crate::Resettable for SPI_FMEM_PMS2_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
