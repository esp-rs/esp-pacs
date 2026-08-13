#[doc = "Register `HP_MEM_CTRL` reader"]
pub type R = crate::R<HP_MEM_CTRL_SPEC>;
#[doc = "Register `HP_MEM_CTRL` writer"]
pub type W = crate::W<HP_MEM_CTRL_SPEC>;
#[doc = "Field `MODEM_MEM_LP_MODE` reader - Configures modem memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type MODEM_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `MODEM_MEM_LP_MODE` writer - Configures modem memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type MODEM_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODEM_MEM_LP_EN` reader - Set this bit to power down modem memory."]
pub type MODEM_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `MODEM_MEM_LP_EN` writer - Set this bit to power down modem memory."]
pub type MODEM_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_MEM_FORCE_CTRL` reader - Set this bit to force software control modem memory, disbale hardware control."]
pub type MODEM_MEM_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `MODEM_MEM_FORCE_CTRL` writer - Set this bit to force software control modem memory, disbale hardware control."]
pub type MODEM_MEM_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_MEM_LP_MODE` reader - Configures mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type MMU_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `MMU_MEM_LP_MODE` writer - Configures mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type MMU_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MMU_MEM_LP_EN` reader - Set this bit to power down mmu memory."]
pub type MMU_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `MMU_MEM_LP_EN` writer - Set this bit to power down mmu memory."]
pub type MMU_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMU_MEM_FORCE_CTRL` reader - Set this bit to force software control mmu memory, disbale hardware control."]
pub type MMU_MEM_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `MMU_MEM_FORCE_CTRL` writer - Set this bit to force software control mmu memory, disbale hardware control."]
pub type MMU_MEM_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SRAM_MEM_LP_MODE` reader - Configures hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_SRAM_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HP_SRAM_MEM_LP_MODE` writer - Configures hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type HP_SRAM_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SRAM_MEM_LP_EN` reader - Set this bit to power down hp_sram memory."]
pub type HP_SRAM_MEM_LP_EN_R = crate::FieldReader;
#[doc = "Field `HP_SRAM_MEM_LP_EN` writer - Set this bit to power down hp_sram memory."]
pub type HP_SRAM_MEM_LP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HP_SRAM_MEM_FORCE_CTRL` reader - Set this bit to force software control hp_sram memory, disbale hardware control."]
pub type HP_SRAM_MEM_FORCE_CTRL_R = crate::FieldReader;
#[doc = "Field `HP_SRAM_MEM_FORCE_CTRL` writer - Set this bit to force software control hp_sram memory, disbale hardware control."]
pub type HP_SRAM_MEM_FORCE_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - Configures modem memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn modem_mem_lp_mode(&self) -> MODEM_MEM_LP_MODE_R {
        MODEM_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down modem memory."]
    #[inline(always)]
    pub fn modem_mem_lp_en(&self) -> MODEM_MEM_LP_EN_R {
        MODEM_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control modem memory, disbale hardware control."]
    #[inline(always)]
    pub fn modem_mem_force_ctrl(&self) -> MODEM_MEM_FORCE_CTRL_R {
        MODEM_MEM_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Configures mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn mmu_mem_lp_mode(&self) -> MMU_MEM_LP_MODE_R {
        MMU_MEM_LP_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to power down mmu memory."]
    #[inline(always)]
    pub fn mmu_mem_lp_en(&self) -> MMU_MEM_LP_EN_R {
        MMU_MEM_LP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to force software control mmu memory, disbale hardware control."]
    #[inline(always)]
    pub fn mmu_mem_force_ctrl(&self) -> MMU_MEM_FORCE_CTRL_R {
        MMU_MEM_FORCE_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Configures hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_sram_mem_lp_mode(&self) -> HP_SRAM_MEM_LP_MODE_R {
        HP_SRAM_MEM_LP_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:24 - Set this bit to power down hp_sram memory."]
    #[inline(always)]
    pub fn hp_sram_mem_lp_en(&self) -> HP_SRAM_MEM_LP_EN_R {
        HP_SRAM_MEM_LP_EN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - Set this bit to force software control hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_sram_mem_force_ctrl(&self) -> HP_SRAM_MEM_FORCE_CTRL_R {
        HP_SRAM_MEM_FORCE_CTRL_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MEM_CTRL")
            .field("modem_mem_lp_mode", &self.modem_mem_lp_mode())
            .field("modem_mem_lp_en", &self.modem_mem_lp_en())
            .field("modem_mem_force_ctrl", &self.modem_mem_force_ctrl())
            .field("mmu_mem_lp_mode", &self.mmu_mem_lp_mode())
            .field("mmu_mem_lp_en", &self.mmu_mem_lp_en())
            .field("mmu_mem_force_ctrl", &self.mmu_mem_force_ctrl())
            .field("hp_sram_mem_lp_mode", &self.hp_sram_mem_lp_mode())
            .field("hp_sram_mem_lp_en", &self.hp_sram_mem_lp_en())
            .field("hp_sram_mem_force_ctrl", &self.hp_sram_mem_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures modem memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn modem_mem_lp_mode(&mut self) -> MODEM_MEM_LP_MODE_W<'_, HP_MEM_CTRL_SPEC> {
        MODEM_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down modem memory."]
    #[inline(always)]
    pub fn modem_mem_lp_en(&mut self) -> MODEM_MEM_LP_EN_W<'_, HP_MEM_CTRL_SPEC> {
        MODEM_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control modem memory, disbale hardware control."]
    #[inline(always)]
    pub fn modem_mem_force_ctrl(&mut self) -> MODEM_MEM_FORCE_CTRL_W<'_, HP_MEM_CTRL_SPEC> {
        MODEM_MEM_FORCE_CTRL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Configures mmu memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn mmu_mem_lp_mode(&mut self) -> MMU_MEM_LP_MODE_W<'_, HP_MEM_CTRL_SPEC> {
        MMU_MEM_LP_MODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to power down mmu memory."]
    #[inline(always)]
    pub fn mmu_mem_lp_en(&mut self) -> MMU_MEM_LP_EN_W<'_, HP_MEM_CTRL_SPEC> {
        MMU_MEM_LP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to force software control mmu memory, disbale hardware control."]
    #[inline(always)]
    pub fn mmu_mem_force_ctrl(&mut self) -> MMU_MEM_FORCE_CTRL_W<'_, HP_MEM_CTRL_SPEC> {
        MMU_MEM_FORCE_CTRL_W::new(self, 7)
    }
    #[doc = "Bits 16:17 - Configures hp_sram memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hp_sram_mem_lp_mode(&mut self) -> HP_SRAM_MEM_LP_MODE_W<'_, HP_MEM_CTRL_SPEC> {
        HP_SRAM_MEM_LP_MODE_W::new(self, 16)
    }
    #[doc = "Bits 18:24 - Set this bit to power down hp_sram memory."]
    #[inline(always)]
    pub fn hp_sram_mem_lp_en(&mut self) -> HP_SRAM_MEM_LP_EN_W<'_, HP_MEM_CTRL_SPEC> {
        HP_SRAM_MEM_LP_EN_W::new(self, 18)
    }
    #[doc = "Bits 25:31 - Set this bit to force software control hp_sram memory, disbale hardware control."]
    #[inline(always)]
    pub fn hp_sram_mem_force_ctrl(&mut self) -> HP_SRAM_MEM_FORCE_CTRL_W<'_, HP_MEM_CTRL_SPEC> {
        HP_SRAM_MEM_FORCE_CTRL_W::new(self, 25)
    }
}
#[doc = "configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MEM_CTRL_SPEC;
impl crate::RegisterSpec for HP_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_mem_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_mem_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MEM_CTRL to value 0"]
impl crate::Resettable for HP_MEM_CTRL_SPEC {}
