#[doc = "Register `OUT` reader"]
pub type R = crate::R<OUT_SPEC>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OUT_SPEC>;
#[doc = "Field `DATA_ORIG` reader - GPIO output register for GPIO0-25"]
pub type DATA_ORIG_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_ORIG` writer - GPIO output register for GPIO0-25"]
pub type DATA_ORIG_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - GPIO output register for GPIO0-25"]
    #[inline(always)]
    pub fn data_orig(&self) -> DATA_ORIG_R {
        DATA_ORIG_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT")
            .field("data_orig", &format_args!("{}", self.data_orig().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:25 - GPIO output register for GPIO0-25"]
    #[inline(always)]
    #[must_use]
    pub fn data_orig(&mut self) -> DATA_ORIG_W<OUT_SPEC> {
        DATA_ORIG_W::new(self, 0)
    }
}
#[doc = "GPIO output register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SPEC;
impl crate::RegisterSpec for OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
