#[doc = "Register `PRO_CACHE_CTRL1` reader"]
pub type R = crate::R<PRO_CACHE_CTRL1_SPEC>;
#[doc = "Register `PRO_CACHE_CTRL1` writer"]
pub type W = crate::W<PRO_CACHE_CTRL1_SPEC>;
#[doc = "Field `PRO_CACHE_MASK_IRAM0` reader - "]
pub type PRO_CACHE_MASK_IRAM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_IRAM0` writer - "]
pub type PRO_CACHE_MASK_IRAM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CACHE_MASK_IRAM1` reader - "]
pub type PRO_CACHE_MASK_IRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_IRAM1` writer - "]
pub type PRO_CACHE_MASK_IRAM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CACHE_MASK_IROM0` reader - "]
pub type PRO_CACHE_MASK_IROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_IROM0` writer - "]
pub type PRO_CACHE_MASK_IROM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CACHE_MASK_DRAM1` reader - "]
pub type PRO_CACHE_MASK_DRAM1_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_DRAM1` writer - "]
pub type PRO_CACHE_MASK_DRAM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CACHE_MASK_DROM0` reader - "]
pub type PRO_CACHE_MASK_DROM0_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_DROM0` writer - "]
pub type PRO_CACHE_MASK_DROM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CACHE_MASK_OPSDRAM` reader - "]
pub type PRO_CACHE_MASK_OPSDRAM_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MASK_OPSDRAM` writer - "]
pub type PRO_CACHE_MASK_OPSDRAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CMMU_SRAM_PAGE_MODE` reader - "]
pub type PRO_CMMU_SRAM_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `PRO_CMMU_SRAM_PAGE_MODE` writer - "]
pub type PRO_CMMU_SRAM_PAGE_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PRO_CMMU_FLASH_PAGE_MODE` reader - "]
pub type PRO_CMMU_FLASH_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `PRO_CMMU_FLASH_PAGE_MODE` writer - "]
pub type PRO_CMMU_FLASH_PAGE_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PRO_CMMU_FORCE_ON` reader - "]
pub type PRO_CMMU_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PRO_CMMU_FORCE_ON` writer - "]
pub type PRO_CMMU_FORCE_ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CMMU_PD` reader - "]
pub type PRO_CMMU_PD_R = crate::BitReader;
#[doc = "Field `PRO_CMMU_PD` writer - "]
pub type PRO_CMMU_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRO_CACHE_MMU_IA_CLR` reader - "]
pub type PRO_CACHE_MMU_IA_CLR_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_MMU_IA_CLR` writer - "]
pub type PRO_CACHE_MMU_IA_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field(
                "pro_cache_mask_iram0",
                &format_args!("{}", self.pro_cache_mask_iram0().bit()),
            )
            .field(
                "pro_cache_mask_iram1",
                &format_args!("{}", self.pro_cache_mask_iram1().bit()),
            )
            .field(
                "pro_cache_mask_irom0",
                &format_args!("{}", self.pro_cache_mask_irom0().bit()),
            )
            .field(
                "pro_cache_mask_dram1",
                &format_args!("{}", self.pro_cache_mask_dram1().bit()),
            )
            .field(
                "pro_cache_mask_drom0",
                &format_args!("{}", self.pro_cache_mask_drom0().bit()),
            )
            .field(
                "pro_cache_mask_opsdram",
                &format_args!("{}", self.pro_cache_mask_opsdram().bit()),
            )
            .field(
                "pro_cmmu_sram_page_mode",
                &format_args!("{}", self.pro_cmmu_sram_page_mode().bits()),
            )
            .field(
                "pro_cmmu_flash_page_mode",
                &format_args!("{}", self.pro_cmmu_flash_page_mode().bits()),
            )
            .field(
                "pro_cmmu_force_on",
                &format_args!("{}", self.pro_cmmu_force_on().bit()),
            )
            .field("pro_cmmu_pd", &format_args!("{}", self.pro_cmmu_pd().bit()))
            .field(
                "pro_cache_mmu_ia_clr",
                &format_args!("{}", self.pro_cache_mmu_ia_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mask_iram0(&mut self) -> PRO_CACHE_MASK_IRAM0_W<PRO_CACHE_CTRL1_SPEC, 0> {
        PRO_CACHE_MASK_IRAM0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mask_iram1(&mut self) -> PRO_CACHE_MASK_IRAM1_W<PRO_CACHE_CTRL1_SPEC, 1> {
        PRO_CACHE_MASK_IRAM1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mask_irom0(&mut self) -> PRO_CACHE_MASK_IROM0_W<PRO_CACHE_CTRL1_SPEC, 2> {
        PRO_CACHE_MASK_IROM0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mask_dram1(&mut self) -> PRO_CACHE_MASK_DRAM1_W<PRO_CACHE_CTRL1_SPEC, 3> {
        PRO_CACHE_MASK_DRAM1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mask_drom0(&mut self) -> PRO_CACHE_MASK_DROM0_W<PRO_CACHE_CTRL1_SPEC, 4> {
        PRO_CACHE_MASK_DROM0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mask_opsdram(&mut self) -> PRO_CACHE_MASK_OPSDRAM_W<PRO_CACHE_CTRL1_SPEC, 5> {
        PRO_CACHE_MASK_OPSDRAM_W::new(self)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cmmu_sram_page_mode(
        &mut self,
    ) -> PRO_CMMU_SRAM_PAGE_MODE_W<PRO_CACHE_CTRL1_SPEC, 6> {
        PRO_CMMU_SRAM_PAGE_MODE_W::new(self)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cmmu_flash_page_mode(
        &mut self,
    ) -> PRO_CMMU_FLASH_PAGE_MODE_W<PRO_CACHE_CTRL1_SPEC, 9> {
        PRO_CMMU_FLASH_PAGE_MODE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cmmu_force_on(&mut self) -> PRO_CMMU_FORCE_ON_W<PRO_CACHE_CTRL1_SPEC, 11> {
        PRO_CMMU_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cmmu_pd(&mut self) -> PRO_CMMU_PD_W<PRO_CACHE_CTRL1_SPEC, 12> {
        PRO_CMMU_PD_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_mmu_ia_clr(&mut self) -> PRO_CACHE_MMU_IA_CLR_W<PRO_CACHE_CTRL1_SPEC, 13> {
        PRO_CACHE_MMU_IA_CLR_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_CTRL1_SPEC;
impl crate::RegisterSpec for PRO_CACHE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_ctrl1::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_ctrl1::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_CTRL1 to value 0x08ff"]
impl crate::Resettable for PRO_CACHE_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x08ff;
}
