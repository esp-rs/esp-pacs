#[doc = "Register `MAC_INTR_MAP` reader"]
pub type R = crate::R<MAC_INTR_MAP_SPEC>;
#[doc = "Register `MAC_INTR_MAP` writer"]
pub type W = crate::W<MAC_INTR_MAP_SPEC>;
#[doc = "Field `WIFI_MAC_INT_MAP` reader - Need add description"]
pub type WIFI_MAC_INT_MAP_R = crate::FieldReader;
#[doc = "Field `WIFI_MAC_INT_MAP` writer - Need add description"]
pub type WIFI_MAC_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn wifi_mac_int_map(&self) -> WIFI_MAC_INT_MAP_R {
        WIFI_MAC_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_INTR_MAP")
            .field("wifi_mac_int_map", &self.wifi_mac_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn wifi_mac_int_map(&mut self) -> WIFI_MAC_INT_MAP_W<MAC_INTR_MAP_SPEC> {
        WIFI_MAC_INT_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_intr_map::R`](R) reader structure"]
impl crate::Readable for MAC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_intr_map::W`](W) writer structure"]
impl crate::Writable for MAC_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_INTR_MAP to value 0"]
impl crate::Resettable for MAC_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
