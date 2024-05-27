#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Register `IN` writer"]
pub type W = crate::W<IN_SPEC>;
#[doc = "Field `DATA_NEXT` reader - GPIO0~31 input value"]
pub type DATA_NEXT_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_NEXT` writer - GPIO0~31 input value"]
pub type DATA_NEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 input value"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN")
            .field("data_next", &self.data_next())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 input value"]
    #[inline(always)]
    #[must_use]
    pub fn data_next(&mut self) -> DATA_NEXT_W<IN_SPEC> {
        DATA_NEXT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_::W`](W) writer structure"]
impl crate::Writable for IN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
