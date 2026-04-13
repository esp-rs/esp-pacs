#[doc = "Register `RGB2GRAY` reader"]
pub type R = crate::R<RGB2GRAY_SPEC>;
#[doc = "Register `RGB2GRAY` writer"]
pub type W = crate::W<RGB2GRAY_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGB2GRAY")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, RGB2GRAY_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rgb2gray::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgb2gray::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGB2GRAY_SPEC;
impl crate::RegisterSpec for RGB2GRAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgb2gray::R`](R) reader structure"]
impl crate::Readable for RGB2GRAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rgb2gray::W`](W) writer structure"]
impl crate::Writable for RGB2GRAY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RGB2GRAY to value 0"]
impl crate::Resettable for RGB2GRAY_SPEC {}
