#[doc = "Register `ASSIST_DEBUG_INTR_MAP` reader"]
pub type R = crate::R<ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "Register `ASSIST_DEBUG_INTR_MAP` writer"]
pub type W = crate::W<ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "Field `ASSIST_DEBUG_INTR_MAP` reader - this register used to map assist_debug interrupt to one of core0's external interrupt"]
pub type ASSIST_DEBUG_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `ASSIST_DEBUG_INTR_MAP` writer - this register used to map assist_debug interrupt to one of core0's external interrupt"]
pub type ASSIST_DEBUG_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map assist_debug interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn assist_debug_intr_map(&self) -> ASSIST_DEBUG_INTR_MAP_R {
        ASSIST_DEBUG_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_DEBUG_INTR_MAP")
            .field("assist_debug_intr_map", &self.assist_debug_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map assist_debug interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn assist_debug_intr_map(&mut self) -> ASSIST_DEBUG_INTR_MAP_W<ASSIST_DEBUG_INTR_MAP_SPEC> {
        ASSIST_DEBUG_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "assist_debug interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`assist_debug_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assist_debug_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASSIST_DEBUG_INTR_MAP_SPEC;
impl crate::RegisterSpec for ASSIST_DEBUG_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`assist_debug_intr_map::R`](R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`assist_debug_intr_map::W`](W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ASSIST_DEBUG_INTR_MAP to value 0x10"]
impl crate::Resettable for ASSIST_DEBUG_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
