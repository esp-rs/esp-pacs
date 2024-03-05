#[doc = "Register `TEXT_IN_3` reader"]
pub type R = crate::R<TEXT_IN_3_SPEC>;
#[doc = "Register `TEXT_IN_3` writer"]
pub type W = crate::W<TEXT_IN_3_SPEC>;
#[doc = "Field `TEXT_IN_3` reader - This bits stores text_in_3 that is a part of source text material."]
pub type TEXT_IN_3_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_IN_3` writer - This bits stores text_in_3 that is a part of source text material."]
pub type TEXT_IN_3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_in_3 that is a part of source text material."]
    #[inline(always)]
    pub fn text_in_3(&self) -> TEXT_IN_3_R {
        TEXT_IN_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_IN_3")
            .field("text_in_3", &format_args!("{}", self.text_in_3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEXT_IN_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_in_3 that is a part of source text material."]
    #[inline(always)]
    #[must_use]
    pub fn text_in_3(&mut self) -> TEXT_IN_3_W<TEXT_IN_3_SPEC> {
        TEXT_IN_3_W::new(self, 0)
    }
}
#[doc = "source text material text_in_3 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEXT_IN_3_SPEC;
impl crate::RegisterSpec for TEXT_IN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`text_in_3::R`](R) reader structure"]
impl crate::Readable for TEXT_IN_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`text_in_3::W`](W) writer structure"]
impl crate::Writable for TEXT_IN_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEXT_IN_3 to value 0"]
impl crate::Resettable for TEXT_IN_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
