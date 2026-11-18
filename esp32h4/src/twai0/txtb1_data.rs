#[doc = "Register `TXTB1_DATA%s` writer"]
pub type W = crate::W<TXTB1_DATA_SPEC>;
#[doc = "Field `DATA` writer - Frame buffer word"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXTB1_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Frame buffer word"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, TXTB1_DATA_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "TX buffer 1 frame memory word\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txtb1_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXTB1_DATA_SPEC;
impl crate::RegisterSpec for TXTB1_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txtb1_data::W`](W) writer structure"]
impl crate::Writable for TXTB1_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXTB1_DATA%s to value 0"]
impl crate::Resettable for TXTB1_DATA_SPEC {}
