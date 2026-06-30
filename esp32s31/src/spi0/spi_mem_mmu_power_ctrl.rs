#[doc = "Register `SPI_MEM_MMU_POWER_CTRL` reader"]
pub type R = crate::R<SPI_MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_MMU_POWER_CTRL` writer"]
pub type W = crate::W<SPI_MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` reader - Set this bit to enable mmu-memory clock force on"]
pub type SPI_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` writer - Set this bit to enable mmu-memory clock force on"]
pub type SPI_MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MMU_PAGE_SIZE` reader - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type SPI_MMU_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SPI_MMU_PAGE_SIZE` writer - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type SPI_MMU_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MEM_AUX_CTRL` reader - MMU PSRAM aux control register"]
pub type SPI_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_AUX_CTRL` writer - MMU PSRAM aux control register"]
pub type SPI_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `SPI_MEM_RDN_ENA` reader - ECO register enable bit"]
pub type SPI_MEM_RDN_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_RDN_ENA` writer - ECO register enable bit"]
pub type SPI_MEM_RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_RDN_RESULT` reader - MSPI module clock domain and AXI clock domain ECO register result register"]
pub type SPI_MEM_RDN_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    pub fn spi_mmu_mem_force_on(&self) -> SPI_MMU_MEM_FORCE_ON_R {
        SPI_MMU_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    pub fn spi_mmu_page_size(&self) -> SPI_MMU_PAGE_SIZE_R {
        SPI_MMU_PAGE_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    pub fn spi_mem_aux_ctrl(&self) -> SPI_MEM_AUX_CTRL_R {
        SPI_MEM_AUX_CTRL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    pub fn spi_mem_rdn_ena(&self) -> SPI_MEM_RDN_ENA_R {
        SPI_MEM_RDN_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MSPI module clock domain and AXI clock domain ECO register result register"]
    #[inline(always)]
    pub fn spi_mem_rdn_result(&self) -> SPI_MEM_RDN_RESULT_R {
        SPI_MEM_RDN_RESULT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_MMU_POWER_CTRL")
            .field("spi_mmu_mem_force_on", &self.spi_mmu_mem_force_on())
            .field("spi_mmu_page_size", &self.spi_mmu_page_size())
            .field("spi_mem_aux_ctrl", &self.spi_mem_aux_ctrl())
            .field("spi_mem_rdn_ena", &self.spi_mem_rdn_ena())
            .field("spi_mem_rdn_result", &self.spi_mem_rdn_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    pub fn spi_mmu_mem_force_on(
        &mut self,
    ) -> SPI_MMU_MEM_FORCE_ON_W<'_, SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    pub fn spi_mmu_page_size(&mut self) -> SPI_MMU_PAGE_SIZE_W<'_, SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MMU_PAGE_SIZE_W::new(self, 3)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    pub fn spi_mem_aux_ctrl(&mut self) -> SPI_MEM_AUX_CTRL_W<'_, SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MEM_AUX_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    pub fn spi_mem_rdn_ena(&mut self) -> SPI_MEM_RDN_ENA_W<'_, SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MEM_RDN_ENA_W::new(self, 30)
    }
}
#[doc = "MSPI MMU power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_mmu_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_mem_mmu_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MEM_MMU_POWER_CTRL to value 0x1320_0000"]
impl crate::Resettable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320_0000;
}
