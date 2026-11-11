#[doc = "Register `FMEM_PMS0_ATTR` reader"]
pub type R = crate::R<FMEM_PMS0_ATTR_SPEC>;
#[doc = "Register `FMEM_PMS0_ATTR` writer"]
pub type W = crate::W<FMEM_PMS0_ATTR_SPEC>;
#[doc = "Field `FMEM_PMS0_RD_ATTR` reader - 1: SPI1 flash PMS section 0 read accessible. 0: Not allowed."]
pub type FMEM_PMS0_RD_ATTR_R = crate::BitReader;
#[doc = "Field `FMEM_PMS0_RD_ATTR` writer - 1: SPI1 flash PMS section 0 read accessible. 0: Not allowed."]
pub type FMEM_PMS0_RD_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMEM_PMS0_WR_ATTR` reader - 1: SPI1 flash PMS section 0 write accessible. 0: Not allowed."]
pub type FMEM_PMS0_WR_ATTR_R = crate::BitReader;
#[doc = "Field `FMEM_PMS0_WR_ATTR` writer - 1: SPI1 flash PMS section 0 write accessible. 0: Not allowed."]
pub type FMEM_PMS0_WR_ATTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMEM_PMS0_ECC` reader - SPI1 flash PMS section 0 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 0 is configured by registers SPI_FMEM_PMS0_ADDR_REG and SPI_FMEM_PMS0_SIZE_REG."]
pub type FMEM_PMS0_ECC_R = crate::BitReader;
#[doc = "Field `FMEM_PMS0_ECC` writer - SPI1 flash PMS section 0 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 0 is configured by registers SPI_FMEM_PMS0_ADDR_REG and SPI_FMEM_PMS0_SIZE_REG."]
pub type FMEM_PMS0_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: SPI1 flash PMS section 0 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn fmem_pms0_rd_attr(&self) -> FMEM_PMS0_RD_ATTR_R {
        FMEM_PMS0_RD_ATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 flash PMS section 0 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn fmem_pms0_wr_attr(&self) -> FMEM_PMS0_WR_ATTR_R {
        FMEM_PMS0_WR_ATTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 flash PMS section 0 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 0 is configured by registers SPI_FMEM_PMS0_ADDR_REG and SPI_FMEM_PMS0_SIZE_REG."]
    #[inline(always)]
    pub fn fmem_pms0_ecc(&self) -> FMEM_PMS0_ECC_R {
        FMEM_PMS0_ECC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMEM_PMS0_ATTR")
            .field("fmem_pms0_rd_attr", &self.fmem_pms0_rd_attr())
            .field("fmem_pms0_wr_attr", &self.fmem_pms0_wr_attr())
            .field("fmem_pms0_ecc", &self.fmem_pms0_ecc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: SPI1 flash PMS section 0 read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn fmem_pms0_rd_attr(&mut self) -> FMEM_PMS0_RD_ATTR_W<'_, FMEM_PMS0_ATTR_SPEC> {
        FMEM_PMS0_RD_ATTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: SPI1 flash PMS section 0 write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn fmem_pms0_wr_attr(&mut self) -> FMEM_PMS0_WR_ATTR_W<'_, FMEM_PMS0_ATTR_SPEC> {
        FMEM_PMS0_WR_ATTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI1 flash PMS section 0 ECC mode, 1: enable ECC mode. 0: Disable it. The flash PMS section 0 is configured by registers SPI_FMEM_PMS0_ADDR_REG and SPI_FMEM_PMS0_SIZE_REG."]
    #[inline(always)]
    pub fn fmem_pms0_ecc(&mut self) -> FMEM_PMS0_ECC_W<'_, FMEM_PMS0_ATTR_SPEC> {
        FMEM_PMS0_ECC_W::new(self, 2)
    }
}
#[doc = "MSPI flash PMS section $n attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms0_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms0_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMEM_PMS0_ATTR_SPEC;
impl crate::RegisterSpec for FMEM_PMS0_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmem_pms0_attr::R`](R) reader structure"]
impl crate::Readable for FMEM_PMS0_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmem_pms0_attr::W`](W) writer structure"]
impl crate::Writable for FMEM_PMS0_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMEM_PMS0_ATTR to value 0x03"]
impl crate::Resettable for FMEM_PMS0_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
