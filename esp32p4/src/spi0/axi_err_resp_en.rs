#[doc = "Register `AXI_ERR_RESP_EN` reader"]
pub type R = crate::R<AXI_ERR_RESP_EN_SPEC>;
#[doc = "Register `AXI_ERR_RESP_EN` writer"]
pub type W = crate::W<AXI_ERR_RESP_EN_SPEC>;
#[doc = "Field `AW_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type AW_RESP_EN_MMU_VLD_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type AW_RESP_EN_MMU_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type AW_RESP_EN_MMU_GID_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type AW_RESP_EN_MMU_GID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type AW_RESP_EN_AXI_SIZE_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type AW_RESP_EN_AXI_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_AXI_FLASH` reader - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type AW_RESP_EN_AXI_FLASH_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_AXI_FLASH` writer - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type AW_RESP_EN_AXI_FLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type AW_RESP_EN_MMU_ECC_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type AW_RESP_EN_MMU_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type AW_RESP_EN_MMU_SENS_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type AW_RESP_EN_MMU_SENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_RESP_EN_AXI_WSTRB` reader - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type AW_RESP_EN_AXI_WSTRB_R = crate::BitReader;
#[doc = "Field `AW_RESP_EN_AXI_WSTRB` writer - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type AW_RESP_EN_AXI_WSTRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type AR_RESP_EN_MMU_VLD_R = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type AR_RESP_EN_MMU_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type AR_RESP_EN_MMU_GID_R = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type AR_RESP_EN_MMU_GID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type AR_RESP_EN_MMU_ECC_R = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type AR_RESP_EN_MMU_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type AR_RESP_EN_MMU_SENS_R = crate::BitReader;
#[doc = "Field `AR_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type AR_RESP_EN_MMU_SENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type AR_RESP_EN_AXI_SIZE_R = crate::BitReader;
#[doc = "Field `AR_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type AR_RESP_EN_AXI_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_vld(&self) -> AW_RESP_EN_MMU_VLD_R {
        AW_RESP_EN_MMU_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_gid(&self) -> AW_RESP_EN_MMU_GID_R {
        AW_RESP_EN_MMU_GID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_size(&self) -> AW_RESP_EN_AXI_SIZE_R {
        AW_RESP_EN_AXI_SIZE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_flash(&self) -> AW_RESP_EN_AXI_FLASH_R {
        AW_RESP_EN_AXI_FLASH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_ecc(&self) -> AW_RESP_EN_MMU_ECC_R {
        AW_RESP_EN_MMU_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_mmu_sens(&self) -> AW_RESP_EN_MMU_SENS_R {
        AW_RESP_EN_MMU_SENS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    pub fn aw_resp_en_axi_wstrb(&self) -> AW_RESP_EN_AXI_WSTRB_R {
        AW_RESP_EN_AXI_WSTRB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_vld(&self) -> AR_RESP_EN_MMU_VLD_R {
        AR_RESP_EN_MMU_VLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_gid(&self) -> AR_RESP_EN_MMU_GID_R {
        AR_RESP_EN_MMU_GID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_ecc(&self) -> AR_RESP_EN_MMU_ECC_R {
        AR_RESP_EN_MMU_ECC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_mmu_sens(&self) -> AR_RESP_EN_MMU_SENS_R {
        AR_RESP_EN_MMU_SENS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    pub fn ar_resp_en_axi_size(&self) -> AR_RESP_EN_AXI_SIZE_R {
        AR_RESP_EN_AXI_SIZE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ERR_RESP_EN")
            .field("aw_resp_en_mmu_vld", &self.aw_resp_en_mmu_vld().bit())
            .field("aw_resp_en_mmu_gid", &self.aw_resp_en_mmu_gid().bit())
            .field("aw_resp_en_axi_size", &self.aw_resp_en_axi_size().bit())
            .field("aw_resp_en_axi_flash", &self.aw_resp_en_axi_flash().bit())
            .field("aw_resp_en_mmu_ecc", &self.aw_resp_en_mmu_ecc().bit())
            .field("aw_resp_en_mmu_sens", &self.aw_resp_en_mmu_sens().bit())
            .field("aw_resp_en_axi_wstrb", &self.aw_resp_en_axi_wstrb().bit())
            .field("ar_resp_en_mmu_vld", &self.ar_resp_en_mmu_vld().bit())
            .field("ar_resp_en_mmu_gid", &self.ar_resp_en_mmu_gid().bit())
            .field("ar_resp_en_mmu_ecc", &self.ar_resp_en_mmu_ecc().bit())
            .field("ar_resp_en_mmu_sens", &self.ar_resp_en_mmu_sens().bit())
            .field("ar_resp_en_axi_size", &self.ar_resp_en_axi_size().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AXI_ERR_RESP_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_mmu_vld(&mut self) -> AW_RESP_EN_MMU_VLD_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_MMU_VLD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_mmu_gid(&mut self) -> AW_RESP_EN_MMU_GID_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_MMU_GID_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_axi_size(&mut self) -> AW_RESP_EN_AXI_SIZE_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_AXI_SIZE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_axi_flash(&mut self) -> AW_RESP_EN_AXI_FLASH_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_AXI_FLASH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_mmu_ecc(&mut self) -> AW_RESP_EN_MMU_ECC_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_MMU_ECC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_mmu_sens(&mut self) -> AW_RESP_EN_MMU_SENS_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_MMU_SENS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    #[must_use]
    pub fn aw_resp_en_axi_wstrb(&mut self) -> AW_RESP_EN_AXI_WSTRB_W<AXI_ERR_RESP_EN_SPEC> {
        AW_RESP_EN_AXI_WSTRB_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn ar_resp_en_mmu_vld(&mut self) -> AR_RESP_EN_MMU_VLD_W<AXI_ERR_RESP_EN_SPEC> {
        AR_RESP_EN_MMU_VLD_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn ar_resp_en_mmu_gid(&mut self) -> AR_RESP_EN_MMU_GID_W<AXI_ERR_RESP_EN_SPEC> {
        AR_RESP_EN_MMU_GID_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn ar_resp_en_mmu_ecc(&mut self) -> AR_RESP_EN_MMU_ECC_W<AXI_ERR_RESP_EN_SPEC> {
        AR_RESP_EN_MMU_ECC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn ar_resp_en_mmu_sens(&mut self) -> AR_RESP_EN_MMU_SENS_W<AXI_ERR_RESP_EN_SPEC> {
        AR_RESP_EN_MMU_SENS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    #[must_use]
    pub fn ar_resp_en_axi_size(&mut self) -> AR_RESP_EN_AXI_SIZE_W<AXI_ERR_RESP_EN_SPEC> {
        AR_RESP_EN_AXI_SIZE_W::new(self, 11)
    }
}
#[doc = "SPI0 AXI error response enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_err_resp_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_err_resp_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_ERR_RESP_EN_SPEC;
impl crate::RegisterSpec for AXI_ERR_RESP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_err_resp_en::R`](R) reader structure"]
impl crate::Readable for AXI_ERR_RESP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_err_resp_en::W`](W) writer structure"]
impl crate::Writable for AXI_ERR_RESP_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AXI_ERR_RESP_EN to value 0"]
impl crate::Resettable for AXI_ERR_RESP_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
