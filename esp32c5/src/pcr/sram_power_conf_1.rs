#[doc = "Register `SRAM_POWER_CONF_1` reader"]
pub type R = crate::R<SRAM_POWER_CONF_1_SPEC>;
#[doc = "Register `SRAM_POWER_CONF_1` writer"]
pub type W = crate::W<SRAM_POWER_CONF_1_SPEC>;
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
impl R {
    #[doc = "Bits 0:3 - Set this bit to force power up SRAM"]
    #[inline(always)]
    pub fn sram_force_pu(&self) -> SRAM_FORCE_PU_R {
        SRAM_FORCE_PU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Set this bit to force power down SRAM."]
    #[inline(always)]
    pub fn sram_force_pd(&self) -> SRAM_FORCE_PD_R {
        SRAM_FORCE_PD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 1: Force to open the clock and bypass the gate-clock when accessing the SRAM. 0: A gate-clock will be used when accessing the SRAM."]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_POWER_CONF_1")
            .field("sram_force_pu", &self.sram_force_pu())
            .field("sram_force_pd", &self.sram_force_pd())
            .field("sram_clkgate_force_on", &self.sram_clkgate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Set this bit to force power up SRAM"]
    #[inline(always)]
    pub fn sram_force_pu(&mut self) -> SRAM_FORCE_PU_W<SRAM_POWER_CONF_1_SPEC> {
        SRAM_FORCE_PU_W::new(self, 0)
    }
    #[doc = "Bits 10:13 - Set this bit to force power down SRAM."]
    #[inline(always)]
    pub fn sram_force_pd(&mut self) -> SRAM_FORCE_PD_W<SRAM_POWER_CONF_1_SPEC> {
        SRAM_FORCE_PD_W::new(self, 10)
    }
    #[doc = "Bits 20:23 - 1: Force to open the clock and bypass the gate-clock when accessing the SRAM. 0: A gate-clock will be used when accessing the SRAM."]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W<SRAM_POWER_CONF_1_SPEC> {
        SRAM_CLKGATE_FORCE_ON_W::new(self, 20)
    }
}
#[doc = "HP SRAM/ROM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_power_conf_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_power_conf_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_POWER_CONF_1_SPEC;
impl crate::RegisterSpec for SRAM_POWER_CONF_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_power_conf_1::R`](R) reader structure"]
impl crate::Readable for SRAM_POWER_CONF_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_power_conf_1::W`](W) writer structure"]
impl crate::Writable for SRAM_POWER_CONF_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_POWER_CONF_1 to value 0x0f"]
impl crate::Resettable for SRAM_POWER_CONF_1_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
