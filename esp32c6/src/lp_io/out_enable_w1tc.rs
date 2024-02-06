#[doc = "Register `OUT_ENABLE_W1TC` writer"]
pub type W = crate::W<OUT_ENABLE_W1TC_SPEC>;
#[doc = "Field `LP_GPIO_ENABLE_W1TC` writer - clear one time output data"]
pub type LP_GPIO_ENABLE_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - clear one time output data"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio_enable_w1tc(&mut self) -> LP_GPIO_ENABLE_W1TC_W<OUT_ENABLE_W1TC_SPEC> {
        LP_GPIO_ENABLE_W1TC_W::new(self, 0)
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
#[doc = "need des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for OUT_ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_enable_w1tc::W`](W) writer structure"]
impl crate::Writable for OUT_ENABLE_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_ENABLE_W1TC to value 0"]
impl crate::Resettable for OUT_ENABLE_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
