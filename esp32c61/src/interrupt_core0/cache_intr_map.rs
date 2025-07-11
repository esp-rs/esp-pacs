#[doc = "Register `CACHE_INTR_MAP` reader"]
pub type R = crate::R<CACHE_INTR_MAP_SPEC>;
#[doc = "Register `CACHE_INTR_MAP` writer"]
pub type W = crate::W<CACHE_INTR_MAP_SPEC>;
#[doc = "Field `CACHE_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type CACHE_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `CACHE_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type CACHE_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CACHE_INTR_PASS_IN_SEC` reader - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type CACHE_INTR_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `CACHE_INTR_PASS_IN_SEC` writer - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type CACHE_INTR_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn cache_intr_map(&self) -> CACHE_INTR_MAP_R {
        CACHE_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn cache_intr_pass_in_sec(&self) -> CACHE_INTR_PASS_IN_SEC_R {
        CACHE_INTR_PASS_IN_SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_INTR_MAP")
            .field("cache_intr_map", &self.cache_intr_map())
            .field("cache_intr_pass_in_sec", &self.cache_intr_pass_in_sec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn cache_intr_map(&mut self) -> CACHE_INTR_MAP_W<CACHE_INTR_MAP_SPEC> {
        CACHE_INTR_MAP_W::new(self, 0)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn cache_intr_pass_in_sec(&mut self) -> CACHE_INTR_PASS_IN_SEC_W<CACHE_INTR_MAP_SPEC> {
        CACHE_INTR_PASS_IN_SEC_W::new(self, 8)
    }
}
#[doc = "CACHE_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_INTR_MAP_SPEC;
impl crate::RegisterSpec for CACHE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_intr_map::R`](R) reader structure"]
impl crate::Readable for CACHE_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_intr_map::W`](W) writer structure"]
impl crate::Writable for CACHE_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_INTR_MAP to value 0"]
impl crate::Resettable for CACHE_INTR_MAP_SPEC {}
