#[doc = "Register `APP_CACHE_CTRL1` reader"]
pub type R = crate::R<APP_CACHE_CTRL1_SPEC>;
#[doc = "Register `APP_CACHE_CTRL1` writer"]
pub type W = crate::W<APP_CACHE_CTRL1_SPEC>;
#[doc = "Field `APP_CACHE_MASK_IRAM0` reader - "]
pub type APP_CACHE_MASK_IRAM0_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MASK_IRAM0` writer - "]
pub type APP_CACHE_MASK_IRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MASK_IRAM1` reader - "]
pub type APP_CACHE_MASK_IRAM1_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MASK_IRAM1` writer - "]
pub type APP_CACHE_MASK_IRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MASK_IROM0` reader - "]
pub type APP_CACHE_MASK_IROM0_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MASK_IROM0` writer - "]
pub type APP_CACHE_MASK_IROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MASK_DRAM1` reader - "]
pub type APP_CACHE_MASK_DRAM1_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MASK_DRAM1` writer - "]
pub type APP_CACHE_MASK_DRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MASK_DROM0` reader - "]
pub type APP_CACHE_MASK_DROM0_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MASK_DROM0` writer - "]
pub type APP_CACHE_MASK_DROM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MASK_OPSDRAM` reader - "]
pub type APP_CACHE_MASK_OPSDRAM_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MASK_OPSDRAM` writer - "]
pub type APP_CACHE_MASK_OPSDRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CMMU_SRAM_PAGE_MODE` reader - "]
pub type APP_CMMU_SRAM_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `APP_CMMU_SRAM_PAGE_MODE` writer - "]
pub type APP_CMMU_SRAM_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APP_CMMU_FLASH_PAGE_MODE` reader - "]
pub type APP_CMMU_FLASH_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `APP_CMMU_FLASH_PAGE_MODE` writer - "]
pub type APP_CMMU_FLASH_PAGE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APP_CMMU_FORCE_ON` reader - "]
pub type APP_CMMU_FORCE_ON_R = crate::BitReader;
#[doc = "Field `APP_CMMU_FORCE_ON` writer - "]
pub type APP_CMMU_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CMMU_PD` reader - "]
pub type APP_CMMU_PD_R = crate::BitReader;
#[doc = "Field `APP_CMMU_PD` writer - "]
pub type APP_CMMU_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CACHE_MMU_IA_CLR` reader - "]
pub type APP_CACHE_MMU_IA_CLR_R = crate::BitReader;
#[doc = "Field `APP_CACHE_MMU_IA_CLR` writer - "]
pub type APP_CACHE_MMU_IA_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mask_iram0(&self) -> APP_CACHE_MASK_IRAM0_R {
        APP_CACHE_MASK_IRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_cache_mask_iram1(&self) -> APP_CACHE_MASK_IRAM1_R {
        APP_CACHE_MASK_IRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn app_cache_mask_irom0(&self) -> APP_CACHE_MASK_IROM0_R {
        APP_CACHE_MASK_IROM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn app_cache_mask_dram1(&self) -> APP_CACHE_MASK_DRAM1_R {
        APP_CACHE_MASK_DRAM1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn app_cache_mask_drom0(&self) -> APP_CACHE_MASK_DROM0_R {
        APP_CACHE_MASK_DROM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn app_cache_mask_opsdram(&self) -> APP_CACHE_MASK_OPSDRAM_R {
        APP_CACHE_MASK_OPSDRAM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn app_cmmu_sram_page_mode(&self) -> APP_CMMU_SRAM_PAGE_MODE_R {
        APP_CMMU_SRAM_PAGE_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn app_cmmu_flash_page_mode(&self) -> APP_CMMU_FLASH_PAGE_MODE_R {
        APP_CMMU_FLASH_PAGE_MODE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn app_cmmu_force_on(&self) -> APP_CMMU_FORCE_ON_R {
        APP_CMMU_FORCE_ON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn app_cmmu_pd(&self) -> APP_CMMU_PD_R {
        APP_CMMU_PD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn app_cache_mmu_ia_clr(&self) -> APP_CACHE_MMU_IA_CLR_R {
        APP_CACHE_MMU_IA_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_CACHE_CTRL1")
            .field(
                "app_cache_mask_iram0",
                &format_args!("{}", self.app_cache_mask_iram0().bit()),
            )
            .field(
                "app_cache_mask_iram1",
                &format_args!("{}", self.app_cache_mask_iram1().bit()),
            )
            .field(
                "app_cache_mask_irom0",
                &format_args!("{}", self.app_cache_mask_irom0().bit()),
            )
            .field(
                "app_cache_mask_dram1",
                &format_args!("{}", self.app_cache_mask_dram1().bit()),
            )
            .field(
                "app_cache_mask_drom0",
                &format_args!("{}", self.app_cache_mask_drom0().bit()),
            )
            .field(
                "app_cache_mask_opsdram",
                &format_args!("{}", self.app_cache_mask_opsdram().bit()),
            )
            .field(
                "app_cmmu_sram_page_mode",
                &format_args!("{}", self.app_cmmu_sram_page_mode().bits()),
            )
            .field(
                "app_cmmu_flash_page_mode",
                &format_args!("{}", self.app_cmmu_flash_page_mode().bits()),
            )
            .field(
                "app_cmmu_force_on",
                &format_args!("{}", self.app_cmmu_force_on().bit()),
            )
            .field("app_cmmu_pd", &format_args!("{}", self.app_cmmu_pd().bit()))
            .field(
                "app_cache_mmu_ia_clr",
                &format_args!("{}", self.app_cache_mmu_ia_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_CACHE_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mask_iram0(&mut self) -> APP_CACHE_MASK_IRAM0_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MASK_IRAM0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mask_iram1(&mut self) -> APP_CACHE_MASK_IRAM1_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MASK_IRAM1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mask_irom0(&mut self) -> APP_CACHE_MASK_IROM0_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MASK_IROM0_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mask_dram1(&mut self) -> APP_CACHE_MASK_DRAM1_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MASK_DRAM1_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mask_drom0(&mut self) -> APP_CACHE_MASK_DROM0_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MASK_DROM0_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mask_opsdram(&mut self) -> APP_CACHE_MASK_OPSDRAM_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MASK_OPSDRAM_W::new(self, 5)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn app_cmmu_sram_page_mode(&mut self) -> APP_CMMU_SRAM_PAGE_MODE_W<APP_CACHE_CTRL1_SPEC> {
        APP_CMMU_SRAM_PAGE_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn app_cmmu_flash_page_mode(&mut self) -> APP_CMMU_FLASH_PAGE_MODE_W<APP_CACHE_CTRL1_SPEC> {
        APP_CMMU_FLASH_PAGE_MODE_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn app_cmmu_force_on(&mut self) -> APP_CMMU_FORCE_ON_W<APP_CACHE_CTRL1_SPEC> {
        APP_CMMU_FORCE_ON_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn app_cmmu_pd(&mut self) -> APP_CMMU_PD_W<APP_CACHE_CTRL1_SPEC> {
        APP_CMMU_PD_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn app_cache_mmu_ia_clr(&mut self) -> APP_CACHE_MMU_IA_CLR_W<APP_CACHE_CTRL1_SPEC> {
        APP_CACHE_MMU_IA_CLR_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_cache_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_cache_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_CACHE_CTRL1_SPEC;
impl crate::RegisterSpec for APP_CACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cache_ctrl1::R`](R) reader structure"]
impl crate::Readable for APP_CACHE_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_cache_ctrl1::W`](W) writer structure"]
impl crate::Writable for APP_CACHE_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_CACHE_CTRL1 to value 0x08ff"]
impl crate::Resettable for APP_CACHE_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x08ff;
}
