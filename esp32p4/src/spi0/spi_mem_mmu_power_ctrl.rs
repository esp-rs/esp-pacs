#[doc = "Register `SPI_MEM_MMU_POWER_CTRL` reader"]
pub type R = crate::R<SPI_MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_MMU_POWER_CTRL` writer"]
pub type W = crate::W<SPI_MEM_MMU_POWER_CTRL_SPEC>;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` reader - Set this bit to enable mmu-memory clock force on"]
pub type SPI_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` writer - Set this bit to enable mmu-memory clock force on"]
pub type SPI_MMU_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MMU_MEM_FORCE_PD` reader - Set this bit to force mmu-memory powerdown"]
pub type SPI_MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_PD` writer - Set this bit to force mmu-memory powerdown"]
pub type SPI_MMU_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MMU_MEM_FORCE_PU` reader - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type SPI_MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_PU` writer - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type SPI_MMU_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    pub fn spi_mmu_mem_force_pd(&self) -> SPI_MMU_MEM_FORCE_PD_R {
        SPI_MMU_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    pub fn spi_mmu_mem_force_pu(&self) -> SPI_MMU_MEM_FORCE_PU_R {
        SPI_MMU_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
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
            .field(
                "spi_mmu_mem_force_on",
                &format_args!("{}", self.spi_mmu_mem_force_on().bit()),
            )
            .field(
                "spi_mmu_mem_force_pd",
                &format_args!("{}", self.spi_mmu_mem_force_pd().bit()),
            )
            .field(
                "spi_mmu_mem_force_pu",
                &format_args!("{}", self.spi_mmu_mem_force_pu().bit()),
            )
            .field(
                "spi_mem_aux_ctrl",
                &format_args!("{}", self.spi_mem_aux_ctrl().bits()),
            )
            .field(
                "spi_mem_rdn_ena",
                &format_args!("{}", self.spi_mem_rdn_ena().bit()),
            )
            .field(
                "spi_mem_rdn_result",
                &format_args!("{}", self.spi_mem_rdn_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_MMU_POWER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_mem_force_on(&mut self) -> SPI_MMU_MEM_FORCE_ON_W<SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MMU_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_mem_force_pd(&mut self) -> SPI_MMU_MEM_FORCE_PD_W<SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MMU_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_mem_force_pu(&mut self) -> SPI_MMU_MEM_FORCE_PU_W<SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MMU_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bits 16:29 - MMU PSRAM aux control register"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aux_ctrl(&mut self) -> SPI_MEM_AUX_CTRL_W<SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MEM_AUX_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 30 - ECO register enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_rdn_ena(&mut self) -> SPI_MEM_RDN_ENA_W<SPI_MEM_MMU_POWER_CTRL_SPEC> {
        SPI_MEM_RDN_ENA_W::new(self, 30)
    }
}
#[doc = "MSPI MMU power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_mmu_power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_mmu_power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_mmu_power_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_MMU_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_mmu_power_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_MMU_POWER_CTRL to value 0x1320_0004"]
impl crate::Resettable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1320_0004;
}
