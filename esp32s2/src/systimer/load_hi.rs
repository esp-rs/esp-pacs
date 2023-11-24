#[doc = "Register `LOAD_HI` reader"]
pub type R = crate::R<LOAD_HI_SPEC>;
#[doc = "Register `LOAD_HI` writer"]
pub type W = crate::W<LOAD_HI_SPEC>;
#[doc = "Field `TIMER_LOAD_HI` reader - The value to be loaded into system timer, high 32 bits."]
pub type TIMER_LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_LOAD_HI` writer - The value to be loaded into system timer, high 32 bits."]
pub type TIMER_LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, high 32 bits."]
    #[inline(always)]
    pub fn timer_load_hi(&self) -> TIMER_LOAD_HI_R {
        TIMER_LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOAD_HI")
            .field(
                "timer_load_hi",
                &format_args!("{}", self.timer_load_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOAD_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value to be loaded into system timer, high 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn timer_load_hi(&mut self) -> TIMER_LOAD_HI_W<LOAD_HI_SPEC> {
        TIMER_LOAD_HI_W::new(self, 0)
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
#[doc = "High 32 bits to be loaded to system timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD_HI_SPEC;
impl crate::RegisterSpec for LOAD_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_hi::R`](R) reader structure"]
impl crate::Readable for LOAD_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`load_hi::W`](W) writer structure"]
impl crate::Writable for LOAD_HI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOAD_HI to value 0"]
impl crate::Resettable for LOAD_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
