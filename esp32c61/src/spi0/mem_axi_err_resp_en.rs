#[doc = "Register `MEM_AXI_ERR_RESP_EN` reader"]
pub type R = crate::R<MEM_AXI_ERR_RESP_EN_SPEC>;
#[doc = "Register `MEM_AXI_ERR_RESP_EN` writer"]
pub type W = crate::W<MEM_AXI_ERR_RESP_EN_SPEC>;
#[doc = "Field `MEM_AW_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type MEM_AW_RESP_EN_MMU_VLD_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
pub type MEM_AW_RESP_EN_MMU_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AW_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type MEM_AW_RESP_EN_MMU_GID_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
pub type MEM_AW_RESP_EN_MMU_GID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AW_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type MEM_AW_RESP_EN_AXI_SIZE_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi write trans."]
pub type MEM_AW_RESP_EN_AXI_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AW_RESP_EN_AXI_FLASH` reader - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type MEM_AW_RESP_EN_AXI_FLASH_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_AXI_FLASH` writer - Set this bit to enable AXI response function for axi flash err in axi write trans."]
pub type MEM_AW_RESP_EN_AXI_FLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AW_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type MEM_AW_RESP_EN_MMU_ECC_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
pub type MEM_AW_RESP_EN_MMU_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AW_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type MEM_AW_RESP_EN_MMU_SENS_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
pub type MEM_AW_RESP_EN_MMU_SENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AW_RESP_EN_AXI_WSTRB` reader - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type MEM_AW_RESP_EN_AXI_WSTRB_R = crate::BitReader;
#[doc = "Field `MEM_AW_RESP_EN_AXI_WSTRB` writer - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
pub type MEM_AW_RESP_EN_AXI_WSTRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AR_RESP_EN_MMU_VLD` reader - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_VLD_R = crate::BitReader;
#[doc = "Field `MEM_AR_RESP_EN_MMU_VLD` writer - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_VLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AR_RESP_EN_MMU_GID` reader - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_GID_R = crate::BitReader;
#[doc = "Field `MEM_AR_RESP_EN_MMU_GID` writer - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_GID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AR_RESP_EN_MMU_ECC` reader - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_ECC_R = crate::BitReader;
#[doc = "Field `MEM_AR_RESP_EN_MMU_ECC` writer - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_ECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AR_RESP_EN_MMU_SENS` reader - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_SENS_R = crate::BitReader;
#[doc = "Field `MEM_AR_RESP_EN_MMU_SENS` writer - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
pub type MEM_AR_RESP_EN_MMU_SENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AR_RESP_EN_AXI_SIZE` reader - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type MEM_AR_RESP_EN_AXI_SIZE_R = crate::BitReader;
#[doc = "Field `MEM_AR_RESP_EN_AXI_SIZE` writer - Set this bit to enable AXI response function for axi size err in axi read trans."]
pub type MEM_AR_RESP_EN_AXI_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_vld(&self) -> MEM_AW_RESP_EN_MMU_VLD_R {
        MEM_AW_RESP_EN_MMU_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_gid(&self) -> MEM_AW_RESP_EN_MMU_GID_R {
        MEM_AW_RESP_EN_MMU_GID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_axi_size(&self) -> MEM_AW_RESP_EN_AXI_SIZE_R {
        MEM_AW_RESP_EN_AXI_SIZE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_axi_flash(&self) -> MEM_AW_RESP_EN_AXI_FLASH_R {
        MEM_AW_RESP_EN_AXI_FLASH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_ecc(&self) -> MEM_AW_RESP_EN_MMU_ECC_R {
        MEM_AW_RESP_EN_MMU_ECC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_sens(&self) -> MEM_AW_RESP_EN_MMU_SENS_R {
        MEM_AW_RESP_EN_MMU_SENS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_axi_wstrb(&self) -> MEM_AW_RESP_EN_AXI_WSTRB_R {
        MEM_AW_RESP_EN_AXI_WSTRB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_vld(&self) -> MEM_AR_RESP_EN_MMU_VLD_R {
        MEM_AR_RESP_EN_MMU_VLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_gid(&self) -> MEM_AR_RESP_EN_MMU_GID_R {
        MEM_AR_RESP_EN_MMU_GID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_ecc(&self) -> MEM_AR_RESP_EN_MMU_ECC_R {
        MEM_AR_RESP_EN_MMU_ECC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_sens(&self) -> MEM_AR_RESP_EN_MMU_SENS_R {
        MEM_AR_RESP_EN_MMU_SENS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_axi_size(&self) -> MEM_AR_RESP_EN_AXI_SIZE_R {
        MEM_AR_RESP_EN_AXI_SIZE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_AXI_ERR_RESP_EN")
            .field("mem_aw_resp_en_mmu_vld", &self.mem_aw_resp_en_mmu_vld())
            .field("mem_aw_resp_en_mmu_gid", &self.mem_aw_resp_en_mmu_gid())
            .field("mem_aw_resp_en_axi_size", &self.mem_aw_resp_en_axi_size())
            .field("mem_aw_resp_en_axi_flash", &self.mem_aw_resp_en_axi_flash())
            .field("mem_aw_resp_en_mmu_ecc", &self.mem_aw_resp_en_mmu_ecc())
            .field("mem_aw_resp_en_mmu_sens", &self.mem_aw_resp_en_mmu_sens())
            .field("mem_aw_resp_en_axi_wstrb", &self.mem_aw_resp_en_axi_wstrb())
            .field("mem_ar_resp_en_mmu_vld", &self.mem_ar_resp_en_mmu_vld())
            .field("mem_ar_resp_en_mmu_gid", &self.mem_ar_resp_en_mmu_gid())
            .field("mem_ar_resp_en_mmu_ecc", &self.mem_ar_resp_en_mmu_ecc())
            .field("mem_ar_resp_en_mmu_sens", &self.mem_ar_resp_en_mmu_sens())
            .field("mem_ar_resp_en_axi_size", &self.mem_ar_resp_en_axi_size())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable AXI response function for mmu valid err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_vld(&mut self) -> MEM_AW_RESP_EN_MMU_VLD_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_MMU_VLD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable AXI response function for mmu gid err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_gid(&mut self) -> MEM_AW_RESP_EN_MMU_GID_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_MMU_GID_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable AXI response function for axi size err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_axi_size(
        &mut self,
    ) -> MEM_AW_RESP_EN_AXI_SIZE_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_AXI_SIZE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable AXI response function for axi flash err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_axi_flash(
        &mut self,
    ) -> MEM_AW_RESP_EN_AXI_FLASH_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_AXI_FLASH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable AXI response function for mmu ecc err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_ecc(&mut self) -> MEM_AW_RESP_EN_MMU_ECC_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_MMU_ECC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable AXI response function for mmu sens in err axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_mmu_sens(
        &mut self,
    ) -> MEM_AW_RESP_EN_MMU_SENS_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_MMU_SENS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable AXI response function for axi wstrb err in axi write trans."]
    #[inline(always)]
    pub fn mem_aw_resp_en_axi_wstrb(
        &mut self,
    ) -> MEM_AW_RESP_EN_AXI_WSTRB_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AW_RESP_EN_AXI_WSTRB_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable AXI response function for mmu valid err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_vld(&mut self) -> MEM_AR_RESP_EN_MMU_VLD_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AR_RESP_EN_MMU_VLD_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable AXI response function for mmu gid err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_gid(&mut self) -> MEM_AR_RESP_EN_MMU_GID_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AR_RESP_EN_MMU_GID_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable AXI response function for mmu ecc err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_ecc(&mut self) -> MEM_AR_RESP_EN_MMU_ECC_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AR_RESP_EN_MMU_ECC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to enable AXI response function for mmu sensitive err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_mmu_sens(
        &mut self,
    ) -> MEM_AR_RESP_EN_MMU_SENS_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AR_RESP_EN_MMU_SENS_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to enable AXI response function for axi size err in axi read trans."]
    #[inline(always)]
    pub fn mem_ar_resp_en_axi_size(
        &mut self,
    ) -> MEM_AR_RESP_EN_AXI_SIZE_W<MEM_AXI_ERR_RESP_EN_SPEC> {
        MEM_AR_RESP_EN_AXI_SIZE_W::new(self, 11)
    }
}
#[doc = "SPI0 AXI error response enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_axi_err_resp_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_axi_err_resp_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AXI_ERR_RESP_EN_SPEC;
impl crate::RegisterSpec for MEM_AXI_ERR_RESP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_axi_err_resp_en::R`](R) reader structure"]
impl crate::Readable for MEM_AXI_ERR_RESP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_axi_err_resp_en::W`](W) writer structure"]
impl crate::Writable for MEM_AXI_ERR_RESP_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_AXI_ERR_RESP_EN to value 0"]
impl crate::Resettable for MEM_AXI_ERR_RESP_EN_SPEC {}
