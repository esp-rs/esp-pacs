///Register `TG1_WDT_INTR_MAP` reader
pub type R = crate::R<TG1_WDT_INTR_MAP_SPEC>;
///Register `TG1_WDT_INTR_MAP` writer
pub type W = crate::W<TG1_WDT_INTR_MAP_SPEC>;
///Field `TG1_WDT_INTR_MAP` reader - CORE0_TG1_WDT_INTR mapping register
pub type TG1_WDT_INTR_MAP_R = crate::FieldReader;
///Field `TG1_WDT_INTR_MAP` writer - CORE0_TG1_WDT_INTR mapping register
pub type TG1_WDT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - CORE0_TG1_WDT_INTR mapping register
    #[inline(always)]
    pub fn tg1_wdt_intr_map(&self) -> TG1_WDT_INTR_MAP_R {
        TG1_WDT_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TG1_WDT_INTR_MAP")
            .field("tg1_wdt_intr_map", &self.tg1_wdt_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - CORE0_TG1_WDT_INTR mapping register
    #[inline(always)]
    #[must_use]
    pub fn tg1_wdt_intr_map(&mut self) -> TG1_WDT_INTR_MAP_W<TG1_WDT_INTR_MAP_SPEC> {
        TG1_WDT_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`tg1_wdt_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg1_wdt_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TG1_WDT_INTR_MAP_SPEC;
impl crate::RegisterSpec for TG1_WDT_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tg1_wdt_intr_map::R`](R) reader structure
impl crate::Readable for TG1_WDT_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`tg1_wdt_intr_map::W`](W) writer structure
impl crate::Writable for TG1_WDT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TG1_WDT_INTR_MAP to value 0
impl crate::Resettable for TG1_WDT_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
