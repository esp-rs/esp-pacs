#[doc = "Register `MULT_START` writer"]
pub type W = crate::W<MULT_START_SPEC>;
#[doc = "Field `MULT_START` writer - Set this bit to 1 to start the multiplication."]
pub type MULT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to start the multiplication."]
    #[inline(always)]
    pub fn mult_start(&mut self) -> MULT_START_W<MULT_START_SPEC> {
        MULT_START_W::new(self, 0)
    }
}
#[doc = "Normal multiplication starting bit\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_START_SPEC;
impl crate::RegisterSpec for MULT_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mult_start::W`](W) writer structure"]
impl crate::Writable for MULT_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_START to value 0"]
impl crate::Resettable for MULT_START_SPEC {}
