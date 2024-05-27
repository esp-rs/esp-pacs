#[doc = "Register `STATUS1_W1TS` reader"]
pub type R = crate::R<STATUS1_W1TS_SPEC>;
#[doc = "Register `STATUS1_W1TS` writer"]
pub type W = crate::W<STATUS1_W1TS_SPEC>;
#[doc = "Field `STATUS1_INT_W1TS` reader - GPIO32~39 interrupt status write 1 to set"]
pub type STATUS1_INT_W1TS_R = crate::FieldReader;
#[doc = "Field `STATUS1_INT_W1TS` writer - GPIO32~39 interrupt status write 1 to set"]
pub type STATUS1_INT_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status1_int_w1ts(&self) -> STATUS1_INT_W1TS_R {
        STATUS1_INT_W1TS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS1_W1TS")
            .field("status1_int_w1ts", &self.status1_int_w1ts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn status1_int_w1ts(&mut self) -> STATUS1_INT_W1TS_W<STATUS1_W1TS_SPEC> {
        STATUS1_INT_W1TS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1_w1ts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_W1TS_SPEC;
impl crate::RegisterSpec for STATUS1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1_w1ts::R`](R) reader structure"]
impl crate::Readable for STATUS1_W1TS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status1_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS1_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS1_W1TS to value 0"]
impl crate::Resettable for STATUS1_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
