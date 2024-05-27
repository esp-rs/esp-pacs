#[doc = "Register `COEX_INT_MAP` reader"]
pub type R = crate::R<COEX_INT_MAP_SPEC>;
#[doc = "Register `COEX_INT_MAP` writer"]
pub type W = crate::W<COEX_INT_MAP_SPEC>;
#[doc = "Field `COEX_INT_MAP` reader - Need add description"]
pub type COEX_INT_MAP_R = crate::FieldReader;
#[doc = "Field `COEX_INT_MAP` writer - Need add description"]
pub type COEX_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn coex_int_map(&self) -> COEX_INT_MAP_R {
        COEX_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEX_INT_MAP")
            .field("coex_int_map", &self.coex_int_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn coex_int_map(&mut self) -> COEX_INT_MAP_W<COEX_INT_MAP_SPEC> {
        COEX_INT_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COEX_INT_MAP_SPEC;
impl crate::RegisterSpec for COEX_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_int_map::R`](R) reader structure"]
impl crate::Readable for COEX_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`coex_int_map::W`](W) writer structure"]
impl crate::Writable for COEX_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COEX_INT_MAP to value 0"]
impl crate::Resettable for COEX_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
