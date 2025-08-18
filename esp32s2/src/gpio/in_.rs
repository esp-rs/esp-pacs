#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Register `IN` writer"]
pub type W = crate::W<IN_SPEC>;
#[doc = "Field `DATA_NEXT` reader - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
pub type DATA_NEXT_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_NEXT` writer - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
pub type DATA_NEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
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
    #[doc = "Bits 0:31 - GPIO0 ~ 31 input value. Each bit represents a pad input value, 1 for high level and 0 for low level."]
    #[inline(always)]
    pub fn data_next(&mut self) -> DATA_NEXT_W<'_, IN_SPEC> {
        DATA_NEXT_W::new(self, 0)
    }
}
#[doc = "GPIO0 ~ 31 input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_::W`](W) writer structure"]
impl crate::Writable for IN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {}
