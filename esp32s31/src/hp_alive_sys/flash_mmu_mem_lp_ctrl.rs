#[doc = "Register `FLASH_MMU_MEM_LP_CTRL` reader"]
pub type R = crate::R<FLASH_MMU_MEM_LP_CTRL_SPEC>;
#[doc = "Register `FLASH_MMU_MEM_LP_CTRL` writer"]
pub type W = crate::W<FLASH_MMU_MEM_LP_CTRL_SPEC>;
#[doc = "Field `FLASH_MMU_MEM_LP_MODE` reader - Configures flash_mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type FLASH_MMU_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `FLASH_MMU_MEM_LP_MODE` writer - Configures flash_mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type FLASH_MMU_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_MMU_MEM_LP_EN` reader - Set this bit to power down flash_mmu memory."]
pub type FLASH_MMU_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `FLASH_MMU_MEM_LP_EN` writer - Set this bit to power down flash_mmu memory."]
pub type FLASH_MMU_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_MMU_MEM_LP_FORCE_CTRL` reader - Set this bit to force software control flash_mmu memory, disbale hardware control."]
pub type FLASH_MMU_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `FLASH_MMU_MEM_LP_FORCE_CTRL` writer - Set this bit to force software control flash_mmu memory, disbale hardware control."]
pub type FLASH_MMU_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures flash_mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn flash_mmu_mem_lp_mode(&self) -> FLASH_MMU_MEM_LP_MODE_R {
        FLASH_MMU_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down flash_mmu memory."]
    #[inline(always)]
    pub fn flash_mmu_mem_lp_en(&self) -> FLASH_MMU_MEM_LP_EN_R {
        FLASH_MMU_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control flash_mmu memory, disbale hardware control."]
    #[inline(always)]
    pub fn flash_mmu_mem_lp_force_ctrl(&self) -> FLASH_MMU_MEM_LP_FORCE_CTRL_R {
        FLASH_MMU_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_MMU_MEM_LP_CTRL")
            .field("flash_mmu_mem_lp_mode", &self.flash_mmu_mem_lp_mode())
            .field("flash_mmu_mem_lp_en", &self.flash_mmu_mem_lp_en())
            .field(
                "flash_mmu_mem_lp_force_ctrl",
                &self.flash_mmu_mem_lp_force_ctrl(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures flash_mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn flash_mmu_mem_lp_mode(
        &mut self,
    ) -> FLASH_MMU_MEM_LP_MODE_W<'_, FLASH_MMU_MEM_LP_CTRL_SPEC> {
        FLASH_MMU_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down flash_mmu memory."]
    #[inline(always)]
    pub fn flash_mmu_mem_lp_en(&mut self) -> FLASH_MMU_MEM_LP_EN_W<'_, FLASH_MMU_MEM_LP_CTRL_SPEC> {
        FLASH_MMU_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control flash_mmu memory, disbale hardware control."]
    #[inline(always)]
    pub fn flash_mmu_mem_lp_force_ctrl(
        &mut self,
    ) -> FLASH_MMU_MEM_LP_FORCE_CTRL_W<'_, FLASH_MMU_MEM_LP_CTRL_SPEC> {
        FLASH_MMU_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_mmu_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_mmu_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_MMU_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_MMU_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_mmu_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for FLASH_MMU_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_mmu_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for FLASH_MMU_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_MMU_MEM_LP_CTRL to value 0"]
impl crate::Resettable for FLASH_MMU_MEM_LP_CTRL_SPEC {}
