#[doc = "Register `OCCUPY_1` reader"]
pub type R = crate::R<OCCUPY_1_SPEC>;
#[doc = "Register `OCCUPY_1` writer"]
pub type W = crate::W<OCCUPY_1_SPEC>;
#[doc = "Field `OCCUPY_CACHE` reader - Configure whether SRAM Block 0-3 is used as cache memory."]
pub type OCCUPY_CACHE_R = crate::FieldReader;
#[doc = "Field `OCCUPY_CACHE` writer - Configure whether SRAM Block 0-3 is used as cache memory."]
pub type OCCUPY_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 0-3 is used as cache memory."]
    #[inline(always)]
    pub fn occupy_cache(&self) -> OCCUPY_CACHE_R {
        OCCUPY_CACHE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_1")
            .field("occupy_cache", &self.occupy_cache())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure whether SRAM Block 0-3 is used as cache memory."]
    #[inline(always)]
    pub fn occupy_cache(&mut self) -> OCCUPY_CACHE_W<'_, OCCUPY_1_SPEC> {
        OCCUPY_CACHE_W::new(self, 0)
    }
}
#[doc = "Occupy permission control register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`occupy_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occupy_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCCUPY_1_SPEC;
impl crate::RegisterSpec for OCCUPY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`occupy_1::R`](R) reader structure"]
impl crate::Readable for OCCUPY_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`occupy_1::W`](W) writer structure"]
impl crate::Writable for OCCUPY_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCCUPY_1 to value 0"]
impl crate::Resettable for OCCUPY_1_SPEC {}
