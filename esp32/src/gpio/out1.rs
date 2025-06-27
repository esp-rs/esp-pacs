#[doc = "Register `OUT1` reader"]
pub type R = crate::R<OUT1_SPEC>;
#[doc = "Register `OUT1` writer"]
pub type W = crate::W<OUT1_SPEC>;
#[doc = "Field `DATA` reader - GPIO32~39 output value"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - GPIO32~39 output value"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output value"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT1").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output value"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<OUT1_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`out1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT1_SPEC;
impl crate::RegisterSpec for OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1::R`](R) reader structure"]
impl crate::Readable for OUT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out1::W`](W) writer structure"]
impl crate::Writable for OUT1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for OUT1_SPEC {}
