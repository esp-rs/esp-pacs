#[doc = "Register `SRAM_POWER_CONF` reader"]
pub type R = crate::R<SRAM_POWER_CONF_SPEC>;
#[doc = "Register `SRAM_POWER_CONF` writer"]
pub type W = crate::W<SRAM_POWER_CONF_SPEC>;
#[doc = "Field `SRAM_FORCE_PU` reader - Set this bit to force power up SRAM"]
pub type SRAM_FORCE_PU_R = crate::FieldReader;
#[doc = "Field `SRAM_FORCE_PU` writer - Set this bit to force power up SRAM"]
pub type SRAM_FORCE_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRAM_FORCE_PD` reader - Set this bit to force power down SRAM."]
pub type SRAM_FORCE_PD_R = crate::FieldReader;
#[doc = "Field `SRAM_FORCE_PD` writer - Set this bit to force power down SRAM."]
pub type SRAM_FORCE_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` reader - 1: Force to open the clock and bypass the gate-clock when accessing the SRAM. 0: A gate-clock will be used when accessing the SRAM."]
pub type SRAM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` writer - 1: Force to open the clock and bypass the gate-clock when accessing the SRAM. 0: A gate-clock will be used when accessing the SRAM."]
pub type SRAM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROM_FORCE_PU` reader - Set this bit to force power up ROM"]
pub type ROM_FORCE_PU_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PU` writer - Set this bit to force power up ROM"]
pub type ROM_FORCE_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ROM_FORCE_PD` reader - Set this bit to force power down ROM."]
pub type ROM_FORCE_PD_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PD` writer - Set this bit to force power down ROM."]
pub type ROM_FORCE_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Set this bit to force power up SRAM"]
    #[inline(always)]
    pub fn sram_force_pu(&self) -> SRAM_FORCE_PU_R {
        SRAM_FORCE_PU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Set this bit to force power down SRAM."]
    #[inline(always)]
    pub fn sram_force_pd(&self) -> SRAM_FORCE_PD_R {
        SRAM_FORCE_PD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 1: Force to open the clock and bypass the gate-clock when accessing the SRAM. 0: A gate-clock will be used when accessing the SRAM."]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Set this bit to force power up ROM"]
    #[inline(always)]
    pub fn rom_force_pu(&self) -> ROM_FORCE_PU_R {
        ROM_FORCE_PU_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Set this bit to force power down ROM."]
    #[inline(always)]
    pub fn rom_force_pd(&self) -> ROM_FORCE_PD_R {
        ROM_FORCE_PD_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new(((self.bits >> 18) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_POWER_CONF")
            .field(
                "sram_force_pu",
                &format_args!("{}", self.sram_force_pu().bits()),
            )
            .field(
                "sram_force_pd",
                &format_args!("{}", self.sram_force_pd().bits()),
            )
            .field(
                "sram_clkgate_force_on",
                &format_args!("{}", self.sram_clkgate_force_on().bits()),
            )
            .field(
                "rom_force_pu",
                &format_args!("{}", self.rom_force_pu().bits()),
            )
            .field(
                "rom_force_pd",
                &format_args!("{}", self.rom_force_pd().bits()),
            )
            .field(
                "rom_clkgate_force_on",
                &format_args!("{}", self.rom_clkgate_force_on().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_POWER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set this bit to force power up SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn sram_force_pu(&mut self) -> SRAM_FORCE_PU_W<SRAM_POWER_CONF_SPEC> {
        SRAM_FORCE_PU_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Set this bit to force power down SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn sram_force_pd(&mut self) -> SRAM_FORCE_PD_W<SRAM_POWER_CONF_SPEC> {
        SRAM_FORCE_PD_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 1: Force to open the clock and bypass the gate-clock when accessing the SRAM. 0: A gate-clock will be used when accessing the SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W<SRAM_POWER_CONF_SPEC> {
        SRAM_CLKGATE_FORCE_ON_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Set this bit to force power up ROM"]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pu(&mut self) -> ROM_FORCE_PU_W<SRAM_POWER_CONF_SPEC> {
        ROM_FORCE_PU_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Set this bit to force power down ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pd(&mut self) -> ROM_FORCE_PD_W<SRAM_POWER_CONF_SPEC> {
        ROM_FORCE_PD_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - 1: Force to open the clock and bypass the gate-clock when accessing the ROM. 0: A gate-clock will be used when accessing the ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<SRAM_POWER_CONF_SPEC> {
        ROM_CLKGATE_FORCE_ON_W::new(self, 18)
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
#[doc = "HP SRAM/ROM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_power_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_power_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_POWER_CONF_SPEC;
impl crate::RegisterSpec for SRAM_POWER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_power_conf::R`](R) reader structure"]
impl crate::Readable for SRAM_POWER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_power_conf::W`](W) writer structure"]
impl crate::Writable for SRAM_POWER_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_POWER_CONF to value 0x700f"]
impl crate::Resettable for SRAM_POWER_CONF_SPEC {
    const RESET_VALUE: u32 = 0x700f;
}
