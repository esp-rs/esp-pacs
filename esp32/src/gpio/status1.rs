#[doc = "Register `STATUS1` reader"]
pub type R = crate::R<STATUS1_SPEC>;
#[doc = "Register `STATUS1` writer"]
pub type W = crate::W<STATUS1_SPEC>;
#[doc = "Field `INT` reader - GPIO32~39 interrupt status"]
pub type INT_R = crate::FieldReader;
#[doc = "Field `INT` writer - GPIO32~39 interrupt status"]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS1").field("int", &self.int()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<STATUS1_SPEC> {
        INT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1::R`](R) reader structure"]
impl crate::Readable for STATUS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status1::W`](W) writer structure"]
impl crate::Writable for STATUS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
