#[doc = "Register `STEP` reader"]
pub type R = crate::R<STEP_SPEC>;
#[doc = "Register `STEP` writer"]
pub type W = crate::W<STEP_SPEC>;
#[doc = "Field `TIMER_XTAL_STEP` reader - Set system timer increment step when using XTAL_CLK."]
pub type TIMER_XTAL_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_XTAL_STEP` writer - Set system timer increment step when using XTAL_CLK."]
pub type TIMER_XTAL_STEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `TIMER_PLL_STEP` reader - Set system timer increment step when using PLL_CLK"]
pub type TIMER_PLL_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_PLL_STEP` writer - Set system timer increment step when using PLL_CLK"]
pub type TIMER_PLL_STEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    pub fn timer_xtal_step(&self) -> TIMER_XTAL_STEP_R {
        TIMER_XTAL_STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    pub fn timer_pll_step(&self) -> TIMER_PLL_STEP_R {
        TIMER_PLL_STEP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STEP")
            .field(
                "timer_xtal_step",
                &format_args!("{}", self.timer_xtal_step().bits()),
            )
            .field(
                "timer_pll_step",
                &format_args!("{}", self.timer_pll_step().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STEP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn timer_xtal_step(&mut self) -> TIMER_XTAL_STEP_W<STEP_SPEC, 0> {
        TIMER_XTAL_STEP_W::new(self)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn timer_pll_step(&mut self) -> TIMER_PLL_STEP_W<STEP_SPEC, 10> {
        TIMER_PLL_STEP_W::new(self)
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
#[doc = "System timer accumulation step\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`step::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`step::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STEP_SPEC;
impl crate::RegisterSpec for STEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`step::R`](R) reader structure"]
impl crate::Readable for STEP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`step::W`](W) writer structure"]
impl crate::Writable for STEP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STEP to value 0x0450"]
impl crate::Resettable for STEP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0450;
}
