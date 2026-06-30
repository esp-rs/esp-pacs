#[doc = "Register `ENABLE2` reader"]
pub type R = crate::R<ENABLE2_SPEC>;
#[doc = "Register `ENABLE2` writer"]
pub type W = crate::W<ENABLE2_SPEC>;
#[doc = "Field `DATA` reader - Configures whether or not to enable the output of GPIO64 ~ GPIO66.\\\\ 0: Not enable\\\\ 1: Enable\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid.\\\\"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Configures whether or not to enable the output of GPIO64 ~ GPIO66.\\\\ 0: Not enable\\\\ 1: Enable\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid.\\\\"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Configures whether or not to enable the output of GPIO64 ~ GPIO66.\\\\ 0: Not enable\\\\ 1: Enable\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid.\\\\"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE2")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures whether or not to enable the output of GPIO64 ~ GPIO66.\\\\ 0: Not enable\\\\ 1: Enable\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid.\\\\"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, ENABLE2_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "GPIO output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE2_SPEC;
impl crate::RegisterSpec for ENABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable2::R`](R) reader structure"]
impl crate::Readable for ENABLE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable2::W`](W) writer structure"]
impl crate::Writable for ENABLE2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE2 to value 0"]
impl crate::Resettable for ENABLE2_SPEC {}
