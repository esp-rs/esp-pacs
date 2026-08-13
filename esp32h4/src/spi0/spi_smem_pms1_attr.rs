#[doc = "Register `SPI_SMEM_PMS1_ATTR` reader"]
pub type R = crate::R<SPI_SMEM_PMS1_ATTR_SPEC>;
#[doc = "Register `SPI_SMEM_PMS1_ATTR` writer"]
pub type W = crate::W<SPI_SMEM_PMS1_ATTR_SPEC>;
#[doc = "Field `SPI_SMEM_PMS1_RD_ATTR` reader - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_RD_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_RD_ATTR` writer - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_WR_ATTR` reader - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_WR_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_WR_ATTR` writer - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_ECC` reader - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SPI_SMEM_PMS1_ECC_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_ECC` writer - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SPI_SMEM_PMS1_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_RD_ATTR` reader - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_NONSECURE_RD_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_RD_ATTR` writer - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_NONSECURE_RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_WR_ATTR` reader - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_NONSECURE_WR_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_WR_ATTR` writer - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
pub type SPI_SMEM_PMS1_NONSECURE_WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_ECC` reader - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SPI_SMEM_PMS1_NONSECURE_ECC_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_PMS1_NONSECURE_ECC` writer - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
pub type SPI_SMEM_PMS1_NONSECURE_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_rd_attr(&self) -> SPI_SMEM_PMS1_RD_ATTR_R {
        SPI_SMEM_PMS1_RD_ATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_wr_attr(&self) -> SPI_SMEM_PMS1_WR_ATTR_R {
        SPI_SMEM_PMS1_WR_ATTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_ecc(&self) -> SPI_SMEM_PMS1_ECC_R {
        SPI_SMEM_PMS1_ECC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_rd_attr(&self) -> SPI_SMEM_PMS1_NONSECURE_RD_ATTR_R {
        SPI_SMEM_PMS1_NONSECURE_RD_ATTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_wr_attr(&self) -> SPI_SMEM_PMS1_NONSECURE_WR_ATTR_R {
        SPI_SMEM_PMS1_NONSECURE_WR_ATTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_ecc(&self) -> SPI_SMEM_PMS1_NONSECURE_ECC_R {
        SPI_SMEM_PMS1_NONSECURE_ECC_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_PMS1_ATTR")
            .field("spi_smem_pms1_rd_attr", &self.spi_smem_pms1_rd_attr())
            .field("spi_smem_pms1_wr_attr", &self.spi_smem_pms1_wr_attr())
            .field("spi_smem_pms1_ecc", &self.spi_smem_pms1_ecc())
            .field(
                "spi_smem_pms1_nonsecure_rd_attr",
                &self.spi_smem_pms1_nonsecure_rd_attr(),
            )
            .field(
                "spi_smem_pms1_nonsecure_wr_attr",
                &self.spi_smem_pms1_nonsecure_wr_attr(),
            )
            .field(
                "spi_smem_pms1_nonsecure_ecc",
                &self.spi_smem_pms1_nonsecure_ecc(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: SPI1 external RAM PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_rd_attr(
        &mut self,
    ) -> SPI_SMEM_PMS1_RD_ATTR_W<'_, SPI_SMEM_PMS1_ATTR_SPEC> {
        SPI_SMEM_PMS1_RD_ATTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: SPI1 external RAM PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_wr_attr(
        &mut self,
    ) -> SPI_SMEM_PMS1_WR_ATTR_W<'_, SPI_SMEM_PMS1_ATTR_SPEC> {
        SPI_SMEM_PMS1_WR_ATTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI1 external RAM PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_ecc(&mut self) -> SPI_SMEM_PMS1_ECC_W<'_, SPI_SMEM_PMS1_ATTR_SPEC> {
        SPI_SMEM_PMS1_ECC_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: SPI1 external RAM non-secure PMS section 1 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_rd_attr(
        &mut self,
    ) -> SPI_SMEM_PMS1_NONSECURE_RD_ATTR_W<'_, SPI_SMEM_PMS1_ATTR_SPEC> {
        SPI_SMEM_PMS1_NONSECURE_RD_ATTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: SPI1 external RAM non-secure PMS section 1 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_wr_attr(
        &mut self,
    ) -> SPI_SMEM_PMS1_NONSECURE_WR_ATTR_W<'_, SPI_SMEM_PMS1_ATTR_SPEC> {
        SPI_SMEM_PMS1_NONSECURE_WR_ATTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - SPI1 external RAM non-secure PMS section 1 ECC mode, 1: enable ECC mode. 0: Disable it. The external RAM PMS section 1 is configured by registers SPI_SMEM_PMS1_ADDR_REG and SPI_SMEM_PMS1_SIZE_REG."]
    #[inline(always)]
    pub fn spi_smem_pms1_nonsecure_ecc(
        &mut self,
    ) -> SPI_SMEM_PMS1_NONSECURE_ECC_W<'_, SPI_SMEM_PMS1_ATTR_SPEC> {
        SPI_SMEM_PMS1_NONSECURE_ECC_W::new(self, 5)
    }
}
#[doc = "SPI1 external RAM PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms1_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_pms1_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_PMS1_ATTR_SPEC;
impl crate::RegisterSpec for SPI_SMEM_PMS1_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_pms1_attr::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_PMS1_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_pms1_attr::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_PMS1_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_PMS1_ATTR to value 0x1b"]
impl crate::Resettable for SPI_SMEM_PMS1_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x1b;
}
