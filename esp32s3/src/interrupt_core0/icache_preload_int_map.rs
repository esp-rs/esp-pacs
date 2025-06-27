#[doc = "Register `ICACHE_PRELOAD_INT_MAP` reader"]
pub type R = crate::R<ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "Register `ICACHE_PRELOAD_INT_MAP` writer"]
pub type W = crate::W<ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "Field `ICACHE_PRELOAD_INT_MAP` reader - this register used to map icache_preload interrupt to one of core0's external interrupt"]
pub type ICACHE_PRELOAD_INT_MAP_R = crate::FieldReader;
#[doc = "Field `ICACHE_PRELOAD_INT_MAP` writer - this register used to map icache_preload interrupt to one of core0's external interrupt"]
pub type ICACHE_PRELOAD_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map icache_preload interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn icache_preload_int_map(&self) -> ICACHE_PRELOAD_INT_MAP_R {
        ICACHE_PRELOAD_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_PRELOAD_INT_MAP")
            .field("icache_preload_int_map", &self.icache_preload_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map icache_preload interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn icache_preload_int_map(
        &mut self,
    ) -> ICACHE_PRELOAD_INT_MAP_W<ICACHE_PRELOAD_INT_MAP_SPEC> {
        ICACHE_PRELOAD_INT_MAP_W::new(self, 0)
    }
}
#[doc = "icache_preload interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_preload_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_preload_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_PRELOAD_INT_MAP_SPEC;
impl crate::RegisterSpec for ICACHE_PRELOAD_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_preload_int_map::R`](R) reader structure"]
impl crate::Readable for ICACHE_PRELOAD_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_preload_int_map::W`](W) writer structure"]
impl crate::Writable for ICACHE_PRELOAD_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE_PRELOAD_INT_MAP to value 0x10"]
impl crate::Resettable for ICACHE_PRELOAD_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
