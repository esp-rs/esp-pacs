#[doc = "Register `TWAI1_INTR_MAP` reader"]
pub type R = crate::R<TWAI1_INTR_MAP_SPEC>;
#[doc = "Register `TWAI1_INTR_MAP` writer"]
pub type W = crate::W<TWAI1_INTR_MAP_SPEC>;
#[doc = "Field `TWAI1_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type TWAI1_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `TWAI1_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type TWAI1_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn twai1_intr_map(&self) -> TWAI1_INTR_MAP_R {
        TWAI1_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI1_INTR_MAP")
            .field("twai1_intr_map", &self.twai1_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn twai1_intr_map(&mut self) -> TWAI1_INTR_MAP_W<TWAI1_INTR_MAP_SPEC> {
        TWAI1_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "TWAI1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`twai1_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai1_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAI1_INTR_MAP_SPEC;
impl crate::RegisterSpec for TWAI1_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twai1_intr_map::R`](R) reader structure"]
impl crate::Readable for TWAI1_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twai1_intr_map::W`](W) writer structure"]
impl crate::Writable for TWAI1_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWAI1_INTR_MAP to value 0"]
impl crate::Resettable for TWAI1_INTR_MAP_SPEC {}
