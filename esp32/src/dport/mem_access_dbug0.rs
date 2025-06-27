#[doc = "Register `MEM_ACCESS_DBUG0` reader"]
pub type R = crate::R<MEM_ACCESS_DBUG0_SPEC>;
#[doc = "Field `PRO_ROM_MPU_AD` reader - "]
pub type PRO_ROM_MPU_AD_R = crate::BitReader;
#[doc = "Field `PRO_ROM_IA` reader - "]
pub type PRO_ROM_IA_R = crate::BitReader;
#[doc = "Field `APP_ROM_MPU_AD` reader - "]
pub type APP_ROM_MPU_AD_R = crate::BitReader;
#[doc = "Field `APP_ROM_IA` reader - "]
pub type APP_ROM_IA_R = crate::BitReader;
#[doc = "Field `SHARE_ROM_MPU_AD` reader - "]
pub type SHARE_ROM_MPU_AD_R = crate::FieldReader;
#[doc = "Field `SHARE_ROM_IA` reader - "]
pub type SHARE_ROM_IA_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_MMU_AD` reader - "]
pub type INTERNAL_SRAM_MMU_AD_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_IA` reader - "]
pub type INTERNAL_SRAM_IA_R = crate::FieldReader<u16>;
#[doc = "Field `INTERNAL_SRAM_MMU_MULTI_HIT` reader - "]
pub type INTERNAL_SRAM_MMU_MULTI_HIT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_mpu_ad(&self) -> PRO_ROM_MPU_AD_R {
        PRO_ROM_MPU_AD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_rom_ia(&self) -> PRO_ROM_IA_R {
        PRO_ROM_IA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_rom_mpu_ad(&self) -> APP_ROM_MPU_AD_R {
        APP_ROM_MPU_AD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_rom_ia(&self) -> APP_ROM_IA_R {
        APP_ROM_IA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn share_rom_mpu_ad(&self) -> SHARE_ROM_MPU_AD_R {
        SHARE_ROM_MPU_AD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn share_rom_ia(&self) -> SHARE_ROM_IA_R {
        SHARE_ROM_IA_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn internal_sram_mmu_ad(&self) -> INTERNAL_SRAM_MMU_AD_R {
        INTERNAL_SRAM_MMU_AD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:25"]
    #[inline(always)]
    pub fn internal_sram_ia(&self) -> INTERNAL_SRAM_IA_R {
        INTERNAL_SRAM_IA_R::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn internal_sram_mmu_multi_hit(&self) -> INTERNAL_SRAM_MMU_MULTI_HIT_R {
        INTERNAL_SRAM_MMU_MULTI_HIT_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_ACCESS_DBUG0")
            .field("pro_rom_mpu_ad", &self.pro_rom_mpu_ad())
            .field("pro_rom_ia", &self.pro_rom_ia())
            .field("app_rom_mpu_ad", &self.app_rom_mpu_ad())
            .field("app_rom_ia", &self.app_rom_ia())
            .field("share_rom_mpu_ad", &self.share_rom_mpu_ad())
            .field("share_rom_ia", &self.share_rom_ia())
            .field("internal_sram_mmu_ad", &self.internal_sram_mmu_ad())
            .field("internal_sram_ia", &self.internal_sram_ia())
            .field(
                "internal_sram_mmu_multi_hit",
                &self.internal_sram_mmu_multi_hit(),
            )
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_access_dbug0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ACCESS_DBUG0_SPEC;
impl crate::RegisterSpec for MEM_ACCESS_DBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_access_dbug0::R`](R) reader structure"]
impl crate::Readable for MEM_ACCESS_DBUG0_SPEC {}
#[doc = "`reset()` method sets MEM_ACCESS_DBUG0 to value 0"]
impl crate::Resettable for MEM_ACCESS_DBUG0_SPEC {}
