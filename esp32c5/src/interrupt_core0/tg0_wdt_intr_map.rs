#[doc = "Register `TG0_WDT_INTR_MAP` reader"]
pub type R = crate::R<TG0_WDT_INTR_MAP_SPEC>;
#[doc = "Register `TG0_WDT_INTR_MAP` writer"]
pub type W = crate::W<TG0_WDT_INTR_MAP_SPEC>;
#[doc = "Field `TG0_WDT_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type TG0_WDT_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `TG0_WDT_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type TG0_WDT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn tg0_wdt_intr_map(&self) -> TG0_WDT_INTR_MAP_R {
        TG0_WDT_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TG0_WDT_INTR_MAP")
            .field("tg0_wdt_intr_map", &self.tg0_wdt_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn tg0_wdt_intr_map(&mut self) -> TG0_WDT_INTR_MAP_W<TG0_WDT_INTR_MAP_SPEC> {
        TG0_WDT_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "TG0_WDT_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`tg0_wdt_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg0_wdt_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TG0_WDT_INTR_MAP_SPEC;
impl crate::RegisterSpec for TG0_WDT_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tg0_wdt_intr_map::R`](R) reader structure"]
impl crate::Readable for TG0_WDT_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tg0_wdt_intr_map::W`](W) writer structure"]
impl crate::Writable for TG0_WDT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TG0_WDT_INTR_MAP to value 0"]
impl crate::Resettable for TG0_WDT_INTR_MAP_SPEC {}
