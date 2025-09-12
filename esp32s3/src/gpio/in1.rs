#[doc = "Register `IN1` reader"]
pub type R = crate::R<IN1_SPEC>;
#[doc = "Register `IN1` writer"]
pub type W = crate::W<IN1_SPEC>;
#[doc = "Field `DATA_NEXT` reader - GPIO input register for GPIO32-53"]
pub type DATA_NEXT_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_NEXT` writer - GPIO input register for GPIO32-53"]
pub type DATA_NEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - GPIO input register for GPIO32-53"]
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new(self.bits & 0x003f_ffff)
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
    #[doc = "Bits 0:21 - GPIO input register for GPIO32-53"]
    #[inline(always)]
    pub fn data_next(&mut self) -> DATA_NEXT_W<'_, IN1_SPEC> {
        DATA_NEXT_W::new(self, 0)
    }
}
#[doc = "GPIO input register for GPIO32-53\n\nYou can [`read`](crate::Reg::read) this register and get [`in1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in1::R`](R) reader structure"]
impl crate::Readable for IN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in1::W`](W) writer structure"]
impl crate::Writable for IN1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN1 to value 0"]
impl crate::Resettable for IN1_SPEC {}
