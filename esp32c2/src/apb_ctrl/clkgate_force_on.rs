///Register `CLKGATE_FORCE_ON` reader
pub type R = crate::R<CLKGATE_FORCE_ON_SPEC>;
///Register `CLKGATE_FORCE_ON` writer
pub type W = crate::W<CLKGATE_FORCE_ON_SPEC>;
///Field `ROM_CLKGATE_FORCE_ON` reader - Set the bit to 1 to force rom always have clock, for low power can clear to 0 then only when have access the rom have clock
pub type ROM_CLKGATE_FORCE_ON_R = crate::FieldReader;
///Field `ROM_CLKGATE_FORCE_ON` writer - Set the bit to 1 to force rom always have clock, for low power can clear to 0 then only when have access the rom have clock
pub type ROM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SRAM_CLKGATE_FORCE_ON` reader - Set the bit to 1 to force sram always have clock, for low power can clear to 0 then only when have access the sram have clock
pub type SRAM_CLKGATE_FORCE_ON_R = crate::FieldReader;
///Field `SRAM_CLKGATE_FORCE_ON` writer - Set the bit to 1 to force sram always have clock, for low power can clear to 0 then only when have access the sram have clock
pub type SRAM_CLKGATE_FORCE_ON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - Set the bit to 1 to force rom always have clock, for low power can clear to 0 then only when have access the rom have clock
    #[inline(always)]
    pub fn rom_clkgate_force_on(&self) -> ROM_CLKGATE_FORCE_ON_R {
        ROM_CLKGATE_FORCE_ON_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:6 - Set the bit to 1 to force sram always have clock, for low power can clear to 0 then only when have access the sram have clock
    #[inline(always)]
    pub fn sram_clkgate_force_on(&self) -> SRAM_CLKGATE_FORCE_ON_R {
        SRAM_CLKGATE_FORCE_ON_R::new(((self.bits >> 3) & 0x0f) as u8)
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
    ///Bits 0:2 - Set the bit to 1 to force rom always have clock, for low power can clear to 0 then only when have access the rom have clock
    #[inline(always)]
    #[must_use]
    pub fn rom_clkgate_force_on(&mut self) -> ROM_CLKGATE_FORCE_ON_W<CLKGATE_FORCE_ON_SPEC> {
        ROM_CLKGATE_FORCE_ON_W::new(self, 0)
    }
    ///Bits 3:6 - Set the bit to 1 to force sram always have clock, for low power can clear to 0 then only when have access the sram have clock
    #[inline(always)]
    #[must_use]
    pub fn sram_clkgate_force_on(&mut self) -> SRAM_CLKGATE_FORCE_ON_W<CLKGATE_FORCE_ON_SPEC> {
        SRAM_CLKGATE_FORCE_ON_W::new(self, 3)
    }
}
/**Memory power configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`clkgate_force_on::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_force_on::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLKGATE_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLKGATE_FORCE_ON_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clkgate_force_on::R`](R) reader structure
impl crate::Readable for CLKGATE_FORCE_ON_SPEC {}
///`write(|w| ..)` method takes [`clkgate_force_on::W`](W) writer structure
impl crate::Writable for CLKGATE_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLKGATE_FORCE_ON to value 0x7f
impl crate::Resettable for CLKGATE_FORCE_ON_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
