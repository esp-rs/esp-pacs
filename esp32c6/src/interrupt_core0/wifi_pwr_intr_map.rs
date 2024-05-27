///Register `WIFI_PWR_INTR_MAP` reader
pub type R = crate::R<WIFI_PWR_INTR_MAP_SPEC>;
///Register `WIFI_PWR_INTR_MAP` writer
pub type W = crate::W<WIFI_PWR_INTR_MAP_SPEC>;
///Field `WIFI_PWR_INTR_MAP` reader - Need add description
pub type WIFI_PWR_INTR_MAP_R = crate::FieldReader;
///Field `WIFI_PWR_INTR_MAP` writer - Need add description
pub type WIFI_PWR_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    pub fn wifi_pwr_intr_map(&self) -> WIFI_PWR_INTR_MAP_R {
        WIFI_PWR_INTR_MAP_R::new((self.bits & 0x1f) as u8)
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
    ///Bits 0:4 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn wifi_pwr_intr_map(&mut self) -> WIFI_PWR_INTR_MAP_W<WIFI_PWR_INTR_MAP_SPEC> {
        WIFI_PWR_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`wifi_pwr_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pwr_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WIFI_PWR_INTR_MAP_SPEC;
impl crate::RegisterSpec for WIFI_PWR_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wifi_pwr_intr_map::R`](R) reader structure
impl crate::Readable for WIFI_PWR_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`wifi_pwr_intr_map::W`](W) writer structure
impl crate::Writable for WIFI_PWR_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WIFI_PWR_INTR_MAP to value 0
impl crate::Resettable for WIFI_PWR_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
