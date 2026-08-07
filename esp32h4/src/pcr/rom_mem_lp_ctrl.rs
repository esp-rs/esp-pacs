#[doc = "Register `ROM_MEM_LP_CTRL` reader"]
pub type R = crate::R<ROM_MEM_LP_CTRL_SPEC>;
#[doc = "Register `ROM_MEM_LP_CTRL` writer"]
pub type W = crate::W<ROM_MEM_LP_CTRL_SPEC>;
#[doc = "Field `INTERNAL_ROM_MEM_LP_MODE` reader - Configures rom low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type INTERNAL_ROM_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_ROM_MEM_LP_MODE` writer - Configures rom low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type INTERNAL_ROM_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERNAL_ROM_MEM_LP_EN` reader - Set this bit to power down rom."]
pub type INTERNAL_ROM_MEM_LP_EN_R = crate::FieldReader;
#[doc = "Field `INTERNAL_ROM_MEM_LP_EN` writer - Set this bit to power down rom."]
pub type INTERNAL_ROM_MEM_LP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERNAL_ROM_MEM_FORCE_CTRL` reader - Set this bit to force software control rom, disbale hardware control."]
pub type INTERNAL_ROM_MEM_FORCE_CTRL_R = crate::FieldReader;
#[doc = "Field `INTERNAL_ROM_MEM_FORCE_CTRL` writer - Set this bit to force software control rom, disbale hardware control."]
pub type INTERNAL_ROM_MEM_FORCE_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Configures rom low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn internal_rom_mem_lp_mode(&self) -> INTERNAL_ROM_MEM_LP_MODE_R {
        INTERNAL_ROM_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set this bit to power down rom."]
    #[inline(always)]
    pub fn internal_rom_mem_lp_en(&self) -> INTERNAL_ROM_MEM_LP_EN_R {
        INTERNAL_ROM_MEM_LP_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set this bit to force software control rom, disbale hardware control."]
    #[inline(always)]
    pub fn internal_rom_mem_force_ctrl(&self) -> INTERNAL_ROM_MEM_FORCE_CTRL_R {
        INTERNAL_ROM_MEM_FORCE_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new(((self.bits >> 6) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_MEM_LP_CTRL")
            .field("internal_rom_mem_lp_mode", &self.internal_rom_mem_lp_mode())
            .field("internal_rom_mem_lp_en", &self.internal_rom_mem_lp_en())
            .field(
                "internal_rom_mem_force_ctrl",
                &self.internal_rom_mem_force_ctrl(),
            )
            .field("rom_clkgate_force_on", &self.rom_clkgate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures rom low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn internal_rom_mem_lp_mode(
        &mut self,
    ) -> INTERNAL_ROM_MEM_LP_MODE_W<'_, ROM_MEM_LP_CTRL_SPEC> {
        INTERNAL_ROM_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set this bit to power down rom."]
    #[inline(always)]
    pub fn internal_rom_mem_lp_en(&mut self) -> INTERNAL_ROM_MEM_LP_EN_W<'_, ROM_MEM_LP_CTRL_SPEC> {
        INTERNAL_ROM_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Set this bit to force software control rom, disbale hardware control."]
    #[inline(always)]
    pub fn internal_rom_mem_force_ctrl(
        &mut self,
    ) -> INTERNAL_ROM_MEM_FORCE_CTRL_W<'_, ROM_MEM_LP_CTRL_SPEC> {
        INTERNAL_ROM_MEM_FORCE_CTRL_W::new(self, 4)
    }
    #[doc = "Bits 6:8 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<'_, ROM_MEM_LP_CTRL_SPEC> {
        ROM_CLKGATE_FORCE_ON_W::new(self, 6)
    }
}
#[doc = "rom power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for ROM_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for ROM_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for ROM_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_MEM_LP_CTRL to value 0"]
impl crate::Resettable for ROM_MEM_LP_CTRL_SPEC {}
