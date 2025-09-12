#[doc = "Register `CLKGATE_FORCE_ON` reader"]
pub type R = crate::R<CLKGATE_FORCE_ON_SPEC>;
#[doc = "Register `CLKGATE_FORCE_ON` writer"]
pub type W = crate::W<CLKGATE_FORCE_ON_SPEC>;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` reader - ******* Description ***********"]
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
#[doc = "Field `ROM_CLKGATE_FORCE_ON` writer - ******* Description ***********"]
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` reader - ******* Description ***********"]
pub type SRAM_CLKGATE_FORCE_ON_R = crate::FieldReader<u16>;
#[doc = "Field `SRAM_CLKGATE_FORCE_ON` writer - ******* Description ***********"]
pub type SRAM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKGATE_FORCE_ON")
            .field("rom_clkgate_force_on", &self.rom_clkgate_force_on())
            .field("sram_clkgate_force_on", &self.sram_clkgate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - ******* Description ***********"]
    #[inline(always)]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<'_, CLKGATE_FORCE_ON_SPEC> {
        ROM_CLKGATE_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bits 3:13 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W<'_, CLKGATE_FORCE_ON_SPEC> {
        SRAM_CLKGATE_FORCE_ON_W::new(self, 3)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`clkgate_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkgate_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKGATE_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLKGATE_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_force_on::R`](R) reader structure"]
impl crate::Readable for CLKGATE_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkgate_force_on::W`](W) writer structure"]
impl crate::Writable for CLKGATE_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKGATE_FORCE_ON to value 0x3fff"]
impl crate::Resettable for CLKGATE_FORCE_ON_SPEC {
    const RESET_VALUE: u32 = 0x3fff;
}
