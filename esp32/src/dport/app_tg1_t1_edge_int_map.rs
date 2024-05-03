#[doc = "Register `APP_TG1_T1_EDGE_INT_MAP` reader"]
pub type R = crate::R<APP_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = "Register `APP_TG1_T1_EDGE_INT_MAP` writer"]
pub type W = crate::W<APP_TG1_T1_EDGE_INT_MAP_SPEC>;
#[doc = "Field `APP_TG1_T1_EDGE_INT_MAP` reader - "]
pub type APP_TG1_T1_EDGE_INT_MAP_R = crate::FieldReader;
#[doc = "Field `APP_TG1_T1_EDGE_INT_MAP` writer - "]
pub type APP_TG1_T1_EDGE_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn app_tg1_t1_edge_int_map(&self) -> APP_TG1_T1_EDGE_INT_MAP_R {
        APP_TG1_T1_EDGE_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_TG1_T1_EDGE_INT_MAP")
            .field(
                "app_tg1_t1_edge_int_map",
                &self.app_tg1_t1_edge_int_map().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_TG1_T1_EDGE_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn app_tg1_t1_edge_int_map(
        &mut self,
    ) -> APP_TG1_T1_EDGE_INT_MAP_W<APP_TG1_T1_EDGE_INT_MAP_SPEC> {
        APP_TG1_T1_EDGE_INT_MAP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_tg1_t1_edge_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_tg1_t1_edge_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_TG1_T1_EDGE_INT_MAP_SPEC;
impl crate::RegisterSpec for APP_TG1_T1_EDGE_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_tg1_t1_edge_int_map::R`](R) reader structure"]
impl crate::Readable for APP_TG1_T1_EDGE_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`app_tg1_t1_edge_int_map::W`](W) writer structure"]
impl crate::Writable for APP_TG1_T1_EDGE_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_TG1_T1_EDGE_INT_MAP to value 0x10"]
impl crate::Resettable for APP_TG1_T1_EDGE_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
