#[doc = "Register `WIFI_WAKEUP_SRC` reader"]
pub type R = crate::R<WIFI_WAKEUP_SRC_SPEC>;
#[doc = "Register `WIFI_WAKEUP_SRC` writer"]
pub type W = crate::W<WIFI_WAKEUP_SRC_SPEC>;
#[doc = "Field `WIFI_WAKEUP_SOURCE_EN` reader - wakeup enable signal for wifi"]
pub type WIFI_WAKEUP_SOURCE_EN_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_WAKEUP_SOURCE_EN` writer - wakeup enable signal for wifi"]
pub type WIFI_WAKEUP_SOURCE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - wakeup enable signal for wifi"]
    #[inline(always)]
    pub fn wifi_wakeup_source_en(&self) -> WIFI_WAKEUP_SOURCE_EN_R {
        WIFI_WAKEUP_SOURCE_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_WAKEUP_SRC")
            .field("wifi_wakeup_source_en", &self.wifi_wakeup_source_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - wakeup enable signal for wifi"]
    #[inline(always)]
    pub fn wifi_wakeup_source_en(&mut self) -> WIFI_WAKEUP_SOURCE_EN_W<'_, WIFI_WAKEUP_SRC_SPEC> {
        WIFI_WAKEUP_SOURCE_EN_W::new(self, 0)
    }
}
#[doc = "wakeup source register for wifi\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_wakeup_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_wakeup_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_WAKEUP_SRC_SPEC;
impl crate::RegisterSpec for WIFI_WAKEUP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_wakeup_src::R`](R) reader structure"]
impl crate::Readable for WIFI_WAKEUP_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_wakeup_src::W`](W) writer structure"]
impl crate::Writable for WIFI_WAKEUP_SRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_WAKEUP_SRC to value 0"]
impl crate::Resettable for WIFI_WAKEUP_SRC_SPEC {}
