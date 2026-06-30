#[doc = "Register `ARG2` reader"]
pub type R = crate::R<ARG2_SPEC>;
#[doc = "Register `ARG2` writer"]
pub type W = crate::W<ARG2_SPEC>;
#[doc = "Field `DATA` reader - Second argument data for calculate"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Second argument data for calculate"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Second argument data for calculate"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARG2").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Second argument data for calculate"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, ARG2_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Cordic argument 2 config register\n\nYou can [`read`](crate::Reg::read) this register and get [`arg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARG2_SPEC;
impl crate::RegisterSpec for ARG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg2::R`](R) reader structure"]
impl crate::Readable for ARG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arg2::W`](W) writer structure"]
impl crate::Writable for ARG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARG2 to value 0"]
impl crate::Resettable for ARG2_SPEC {}
