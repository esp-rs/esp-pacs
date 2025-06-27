#[doc = "Register `STATUS1_W1TC` reader"]
pub type R = crate::R<STATUS1_W1TC_SPEC>;
#[doc = "Register `STATUS1_W1TC` writer"]
pub type W = crate::W<STATUS1_W1TC_SPEC>;
#[doc = "Field `STATUS1_INT_W1TC` reader - GPIO32~39 interrupt status write 1 to clear"]
pub type STATUS1_INT_W1TC_R = crate::FieldReader;
#[doc = "Field `STATUS1_INT_W1TC` writer - GPIO32~39 interrupt status write 1 to clear"]
pub type STATUS1_INT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to clear"]
    #[inline(always)]
    pub fn status1_int_w1tc(&self) -> STATUS1_INT_W1TC_R {
        STATUS1_INT_W1TC_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS1_W1TC")
            .field("status1_int_w1tc", &self.status1_int_w1tc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to clear"]
    #[inline(always)]
    pub fn status1_int_w1tc(&mut self) -> STATUS1_INT_W1TC_W<STATUS1_W1TC_SPEC> {
        STATUS1_INT_W1TC_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status1_w1tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_W1TC_SPEC;
impl crate::RegisterSpec for STATUS1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1_w1tc::R`](R) reader structure"]
impl crate::Readable for STATUS1_W1TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status1_w1tc::W`](W) writer structure"]
impl crate::Writable for STATUS1_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS1_W1TC to value 0"]
impl crate::Resettable for STATUS1_W1TC_SPEC {}
