#[doc = "Register `TIMER5` reader"]
pub type R = crate::R<TIMER5_SPEC>;
#[doc = "Register `TIMER5` writer"]
pub type W = crate::W<TIMER5_SPEC>;
#[doc = "Field `MIN_SLP_VAL` reader - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_R = crate::FieldReader;
#[doc = "Field `MIN_SLP_VAL` writer - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER5")
            .field(
                "min_slp_val",
                &format_args!("{}", self.min_slp_val().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W<TIMER5_SPEC> {
        MIN_SLP_VAL_W::new(self, 8)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER5_SPEC;
impl crate::RegisterSpec for TIMER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer5::R`](R) reader structure"]
impl crate::Readable for TIMER5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer5::W`](W) writer structure"]
impl crate::Writable for TIMER5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER5 to value 0x8000"]
impl crate::Resettable for TIMER5_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
