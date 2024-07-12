#[doc = "Register `SRAM_POWER_CONF_0` reader"]
pub type R = crate::R<SRAM_POWER_CONF_0_SPEC>;
#[doc = "Register `SRAM_POWER_CONF_0` writer"]
pub type W = crate::W<SRAM_POWER_CONF_0_SPEC>;
#[doc = "Field `ROM_FORCE_PU` reader - Set this bit to force power up ROM"]
pub type ROM_FORCE_PU_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PU` writer - Set this bit to force power up ROM"]
pub type ROM_FORCE_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROM_FORCE_PD` reader - Set this bit to force power down ROM."]
pub type ROM_FORCE_PD_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PD` writer - Set this bit to force power down ROM."]
pub type ROM_FORCE_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 13:14 - Set this bit to force power up ROM"]
    #[inline(always)]
    pub fn rom_force_pu(&self) -> ROM_FORCE_PU_R {
        ROM_FORCE_PU_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Set this bit to force power down ROM."]
    #[inline(always)]
    pub fn rom_force_pd(&self) -> ROM_FORCE_PD_R {
        ROM_FORCE_PD_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_POWER_CONF_0")
            .field("rom_force_pu", &self.rom_force_pu())
            .field("rom_force_pd", &self.rom_force_pd())
            .field("rom_clkgate_force_on", &self.rom_clkgate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bits 13:14 - Set this bit to force power up ROM"]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pu(&mut self) -> ROM_FORCE_PU_W<SRAM_POWER_CONF_0_SPEC> {
        ROM_FORCE_PU_W::new(self, 13)
    }
    #[doc = "Bits 15:16 - Set this bit to force power down ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pd(&mut self) -> ROM_FORCE_PD_W<SRAM_POWER_CONF_0_SPEC> {
        ROM_FORCE_PD_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<SRAM_POWER_CONF_0_SPEC> {
        ROM_CLKGATE_FORCE_ON_W::new(self, 17)
    }
}
#[doc = "HP SRAM/ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_power_conf_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_power_conf_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_POWER_CONF_0_SPEC;
impl crate::RegisterSpec for SRAM_POWER_CONF_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_power_conf_0::R`](R) reader structure"]
impl crate::Readable for SRAM_POWER_CONF_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_power_conf_0::W`](W) writer structure"]
impl crate::Writable for SRAM_POWER_CONF_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_POWER_CONF_0 to value 0x6000"]
impl crate::Resettable for SRAM_POWER_CONF_0_SPEC {
    const RESET_VALUE: u32 = 0x6000;
}
