#[doc = "Register `TIMER_INT2_MAP` reader"]
pub type R = crate::R<TIMER_INT2_MAP_SPEC>;
#[doc = "Register `TIMER_INT2_MAP` writer"]
pub type W = crate::W<TIMER_INT2_MAP_SPEC>;
#[doc = "Field `TIMER_INT2_MAP` reader - this register used to map timer_int2 interrupt to one of core1's external interrupt"]
pub type TIMER_INT2_MAP_R = crate::FieldReader;
#[doc = "Field `TIMER_INT2_MAP` writer - this register used to map timer_int2 interrupt to one of core1's external interrupt"]
pub type TIMER_INT2_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map timer_int2 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn timer_int2_map(&self) -> TIMER_INT2_MAP_R {
        TIMER_INT2_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT2_MAP")
            .field(
                "timer_int2_map",
                &format_args!("{}", self.timer_int2_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_INT2_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map timer_int2 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_int2_map(&mut self) -> TIMER_INT2_MAP_W<TIMER_INT2_MAP_SPEC> {
        TIMER_INT2_MAP_W::new(self, 0)
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
#[doc = "timer_int2 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int2_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int2_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT2_MAP_SPEC;
impl crate::RegisterSpec for TIMER_INT2_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_int2_map::R`](R) reader structure"]
impl crate::Readable for TIMER_INT2_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_int2_map::W`](W) writer structure"]
impl crate::Writable for TIMER_INT2_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_INT2_MAP to value 0x10"]
impl crate::Resettable for TIMER_INT2_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
