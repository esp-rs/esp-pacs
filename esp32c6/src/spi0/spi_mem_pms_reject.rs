#[doc = "Register `SPI_MEM_PMS_REJECT` reader"]
pub type R = crate::R<SPI_MEM_PMS_REJECT_SPEC>;
#[doc = "Register `SPI_MEM_PMS_REJECT` writer"]
pub type W = crate::W<SPI_MEM_PMS_REJECT_SPEC>;
#[doc = "Field `SPI_MEM_REJECT_ADDR` reader - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type SPI_MEM_REJECT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_PM_EN` reader - Set this bit to enable SPI0/1 transfer permission control function."]
pub type SPI_MEM_PM_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PM_EN` writer - Set this bit to enable SPI0/1 transfer permission control function."]
pub type SPI_MEM_PM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PMS_LD` reader - 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type SPI_MEM_PMS_LD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_ST` reader - 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type SPI_MEM_PMS_ST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_MULTI_HIT` reader - 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type SPI_MEM_PMS_MULTI_HIT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_IVD` reader - 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type SPI_MEM_PMS_IVD_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:25 - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_reject_addr(&self) -> SPI_MEM_REJECT_ADDR_R {
        SPI_MEM_REJECT_ADDR_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - Set this bit to enable SPI0/1 transfer permission control function."]
    #[inline(always)]
    pub fn spi_mem_pm_en(&self) -> SPI_MEM_PM_EN_R {
        SPI_MEM_PM_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: SPI1 write access error. 0: No write access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_pms_ld(&self) -> SPI_MEM_PMS_LD_R {
        SPI_MEM_PMS_LD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: SPI1 read access error. 0: No read access error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_pms_st(&self) -> SPI_MEM_PMS_ST_R {
        SPI_MEM_PMS_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: SPI1 access is rejected because of address miss. 0: No address miss error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_pms_multi_hit(&self) -> SPI_MEM_PMS_MULTI_HIT_R {
        SPI_MEM_PMS_MULTI_HIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: SPI1 access is rejected because of address multi-hit. 0: No address multi-hit error. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_pms_ivd(&self) -> SPI_MEM_PMS_IVD_R {
        SPI_MEM_PMS_IVD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_PMS_REJECT")
            .field(
                "spi_mem_reject_addr",
                &format_args!("{}", self.spi_mem_reject_addr().bits()),
            )
            .field(
                "spi_mem_pm_en",
                &format_args!("{}", self.spi_mem_pm_en().bit()),
            )
            .field(
                "spi_mem_pms_ld",
                &format_args!("{}", self.spi_mem_pms_ld().bit()),
            )
            .field(
                "spi_mem_pms_st",
                &format_args!("{}", self.spi_mem_pms_st().bit()),
            )
            .field(
                "spi_mem_pms_multi_hit",
                &format_args!("{}", self.spi_mem_pms_multi_hit().bit()),
            )
            .field(
                "spi_mem_pms_ivd",
                &format_args!("{}", self.spi_mem_pms_ivd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_PMS_REJECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 26 - Set this bit to enable SPI0/1 transfer permission control function."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pm_en(&mut self) -> SPI_MEM_PM_EN_W<SPI_MEM_PMS_REJECT_SPEC> {
        SPI_MEM_PM_EN_W::new(self, 26)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI1 access reject register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_pms_reject::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_pms_reject::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_PMS_REJECT_SPEC;
impl crate::RegisterSpec for SPI_MEM_PMS_REJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_pms_reject::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_PMS_REJECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_pms_reject::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_PMS_REJECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_PMS_REJECT to value 0"]
impl crate::Resettable for SPI_MEM_PMS_REJECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
