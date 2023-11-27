#[doc = "Register `SPI_MEM_AXI_ERR_RESP_EN` reader"]
pub type R = crate::R<SPI_MEM_AXI_ERR_RESP_EN_SPEC>;
#[doc = "Register `SPI_MEM_AXI_ERR_RESP_EN` writer"]
pub type W = crate::W<SPI_MEM_AXI_ERR_RESP_EN_SPEC>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_VLD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_GID_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_GID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_AXI_SIZE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_AXI_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_AXI_FLASH` reader - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_AXI_FLASH_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_AXI_FLASH` writer - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_AXI_FLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_ECC_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_SENS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type SPI_MEM_AW_RESP_EN_MMU_SENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AW_RESP_EN_AXI_WSTRB` reader - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_AXI_WSTRB_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_RESP_EN_AXI_WSTRB` writer - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type SPI_MEM_AW_RESP_EN_AXI_WSTRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_VLD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_GID_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_GID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_ECC_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_SENS_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AR_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_MMU_SENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AR_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_AXI_SIZE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AR_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type SPI_MEM_AR_RESP_EN_AXI_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_vld(&self) -> SPI_MEM_AW_RESP_EN_MMU_VLD_R {
        SPI_MEM_AW_RESP_EN_MMU_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_gid(&self) -> SPI_MEM_AW_RESP_EN_MMU_GID_R {
        SPI_MEM_AW_RESP_EN_MMU_GID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_axi_size(&self) -> SPI_MEM_AW_RESP_EN_AXI_SIZE_R {
        SPI_MEM_AW_RESP_EN_AXI_SIZE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_axi_flash(&self) -> SPI_MEM_AW_RESP_EN_AXI_FLASH_R {
        SPI_MEM_AW_RESP_EN_AXI_FLASH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_ecc(&self) -> SPI_MEM_AW_RESP_EN_MMU_ECC_R {
        SPI_MEM_AW_RESP_EN_MMU_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_mmu_sens(&self) -> SPI_MEM_AW_RESP_EN_MMU_SENS_R {
        SPI_MEM_AW_RESP_EN_MMU_SENS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    pub fn spi_mem_aw_resp_en_axi_wstrb(&self) -> SPI_MEM_AW_RESP_EN_AXI_WSTRB_R {
        SPI_MEM_AW_RESP_EN_AXI_WSTRB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_vld(&self) -> SPI_MEM_AR_RESP_EN_MMU_VLD_R {
        SPI_MEM_AR_RESP_EN_MMU_VLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_gid(&self) -> SPI_MEM_AR_RESP_EN_MMU_GID_R {
        SPI_MEM_AR_RESP_EN_MMU_GID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_ecc(&self) -> SPI_MEM_AR_RESP_EN_MMU_ECC_R {
        SPI_MEM_AR_RESP_EN_MMU_ECC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_mmu_sens(&self) -> SPI_MEM_AR_RESP_EN_MMU_SENS_R {
        SPI_MEM_AR_RESP_EN_MMU_SENS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    pub fn spi_mem_ar_resp_en_axi_size(&self) -> SPI_MEM_AR_RESP_EN_AXI_SIZE_R {
        SPI_MEM_AR_RESP_EN_AXI_SIZE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_AXI_ERR_RESP_EN")
            .field(
                "spi_mem_aw_resp_en_mmu_vld",
                &format_args!("{}", self.spi_mem_aw_resp_en_mmu_vld().bit()),
            )
            .field(
                "spi_mem_aw_resp_en_mmu_gid",
                &format_args!("{}", self.spi_mem_aw_resp_en_mmu_gid().bit()),
            )
            .field(
                "spi_mem_aw_resp_en_axi_size",
                &format_args!("{}", self.spi_mem_aw_resp_en_axi_size().bit()),
            )
            .field(
                "spi_mem_aw_resp_en_axi_flash",
                &format_args!("{}", self.spi_mem_aw_resp_en_axi_flash().bit()),
            )
            .field(
                "spi_mem_aw_resp_en_mmu_ecc",
                &format_args!("{}", self.spi_mem_aw_resp_en_mmu_ecc().bit()),
            )
            .field(
                "spi_mem_aw_resp_en_mmu_sens",
                &format_args!("{}", self.spi_mem_aw_resp_en_mmu_sens().bit()),
            )
            .field(
                "spi_mem_aw_resp_en_axi_wstrb",
                &format_args!("{}", self.spi_mem_aw_resp_en_axi_wstrb().bit()),
            )
            .field(
                "spi_mem_ar_resp_en_mmu_vld",
                &format_args!("{}", self.spi_mem_ar_resp_en_mmu_vld().bit()),
            )
            .field(
                "spi_mem_ar_resp_en_mmu_gid",
                &format_args!("{}", self.spi_mem_ar_resp_en_mmu_gid().bit()),
            )
            .field(
                "spi_mem_ar_resp_en_mmu_ecc",
                &format_args!("{}", self.spi_mem_ar_resp_en_mmu_ecc().bit()),
            )
            .field(
                "spi_mem_ar_resp_en_mmu_sens",
                &format_args!("{}", self.spi_mem_ar_resp_en_mmu_sens().bit()),
            )
            .field(
                "spi_mem_ar_resp_en_axi_size",
                &format_args!("{}", self.spi_mem_ar_resp_en_axi_size().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_mmu_vld(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_MMU_VLD_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_MMU_VLD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_mmu_gid(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_MMU_GID_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_MMU_GID_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_axi_size(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_AXI_SIZE_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_AXI_SIZE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_axi_flash(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_AXI_FLASH_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_AXI_FLASH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_mmu_ecc(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_MMU_ECC_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_MMU_ECC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_mmu_sens(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_MMU_SENS_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_MMU_SENS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_aw_resp_en_axi_wstrb(
        &mut self,
    ) -> SPI_MEM_AW_RESP_EN_AXI_WSTRB_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AW_RESP_EN_AXI_WSTRB_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ar_resp_en_mmu_vld(
        &mut self,
    ) -> SPI_MEM_AR_RESP_EN_MMU_VLD_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AR_RESP_EN_MMU_VLD_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ar_resp_en_mmu_gid(
        &mut self,
    ) -> SPI_MEM_AR_RESP_EN_MMU_GID_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AR_RESP_EN_MMU_GID_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ar_resp_en_mmu_ecc(
        &mut self,
    ) -> SPI_MEM_AR_RESP_EN_MMU_ECC_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AR_RESP_EN_MMU_ECC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ar_resp_en_mmu_sens(
        &mut self,
    ) -> SPI_MEM_AR_RESP_EN_MMU_SENS_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AR_RESP_EN_MMU_SENS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ar_resp_en_axi_size(
        &mut self,
    ) -> SPI_MEM_AR_RESP_EN_AXI_SIZE_W<SPI_MEM_AXI_ERR_RESP_EN_SPEC> {
        SPI_MEM_AR_RESP_EN_AXI_SIZE_W::new(self, 11)
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
#[doc = "SPI0 AXI error response enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_axi_err_resp_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_axi_err_resp_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_AXI_ERR_RESP_EN_SPEC;
impl crate::RegisterSpec for SPI_MEM_AXI_ERR_RESP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_axi_err_resp_en::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_AXI_ERR_RESP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_axi_err_resp_en::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_AXI_ERR_RESP_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_AXI_ERR_RESP_EN to value 0"]
impl crate::Resettable for SPI_MEM_AXI_ERR_RESP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
