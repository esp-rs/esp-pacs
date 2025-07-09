#[doc = "Register `WIFI_PWR_INTR_MAP` reader"]
pub type R = crate::R<WIFI_PWR_INTR_MAP_SPEC>;
#[doc = "Register `WIFI_PWR_INTR_MAP` writer"]
pub type W = crate::W<WIFI_PWR_INTR_MAP_SPEC>;
#[doc = "Field `WIFI_PWR_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type WIFI_PWR_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `WIFI_PWR_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type WIFI_PWR_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn wifi_pwr_intr_map(&self) -> WIFI_PWR_INTR_MAP_R {
        WIFI_PWR_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_PWR_INTR_MAP")
            .field("wifi_pwr_intr_map", &self.wifi_pwr_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn wifi_pwr_intr_map(&mut self) -> WIFI_PWR_INTR_MAP_W<WIFI_PWR_INTR_MAP_SPEC> {
        WIFI_PWR_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "WIFI_PWR_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_pwr_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_pwr_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PWR_INTR_MAP_SPEC;
impl crate::RegisterSpec for WIFI_PWR_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pwr_intr_map::R`](R) reader structure"]
impl crate::Readable for WIFI_PWR_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pwr_intr_map::W`](W) writer structure"]
impl crate::Writable for WIFI_PWR_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_PWR_INTR_MAP to value 0"]
impl crate::Resettable for WIFI_PWR_INTR_MAP_SPEC {}
