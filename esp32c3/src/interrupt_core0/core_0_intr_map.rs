#[doc = "Register `CORE_0_INTR_MAP%s` reader"]
pub type R = crate::R<CORE_0_INTR_MAP_SPEC>;
#[doc = "Register `CORE_0_INTR_MAP%s` writer"]
pub type W = crate::W<CORE_0_INTR_MAP_SPEC>;
#[doc = "Field `MAP` reader - "]
pub type MAP_R = crate::FieldReader;
#[doc = "Field `MAP` writer - "]
pub type MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_MAP")
            .field("map", &self.map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W<'_, CORE_0_INTR_MAP_SPEC> {
        MAP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_MAP_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_map::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_intr_map::W`](W) writer structure"]
impl crate::Writable for CORE_0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_INTR_MAP%s to value 0"]
impl crate::Resettable for CORE_0_INTR_MAP_SPEC {}
