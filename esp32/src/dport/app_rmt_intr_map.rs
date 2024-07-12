#[doc = "Register `APP_RMT_INTR_MAP` reader"]
pub type R = crate::R<APP_RMT_INTR_MAP_SPEC>;
#[doc = "Register `APP_RMT_INTR_MAP` writer"]
pub type W = crate::W<APP_RMT_INTR_MAP_SPEC>;
#[doc = "Field `APP_RMT_INTR_MAP` reader - "]
pub type APP_RMT_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `APP_RMT_INTR_MAP` writer - "]
pub type APP_RMT_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_rmt_intr_map(&self) -> APP_RMT_INTR_MAP_R {
        APP_RMT_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_RMT_INTR_MAP")
            .field("app_rmt_intr_map", &self.app_rmt_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn app_rmt_intr_map(&mut self) -> APP_RMT_INTR_MAP_W<APP_RMT_INTR_MAP_SPEC> {
        APP_RMT_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`app_rmt_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_rmt_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_RMT_INTR_MAP_SPEC;
impl crate::RegisterSpec for APP_RMT_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_rmt_intr_map::R`](R) reader structure"]
impl crate::Readable for APP_RMT_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_rmt_intr_map::W`](W) writer structure"]
impl crate::Writable for APP_RMT_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_RMT_INTR_MAP to value 0x10"]
impl crate::Resettable for APP_RMT_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
