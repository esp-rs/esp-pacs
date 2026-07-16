#[doc = "Register `MMU_POWER_CTRL` reader"]
pub type R = crate::R<MMU_POWER_CTRL_SPEC>;
#[doc = "Register `MMU_POWER_CTRL` writer"]
pub type W = crate::W<MMU_POWER_CTRL_SPEC>;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` reader - "]
pub type SPI_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` writer - "]
pub type SPI_MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MMU_PAGE_SIZE` reader - "]
pub type SPI_MMU_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SPI_MMU_PAGE_SIZE` writer - "]
pub type SPI_MMU_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AUX_CTRL` reader - "]
pub type AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `AUX_CTRL` writer - "]
pub type AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RDN_ENA` reader - "]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - "]
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDN_RESULT` reader - "]
pub type RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mmu_mem_force_on(&self) -> SPI_MMU_MEM_FORCE_ON_R {
        SPI_MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn spi_mmu_page_size(&self) -> SPI_MMU_PAGE_SIZE_R {
        SPI_MMU_PAGE_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn aux_ctrl(&self) -> AUX_CTRL_R {
        AUX_CTRL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMU_POWER_CTRL")
            .field("spi_mmu_mem_force_on", &self.spi_mmu_mem_force_on())
            .field("spi_mmu_page_size", &self.spi_mmu_page_size())
            .field("aux_ctrl", &self.aux_ctrl())
            .field("rdn_ena", &self.rdn_ena())
            .field("rdn_result", &self.rdn_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_mmu_mem_force_on(&mut self) -> SPI_MMU_MEM_FORCE_ON_W<'_, MMU_POWER_CTRL_SPEC> {
        SPI_MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn spi_mmu_page_size(&mut self) -> SPI_MMU_PAGE_SIZE_W<'_, MMU_POWER_CTRL_SPEC> {
        SPI_MMU_PAGE_SIZE_W::new(self, 3)
    }
    #[doc = "Bits 16:29"]
    #[inline(always)]
    pub fn aux_ctrl(&mut self) -> AUX_CTRL_W<'_, MMU_POWER_CTRL_SPEC> {
        AUX_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<'_, MMU_POWER_CTRL_SPEC> {
        RDN_ENA_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMU_POWER_CTRL to value 0x1320_0000"]
impl crate::Resettable for MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320_0000;
}
