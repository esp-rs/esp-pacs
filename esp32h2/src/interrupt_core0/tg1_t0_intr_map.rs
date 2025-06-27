#[doc = "Register `TG1_T0_INTR_MAP` reader"]
pub type R = crate::R<TG1_T0_INTR_MAP_SPEC>;
#[doc = "Register `TG1_T0_INTR_MAP` writer"]
pub type W = crate::W<TG1_T0_INTR_MAP_SPEC>;
#[doc = "Field `TG1_T0_INTR_MAP` reader - CORE0_TG1_T0_INTR mapping register"]
pub type TG1_T0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `TG1_T0_INTR_MAP` writer - CORE0_TG1_T0_INTR mapping register"]
pub type TG1_T0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_TG1_T0_INTR mapping register"]
    #[inline(always)]
    pub fn tg1_t0_intr_map(&self) -> TG1_T0_INTR_MAP_R {
        TG1_T0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TG1_T0_INTR_MAP")
            .field("tg1_t0_intr_map", &self.tg1_t0_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_TG1_T0_INTR mapping register"]
    #[inline(always)]
    pub fn tg1_t0_intr_map(&mut self) -> TG1_T0_INTR_MAP_W<TG1_T0_INTR_MAP_SPEC> {
        TG1_T0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`tg1_t0_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tg1_t0_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TG1_T0_INTR_MAP_SPEC;
impl crate::RegisterSpec for TG1_T0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tg1_t0_intr_map::R`](R) reader structure"]
impl crate::Readable for TG1_T0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tg1_t0_intr_map::W`](W) writer structure"]
impl crate::Writable for TG1_T0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TG1_T0_INTR_MAP to value 0"]
impl crate::Resettable for TG1_T0_INTR_MAP_SPEC {}
