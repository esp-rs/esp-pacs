#[doc = "Register `OUT_W1TC` writer"]
pub type W = crate::W<OUT_W1TC_SPEC>;
#[doc = "Field `OUT_W1TC` writer - GPIO output clear register for GPIO0-25"]
pub type OUT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:25 - GPIO output clear register for GPIO0-25"]
    #[inline(always)]
    #[must_use]
    pub fn out_w1tc(&mut self) -> OUT_W1TC_W<OUT_W1TC_SPEC> {
        OUT_W1TC_W::new(self, 0)
    }
}
#[doc = "GPIO output clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_W1TC_SPEC;
impl crate::RegisterSpec for OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1tc::W`](W) writer structure"]
impl crate::Writable for OUT_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_W1TC to value 0"]
impl crate::Resettable for OUT_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
