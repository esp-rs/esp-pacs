#[doc = "Register `UHCI1_INTR_MAP` reader"]
pub type R = crate::R<UHCI1_INTR_MAP_SPEC>;
#[doc = "Register `UHCI1_INTR_MAP` writer"]
pub type W = crate::W<UHCI1_INTR_MAP_SPEC>;
#[doc = "Field `UHCI1_INTR_MAP` reader - this register used to map uhci1 interrupt to one of core1's external interrupt"]
pub type UHCI1_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `UHCI1_INTR_MAP` writer - this register used to map uhci1 interrupt to one of core1's external interrupt"]
pub type UHCI1_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map uhci1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn uhci1_intr_map(&self) -> UHCI1_INTR_MAP_R {
        UHCI1_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI1_INTR_MAP")
            .field(
                "uhci1_intr_map",
                &format_args!("{}", self.uhci1_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UHCI1_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map uhci1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn uhci1_intr_map(&mut self) -> UHCI1_INTR_MAP_W<UHCI1_INTR_MAP_SPEC> {
        UHCI1_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "uhci1 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhci1_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci1_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHCI1_INTR_MAP_SPEC;
impl crate::RegisterSpec for UHCI1_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci1_intr_map::R`](R) reader structure"]
impl crate::Readable for UHCI1_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhci1_intr_map::W`](W) writer structure"]
impl crate::Writable for UHCI1_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UHCI1_INTR_MAP to value 0x10"]
impl crate::Resettable for UHCI1_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
