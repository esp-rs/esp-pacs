#[doc = "Register `HCDMA3` reader"]
pub type R = crate::R<HCDMA3_SPEC>;
#[doc = "Register `HCDMA3` writer"]
pub type W = crate::W<HCDMA3_SPEC>;
#[doc = "Field `H_DMAADDR3` reader - "]
pub type H_DMAADDR3_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR3` writer - "]
pub type H_DMAADDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr3(&self) -> H_DMAADDR3_R {
        H_DMAADDR3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA3")
            .field("h_dmaaddr3", &format_args!("{}", self.h_dmaaddr3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr3(&mut self) -> H_DMAADDR3_W<HCDMA3_SPEC> {
        H_DMAADDR3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA3_SPEC;
impl crate::RegisterSpec for HCDMA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma3::R`](R) reader structure"]
impl crate::Readable for HCDMA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma3::W`](W) writer structure"]
impl crate::Writable for HCDMA3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA3 to value 0"]
impl crate::Resettable for HCDMA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
