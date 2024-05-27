///Register `MEM_POWER_DOWN` reader
pub type R = crate::R<MEM_POWER_DOWN_SPEC>;
///Register `MEM_POWER_DOWN` writer
pub type W = crate::W<MEM_POWER_DOWN_SPEC>;
///Field `ROM_POWER_DOWN` reader - Set 1 to let rom power down
pub type ROM_POWER_DOWN_R = crate::FieldReader;
///Field `ROM_POWER_DOWN` writer - Set 1 to let rom power down
pub type ROM_POWER_DOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SRAM_POWER_DOWN` reader - Set 1 to let sram power down
pub type SRAM_POWER_DOWN_R = crate::FieldReader;
///Field `SRAM_POWER_DOWN` writer - Set 1 to let sram power down
pub type SRAM_POWER_DOWN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - Set 1 to let rom power down
    #[inline(always)]
    pub fn rom_power_down(&self) -> ROM_POWER_DOWN_R {
        ROM_POWER_DOWN_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:6 - Set 1 to let sram power down
    #[inline(always)]
    pub fn sram_power_down(&self) -> SRAM_POWER_DOWN_R {
        SRAM_POWER_DOWN_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_POWER_DOWN")
            .field("rom_power_down", &self.rom_power_down())
            .field("sram_power_down", &self.sram_power_down())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Set 1 to let rom power down
    #[inline(always)]
    #[must_use]
    pub fn rom_power_down(&mut self) -> ROM_POWER_DOWN_W<MEM_POWER_DOWN_SPEC> {
        ROM_POWER_DOWN_W::new(self, 0)
    }
    ///Bits 3:6 - Set 1 to let sram power down
    #[inline(always)]
    #[must_use]
    pub fn sram_power_down(&mut self) -> SRAM_POWER_DOWN_W<MEM_POWER_DOWN_SPEC> {
        SRAM_POWER_DOWN_W::new(self, 3)
    }
}
/**Memory power configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`mem_power_down::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_power_down::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_POWER_DOWN_SPEC;
impl crate::RegisterSpec for MEM_POWER_DOWN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_power_down::R`](R) reader structure
impl crate::Readable for MEM_POWER_DOWN_SPEC {}
///`write(|w| ..)` method takes [`mem_power_down::W`](W) writer structure
impl crate::Writable for MEM_POWER_DOWN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_POWER_DOWN to value 0x7f
impl crate::Resettable for MEM_POWER_DOWN_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
