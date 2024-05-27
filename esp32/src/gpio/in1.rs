#[doc = "Register `IN1` reader"]
pub type R = crate::R<IN1_SPEC>;
#[doc = "Register `IN1` writer"]
pub type W = crate::W<IN1_SPEC>;
#[doc = "Field `DATA_NEXT` reader - GPIO32~39 input value"]
pub type DATA_NEXT_R = crate::FieldReader;
#[doc = "Field `DATA_NEXT` writer - GPIO32~39 input value"]
pub type DATA_NEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN1")
            .field("data_next", &self.data_next())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 input value"]
    #[inline(always)]
    #[must_use]
    pub fn data_next(&mut self) -> DATA_NEXT_W<IN1_SPEC> {
        DATA_NEXT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in1::R`](R) reader structure"]
impl crate::Readable for IN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in1::W`](W) writer structure"]
impl crate::Writable for IN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
