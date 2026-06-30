#[doc = "Register `ARG1` reader"]
pub type R = crate::R<ARG1_SPEC>;
#[doc = "Register `ARG1` writer"]
pub type W = crate::W<ARG1_SPEC>;
#[doc = "Field `DATA` reader - First argument data for calculate"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - First argument data for calculate"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - First argument data for calculate"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARG1").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - First argument data for calculate"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, ARG1_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Cordic argument 1 config register\n\nYou can [`read`](crate::Reg::read) this register and get [`arg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARG1_SPEC;
impl crate::RegisterSpec for ARG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg1::R`](R) reader structure"]
impl crate::Readable for ARG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arg1::W`](W) writer structure"]
impl crate::Writable for ARG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARG1 to value 0"]
impl crate::Resettable for ARG1_SPEC {}
