#[doc = "Register `SET_FINISH` writer"]
pub type W = crate::W<SET_FINISH_SPEC>;
#[doc = "Field `SET_FINISH` writer - Set this bit to finish DS process."]
pub type SET_FINISH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_FINISH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to finish DS process."]
    #[inline(always)]
    #[must_use]
    pub fn set_finish(&mut self) -> SET_FINISH_W<SET_FINISH_SPEC> {
        SET_FINISH_W::new(self, 0)
    }
}
#[doc = "DS finish control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_finish::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_FINISH_SPEC;
impl crate::RegisterSpec for SET_FINISH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_finish::W`](W) writer structure"]
impl crate::Writable for SET_FINISH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET_FINISH to value 0"]
impl crate::Resettable for SET_FINISH_SPEC {
    const RESET_VALUE: u32 = 0;
}
