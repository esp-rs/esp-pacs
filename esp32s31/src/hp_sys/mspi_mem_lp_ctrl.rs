#[doc = "Register `MSPI_MEM_LP_CTRL` reader"]
pub type R = crate::R<MSPI_MEM_LP_CTRL_SPEC>;
#[doc = "Register `MSPI_MEM_LP_CTRL` writer"]
pub type W = crate::W<MSPI_MEM_LP_CTRL_SPEC>;
#[doc = "Field `PSRAM_MSPI_MEM_LP_MODE` reader - Configures psram_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type PSRAM_MSPI_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `PSRAM_MSPI_MEM_LP_MODE` writer - Configures psram_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type PSRAM_MSPI_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSRAM_MSPI_MEM_LP_EN` reader - Set this bit to power down psram_mspi memory."]
pub type PSRAM_MSPI_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_MSPI_MEM_LP_EN` writer - Set this bit to power down psram_mspi memory."]
pub type PSRAM_MSPI_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_MSPI_MEM_LP_FORCE_CTRL` reader - Set this bit to force software control psram_mspi memory, disbale hardware control."]
pub type PSRAM_MSPI_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `PSRAM_MSPI_MEM_LP_FORCE_CTRL` writer - Set this bit to force software control psram_mspi memory, disbale hardware control."]
pub type PSRAM_MSPI_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_MSPI_MEM_LP_MODE` reader - Configures flash_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type FLASH_MSPI_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `FLASH_MSPI_MEM_LP_MODE` writer - Configures flash_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type FLASH_MSPI_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_MSPI_MEM_LP_EN` reader - Set this bit to power down flash_mspi memory."]
pub type FLASH_MSPI_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `FLASH_MSPI_MEM_LP_EN` writer - Set this bit to power down flash_mspi memory."]
pub type FLASH_MSPI_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_MSPI_MEM_LP_FORCE_CTRL` reader - Set this bit to force software control flash_mspi memory, disbale hardware control."]
pub type FLASH_MSPI_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `FLASH_MSPI_MEM_LP_FORCE_CTRL` writer - Set this bit to force software control flash_mspi memory, disbale hardware control."]
pub type FLASH_MSPI_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures psram_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn psram_mspi_mem_lp_mode(&self) -> PSRAM_MSPI_MEM_LP_MODE_R {
        PSRAM_MSPI_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down psram_mspi memory."]
    #[inline(always)]
    pub fn psram_mspi_mem_lp_en(&self) -> PSRAM_MSPI_MEM_LP_EN_R {
        PSRAM_MSPI_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control psram_mspi memory, disbale hardware control."]
    #[inline(always)]
    pub fn psram_mspi_mem_lp_force_ctrl(&self) -> PSRAM_MSPI_MEM_LP_FORCE_CTRL_R {
        PSRAM_MSPI_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Configures flash_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn flash_mspi_mem_lp_mode(&self) -> FLASH_MSPI_MEM_LP_MODE_R {
        FLASH_MSPI_MEM_LP_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to power down flash_mspi memory."]
    #[inline(always)]
    pub fn flash_mspi_mem_lp_en(&self) -> FLASH_MSPI_MEM_LP_EN_R {
        FLASH_MSPI_MEM_LP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to force software control flash_mspi memory, disbale hardware control."]
    #[inline(always)]
    pub fn flash_mspi_mem_lp_force_ctrl(&self) -> FLASH_MSPI_MEM_LP_FORCE_CTRL_R {
        FLASH_MSPI_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPI_MEM_LP_CTRL")
            .field("psram_mspi_mem_lp_mode", &self.psram_mspi_mem_lp_mode())
            .field("psram_mspi_mem_lp_en", &self.psram_mspi_mem_lp_en())
            .field(
                "psram_mspi_mem_lp_force_ctrl",
                &self.psram_mspi_mem_lp_force_ctrl(),
            )
            .field("flash_mspi_mem_lp_mode", &self.flash_mspi_mem_lp_mode())
            .field("flash_mspi_mem_lp_en", &self.flash_mspi_mem_lp_en())
            .field(
                "flash_mspi_mem_lp_force_ctrl",
                &self.flash_mspi_mem_lp_force_ctrl(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures psram_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn psram_mspi_mem_lp_mode(
        &mut self,
    ) -> PSRAM_MSPI_MEM_LP_MODE_W<'_, MSPI_MEM_LP_CTRL_SPEC> {
        PSRAM_MSPI_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down psram_mspi memory."]
    #[inline(always)]
    pub fn psram_mspi_mem_lp_en(&mut self) -> PSRAM_MSPI_MEM_LP_EN_W<'_, MSPI_MEM_LP_CTRL_SPEC> {
        PSRAM_MSPI_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control psram_mspi memory, disbale hardware control."]
    #[inline(always)]
    pub fn psram_mspi_mem_lp_force_ctrl(
        &mut self,
    ) -> PSRAM_MSPI_MEM_LP_FORCE_CTRL_W<'_, MSPI_MEM_LP_CTRL_SPEC> {
        PSRAM_MSPI_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Configures flash_mspi memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn flash_mspi_mem_lp_mode(
        &mut self,
    ) -> FLASH_MSPI_MEM_LP_MODE_W<'_, MSPI_MEM_LP_CTRL_SPEC> {
        FLASH_MSPI_MEM_LP_MODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to power down flash_mspi memory."]
    #[inline(always)]
    pub fn flash_mspi_mem_lp_en(&mut self) -> FLASH_MSPI_MEM_LP_EN_W<'_, MSPI_MEM_LP_CTRL_SPEC> {
        FLASH_MSPI_MEM_LP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to force software control flash_mspi memory, disbale hardware control."]
    #[inline(always)]
    pub fn flash_mspi_mem_lp_force_ctrl(
        &mut self,
    ) -> FLASH_MSPI_MEM_LP_FORCE_CTRL_W<'_, MSPI_MEM_LP_CTRL_SPEC> {
        FLASH_MSPI_MEM_LP_FORCE_CTRL_W::new(self, 7)
    }
}
#[doc = "HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPI_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for MSPI_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspi_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for MSPI_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspi_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for MSPI_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSPI_MEM_LP_CTRL to value 0x22"]
impl crate::Resettable for MSPI_MEM_LP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x22;
}
