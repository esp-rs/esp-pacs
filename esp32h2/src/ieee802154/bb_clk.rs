#[doc = "Register `BB_CLK` reader"]
pub type R = crate::R<BB_CLK_SPEC>;
#[doc = "Register `BB_CLK` writer"]
pub type W = crate::W<BB_CLK_SPEC>;
#[doc = "Field `FREQ_MINUS_1` reader - "]
pub type FREQ_MINUS_1_R = crate::FieldReader;
#[doc = "Field `FREQ_MINUS_1` writer - "]
pub type FREQ_MINUS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn freq_minus_1(&self) -> FREQ_MINUS_1_R {
        FREQ_MINUS_1_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BB_CLK")
            .field(
                "freq_minus_1",
                &format_args!("{}", self.freq_minus_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BB_CLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn freq_minus_1(&mut self) -> FREQ_MINUS_1_W<BB_CLK_SPEC> {
        FREQ_MINUS_1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bb_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bb_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BB_CLK_SPEC;
impl crate::RegisterSpec for BB_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bb_clk::R`](R) reader structure"]
impl crate::Readable for BB_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bb_clk::W`](W) writer structure"]
impl crate::Writable for BB_CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BB_CLK to value 0"]
impl crate::Resettable for BB_CLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
