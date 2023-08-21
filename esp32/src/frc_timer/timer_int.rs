#[doc = "Register `TIMER_INT` reader"]
pub type R = crate::R<TIMER_INT_SPEC>;
#[doc = "Register `TIMER_INT` writer"]
pub type W = crate::W<TIMER_INT_SPEC>;
#[doc = "Field `CLR` reader - "]
pub type CLR_R = crate::BitReader;
#[doc = "Field `CLR` writer - "]
pub type CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT")
            .field("clr", &format_args!("{}", self.clr().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<TIMER_INT_SPEC, 0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT_SPEC;
impl crate::RegisterSpec for TIMER_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_int::R`](R) reader structure"]
impl crate::Readable for TIMER_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_int::W`](W) writer structure"]
impl crate::Writable for TIMER_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_INT to value 0"]
impl crate::Resettable for TIMER_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
