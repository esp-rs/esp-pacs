#[doc = "Register `UHCI0_INTR_MAP` reader"]
pub type R = crate::R<UHCI0_INTR_MAP_SPEC>;
#[doc = "Register `UHCI0_INTR_MAP` writer"]
pub type W = crate::W<UHCI0_INTR_MAP_SPEC>;
#[doc = "Field `UHCI0_INTR_MAP` reader - reg_core0_uhci0_intr_map"]
pub type UHCI0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `UHCI0_INTR_MAP` writer - reg_core0_uhci0_intr_map"]
pub type UHCI0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_uhci0_intr_map"]
    #[inline(always)]
    pub fn uhci0_intr_map(&self) -> UHCI0_INTR_MAP_R {
        UHCI0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI0_INTR_MAP")
            .field(
                "uhci0_intr_map",
                &format_args!("{}", self.uhci0_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UHCI0_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_uhci0_intr_map"]
    #[inline(always)]
    #[must_use]
    pub fn uhci0_intr_map(&mut self) -> UHCI0_INTR_MAP_W<UHCI0_INTR_MAP_SPEC> {
        UHCI0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "uchi0 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHCI0_INTR_MAP_SPEC;
impl crate::RegisterSpec for UHCI0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci0_intr_map::R`](R) reader structure"]
impl crate::Readable for UHCI0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhci0_intr_map::W`](W) writer structure"]
impl crate::Writable for UHCI0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UHCI0_INTR_MAP to value 0"]
impl crate::Resettable for UHCI0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
