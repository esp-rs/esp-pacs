#[doc = "Register `SPI_MEM_MMU_POWER_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_MMU_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_MMU_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_MMU_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_MMU_POWER_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_MMU_POWER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_MMU_POWER_CTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_MMU_POWER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_MMU_POWER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` reader - Set this bit to enable mmu-memory clock force on"]
pub type SPI_MMU_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_ON` writer - Set this bit to enable mmu-memory clock force on"]
pub type SPI_MMU_MEM_FORCE_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_MMU_POWER_CTRL_SPEC, O>;
#[doc = "Field `SPI_MMU_MEM_FORCE_PD` reader - Set this bit to force mmu-memory powerdown"]
pub type SPI_MMU_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_PD` writer - Set this bit to force mmu-memory powerdown"]
pub type SPI_MMU_MEM_FORCE_PD_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_MMU_POWER_CTRL_SPEC, O>;
#[doc = "Field `SPI_MMU_MEM_FORCE_PU` reader - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type SPI_MMU_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SPI_MMU_MEM_FORCE_PU` writer - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
pub type SPI_MMU_MEM_FORCE_PU_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_MMU_POWER_CTRL_SPEC, O>;
#[doc = "Field `SPI_MMU_PAGE_SIZE` reader - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type SPI_MMU_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SPI_MMU_PAGE_SIZE` writer - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
pub type SPI_MMU_PAGE_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_MMU_POWER_CTRL_SPEC, 2, O>;
#[doc = "Field `SPI_MEM_AUX_CTRL` reader - MMU PSRAM aux control register"]
pub type SPI_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_MEM_RDN_ENA` reader - ECO register enable bit"]
pub type SPI_MEM_RDN_ENA_R = crate::BitReader;
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
                "spi_mmu_page_size",
                &format_args!("{}", self.spi_mmu_page_size().bits()),
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable mmu-memory clock force on"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_mem_force_on(&mut self) -> SPI_MMU_MEM_FORCE_ON_W<0> {
        SPI_MMU_MEM_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to force mmu-memory powerdown"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_mem_force_pd(&mut self) -> SPI_MMU_MEM_FORCE_PD_W<1> {
        SPI_MMU_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to force mmu-memory powerup, in this case, the power should also be controlled by rtc."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_mem_force_pu(&mut self) -> SPI_MMU_MEM_FORCE_PU_W<2> {
        SPI_MMU_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bits 3:4 - 0: Max page size , 1: Max page size/2 , 2: Max page size/4, 3: Max page size/8"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mmu_page_size(&mut self) -> SPI_MMU_PAGE_SIZE_W<3> {
        SPI_MMU_PAGE_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI MMU power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_mmu_power_ctrl](index.html) module"]
pub struct SPI_MEM_MMU_POWER_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_mmu_power_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_mmu_power_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_MMU_POWER_CTRL to value 0x1320_0004"]
impl crate::Resettable for SPI_MEM_MMU_POWER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1320_0004;
}
