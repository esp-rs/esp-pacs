#[doc = "Register `SLC0_INTR_MAP` reader"]
pub type R = crate::R<SLC0_INTR_MAP_SPEC>;
#[doc = "Register `SLC0_INTR_MAP` writer"]
pub type W = crate::W<SLC0_INTR_MAP_SPEC>;
#[doc = "Field `SLC0_INTR_MAP` reader - reg_core0_slc0_intr_map"]
pub type SLC0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `SLC0_INTR_MAP` writer - reg_core0_slc0_intr_map"]
pub type SLC0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_slc0_intr_map"]
    #[inline(always)]
    pub fn slc0_intr_map(&self) -> SLC0_INTR_MAP_R {
        SLC0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_INTR_MAP")
            .field(
                "slc0_intr_map",
                &format_args!("{}", self.slc0_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_slc0_intr_map"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_intr_map(&mut self) -> SLC0_INTR_MAP_W<SLC0_INTR_MAP_SPEC> {
        SLC0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "slc0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0_INTR_MAP_SPEC;
impl crate::RegisterSpec for SLC0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0_intr_map::R`](R) reader structure"]
impl crate::Readable for SLC0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0_intr_map::W`](W) writer structure"]
impl crate::Writable for SLC0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0_INTR_MAP to value 0"]
impl crate::Resettable for SLC0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
