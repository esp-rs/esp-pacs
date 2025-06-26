#[doc = "Register `PRO_CACHE_CTRL1` reader"]
pub type R = crate::R<PRO_CACHE_CTRL1_SPEC>;
#[doc = "Register `PRO_CACHE_CTRL1` writer"]
pub type W = crate::W<PRO_CACHE_CTRL1_SPEC>;
#[doc = "Field `PRO_CACHE_MASK_IRAM0` reader - "]
pub type PRO_CACHE_MASK_IRAM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_IRAM0` writer - "]
pub type PRO_CACHE_MASK_IRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MASK_IRAM1` reader - "]
pub type PRO_CACHE_MASK_IRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_IRAM1` writer - "]
pub type PRO_CACHE_MASK_IRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MASK_IROM0` reader - "]
pub type PRO_CACHE_MASK_IROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_IROM0` writer - "]
pub type PRO_CACHE_MASK_IROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MASK_DRAM1` reader - "]
pub type PRO_CACHE_MASK_DRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_DRAM1` writer - "]
pub type PRO_CACHE_MASK_DRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MASK_DROM0` reader - "]
pub type PRO_CACHE_MASK_DROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_DROM0` writer - "]
pub type PRO_CACHE_MASK_DROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MASK_OPSDRAM` reader - "]
pub type PRO_CACHE_MASK_OPSDRAM_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_OPSDRAM` writer - "]
pub type PRO_CACHE_MASK_OPSDRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CMMU_SRAM_PAGE_MODE` reader - "]
pub type PRO_CMMU_SRAM_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `PRO_CMMU_SRAM_PAGE_MODE` writer - "]
pub type PRO_CMMU_SRAM_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRO_CMMU_FLASH_PAGE_MODE` reader - "]
pub type PRO_CMMU_FLASH_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `PRO_CMMU_FLASH_PAGE_MODE` writer - "]
pub type PRO_CMMU_FLASH_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRO_CMMU_FORCE_ON` reader - "]
pub type PRO_CMMU_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PRO_CMMU_FORCE_ON` writer - "]
pub type PRO_CMMU_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CMMU_PD` reader - "]
pub type PRO_CMMU_PD_R = crate::BitReader;
#[doc = "Field `PRO_CMMU_PD` writer - "]
pub type PRO_CMMU_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_CACHE_MMU_IA_CLR` reader - "]
pub type PRO_CACHE_MMU_IA_CLR_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MMU_IA_CLR` writer - "]
pub type PRO_CACHE_MMU_IA_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_mask_iram0(&self) -> PRO_CACHE_MASK_IRAM0_R {
        PRO_CACHE_MASK_IRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_mask_iram1(&self) -> PRO_CACHE_MASK_IRAM1_R {
        PRO_CACHE_MASK_IRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pro_cache_mask_irom0(&self) -> PRO_CACHE_MASK_IROM0_R {
        PRO_CACHE_MASK_IROM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pro_cache_mask_dram1(&self) -> PRO_CACHE_MASK_DRAM1_R {
        PRO_CACHE_MASK_DRAM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cache_mask_drom0(&self) -> PRO_CACHE_MASK_DROM0_R {
        PRO_CACHE_MASK_DROM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro_cache_mask_opsdram(&self) -> PRO_CACHE_MASK_OPSDRAM_R {
        PRO_CACHE_MASK_OPSDRAM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn pro_cmmu_sram_page_mode(&self) -> PRO_CMMU_SRAM_PAGE_MODE_R {
        PRO_CMMU_SRAM_PAGE_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn pro_cmmu_flash_page_mode(&self) -> PRO_CMMU_FLASH_PAGE_MODE_R {
        PRO_CMMU_FLASH_PAGE_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_cmmu_force_on(&self) -> PRO_CMMU_FORCE_ON_R {
        PRO_CMMU_FORCE_ON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_cmmu_pd(&self) -> PRO_CMMU_PD_R {
        PRO_CMMU_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_cache_mmu_ia_clr(&self) -> PRO_CACHE_MMU_IA_CLR_R {
        PRO_CACHE_MMU_IA_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_CTRL1")
            .field("pro_cache_mask_iram0", &self.pro_cache_mask_iram0())
            .field("pro_cache_mask_iram1", &self.pro_cache_mask_iram1())
            .field("pro_cache_mask_irom0", &self.pro_cache_mask_irom0())
            .field("pro_cache_mask_dram1", &self.pro_cache_mask_dram1())
            .field("pro_cache_mask_drom0", &self.pro_cache_mask_drom0())
            .field("pro_cache_mask_opsdram", &self.pro_cache_mask_opsdram())
            .field("pro_cmmu_sram_page_mode", &self.pro_cmmu_sram_page_mode())
            .field("pro_cmmu_flash_page_mode", &self.pro_cmmu_flash_page_mode())
            .field("pro_cmmu_force_on", &self.pro_cmmu_force_on())
            .field("pro_cmmu_pd", &self.pro_cmmu_pd())
            .field("pro_cache_mmu_ia_clr", &self.pro_cache_mmu_ia_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_mask_iram0(&mut self) -> PRO_CACHE_MASK_IRAM0_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MASK_IRAM0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_cache_mask_iram1(&mut self) -> PRO_CACHE_MASK_IRAM1_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MASK_IRAM1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pro_cache_mask_irom0(&mut self) -> PRO_CACHE_MASK_IROM0_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MASK_IROM0_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pro_cache_mask_dram1(&mut self) -> PRO_CACHE_MASK_DRAM1_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MASK_DRAM1_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pro_cache_mask_drom0(&mut self) -> PRO_CACHE_MASK_DROM0_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MASK_DROM0_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro_cache_mask_opsdram(&mut self) -> PRO_CACHE_MASK_OPSDRAM_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MASK_OPSDRAM_W::new(self, 5)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn pro_cmmu_sram_page_mode(&mut self) -> PRO_CMMU_SRAM_PAGE_MODE_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CMMU_SRAM_PAGE_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn pro_cmmu_flash_page_mode(&mut self) -> PRO_CMMU_FLASH_PAGE_MODE_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CMMU_FLASH_PAGE_MODE_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pro_cmmu_force_on(&mut self) -> PRO_CMMU_FORCE_ON_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CMMU_FORCE_ON_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pro_cmmu_pd(&mut self) -> PRO_CMMU_PD_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CMMU_PD_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pro_cache_mmu_ia_clr(&mut self) -> PRO_CACHE_MMU_IA_CLR_W<PRO_CACHE_CTRL1_SPEC> {
        PRO_CACHE_MMU_IA_CLR_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cache_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cache_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_CTRL1_SPEC;
impl crate::RegisterSpec for PRO_CACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_ctrl1::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_ctrl1::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_CACHE_CTRL1 to value 0x08ff"]
impl crate::Resettable for PRO_CACHE_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x08ff;
}
