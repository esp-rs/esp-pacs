#[doc = "Register `STEP` reader"]
pub type R = crate::R<STEP_SPEC>;
#[doc = "Register `STEP` writer"]
pub type W = crate::W<STEP_SPEC>;
#[doc = "Field `XTAL_STEP` reader - Set system timer increment step when using XTAL_CLK."]
pub type XTAL_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `XTAL_STEP` writer - Set system timer increment step when using XTAL_CLK."]
pub type XTAL_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PLL_STEP` reader - Set system timer increment step when using PLL_CLK"]
pub type PLL_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `PLL_STEP` writer - Set system timer increment step when using PLL_CLK"]
pub type PLL_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    pub fn xtal_step(&self) -> XTAL_STEP_R {
        XTAL_STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    pub fn pll_step(&self) -> PLL_STEP_R {
        PLL_STEP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STEP")
            .field("xtal_step", &format_args!("{}", self.xtal_step().bits()))
            .field("pll_step", &format_args!("{}", self.pll_step().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STEP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set system timer increment step when using XTAL_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn xtal_step(&mut self) -> XTAL_STEP_W<STEP_SPEC> {
        XTAL_STEP_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Set system timer increment step when using PLL_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn pll_step(&mut self) -> PLL_STEP_W<STEP_SPEC> {
        PLL_STEP_W::new(self, 10)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STEP to value 0x0450"]
impl crate::Resettable for STEP_SPEC {
    const RESET_VALUE: u32 = 0x0450;
}
