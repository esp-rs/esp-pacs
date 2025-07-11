#[doc = "Register `TEXT_OUT_%s` reader"]
pub type R = crate::R<TEXT_OUT__SPEC>;
#[doc = "Register `TEXT_OUT_%s` writer"]
pub type W = crate::W<TEXT_OUT__SPEC>;
#[doc = "Field `TEXT_OUT_0` reader - This bits stores text_out_0 that is a part of result text material."]
pub type TEXT_OUT_0_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_OUT_0` writer - This bits stores text_out_0 that is a part of result text material."]
pub type TEXT_OUT_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_out_0 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_0(&self) -> TEXT_OUT_0_R {
        TEXT_OUT_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_OUT_")
            .field("text_out_0", &self.text_out_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_out_0 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_0(&mut self) -> TEXT_OUT_0_W<TEXT_OUT__SPEC> {
        TEXT_OUT_0_W::new(self, 0)
    }
}
#[doc = "Result text data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_out_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_out_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEXT_OUT__SPEC;
impl crate::RegisterSpec for TEXT_OUT__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_out_::R`](R) reader structure"]
impl crate::Readable for TEXT_OUT__SPEC {}
#[doc = "`write(|w| ..)` method takes [`text_out_::W`](W) writer structure"]
impl crate::Writable for TEXT_OUT__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEXT_OUT_%s to value 0"]
impl crate::Resettable for TEXT_OUT__SPEC {}
