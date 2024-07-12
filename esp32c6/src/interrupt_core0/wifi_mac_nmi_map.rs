#[doc = "Register `WIFI_MAC_NMI_MAP` reader"]
pub type R = crate::R<WIFI_MAC_NMI_MAP_SPEC>;
#[doc = "Register `WIFI_MAC_NMI_MAP` writer"]
pub type W = crate::W<WIFI_MAC_NMI_MAP_SPEC>;
#[doc = "Field `WIFI_MAC_NMI_MAP` reader - Need add description"]
pub type WIFI_MAC_NMI_MAP_R = crate::FieldReader;
#[doc = "Field `WIFI_MAC_NMI_MAP` writer - Need add description"]
pub type WIFI_MAC_NMI_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn wifi_mac_nmi_map(&self) -> WIFI_MAC_NMI_MAP_R {
        WIFI_MAC_NMI_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_MAC_NMI_MAP")
            .field("wifi_mac_nmi_map", &self.wifi_mac_nmi_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mac_nmi_map(&mut self) -> WIFI_MAC_NMI_MAP_W<WIFI_MAC_NMI_MAP_SPEC> {
        WIFI_MAC_NMI_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_mac_nmi_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_mac_nmi_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_MAC_NMI_MAP_SPEC;
impl crate::RegisterSpec for WIFI_MAC_NMI_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_mac_nmi_map::R`](R) reader structure"]
impl crate::Readable for WIFI_MAC_NMI_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_mac_nmi_map::W`](W) writer structure"]
impl crate::Writable for WIFI_MAC_NMI_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIFI_MAC_NMI_MAP to value 0"]
impl crate::Resettable for WIFI_MAC_NMI_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
