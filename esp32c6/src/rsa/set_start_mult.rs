#[doc = "Register `SET_START_MULT` writer"]
pub type W = crate::W<SET_START_MULT_SPEC>;
#[doc = "Field `SET_START_MULT` writer - start multiplicaiton"]
pub type SET_START_MULT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_START_MULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - start multiplicaiton"]
    #[inline(always)]
    pub fn set_start_mult(&mut self) -> SET_START_MULT_W<SET_START_MULT_SPEC> {
        SET_START_MULT_W::new(self, 0)
    }
}
#[doc = "RSA normal multiplication trigger register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_start_mult::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_START_MULT_SPEC;
impl crate::RegisterSpec for SET_START_MULT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_mult::W`](W) writer structure"]
impl crate::Writable for SET_START_MULT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_START_MULT to value 0"]
impl crate::Resettable for SET_START_MULT_SPEC {}
