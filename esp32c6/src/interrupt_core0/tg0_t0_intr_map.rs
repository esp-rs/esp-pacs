#[doc = "Register `TG0_T0_INTR_MAP` reader"]
pub type R = crate::R<TG0_T0_INTR_MAP_SPEC>;
#[doc = "Register `TG0_T0_INTR_MAP` writer"]
pub type W = crate::W<TG0_T0_INTR_MAP_SPEC>;
#[doc = "Field `TG0_T0_INTR_MAP` reader - Need add description"]
pub type TG0_T0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `TG0_T0_INTR_MAP` writer - Need add description"]
pub type TG0_T0_INTR_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn tg0_t0_intr_map(&self) -> TG0_T0_INTR_MAP_R {
        TG0_T0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TG0_T0_INTR_MAP")
            .field(
                "tg0_t0_intr_map",
                &format_args!("{}", self.tg0_t0_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TG0_T0_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn tg0_t0_intr_map(&mut self) -> TG0_T0_INTR_MAP_W<TG0_T0_INTR_MAP_SPEC, 0> {
        TG0_T0_INTR_MAP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tg0_t0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tg0_t0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TG0_T0_INTR_MAP_SPEC;
impl crate::RegisterSpec for TG0_T0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tg0_t0_intr_map::R`](R) reader structure"]
impl crate::Readable for TG0_T0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tg0_t0_intr_map::W`](W) writer structure"]
impl crate::Writable for TG0_T0_INTR_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TG0_T0_INTR_MAP to value 0"]
impl crate::Resettable for TG0_T0_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
