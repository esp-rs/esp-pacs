#[doc = "Register `STATIC` reader"]
pub type R = crate::R<STATIC_SPEC>;
#[doc = "Register `STATIC` writer"]
pub type W = crate::W<STATIC_SPEC>;
#[doc = "Field `KEY_SOURCE` reader - digital signature key source bit. \\\\ 1'b0: key is from hmac.\\\\ 1'b1: key is from key manager. \\\\"]
pub type KEY_SOURCE_R = crate::BitReader;
#[doc = "Field `KEY_SOURCE` writer - digital signature key source bit. \\\\ 1'b0: key is from hmac.\\\\ 1'b1: key is from key manager. \\\\"]
pub type KEY_SOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIP_PADDING_CHECK` reader - digital signature padding check bypass bit. \\\\ 1'b0: padding check enabled.\\\\ 1'b1: padding check skipped. \\\\"]
pub type SKIP_PADDING_CHECK_R = crate::BitReader;
#[doc = "Field `SKIP_PADDING_CHECK` writer - digital signature padding check bypass bit. \\\\ 1'b0: padding check enabled.\\\\ 1'b1: padding check skipped. \\\\"]
pub type SKIP_PADDING_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - digital signature key source bit. \\\\ 1'b0: key is from hmac.\\\\ 1'b1: key is from key manager. \\\\"]
    #[inline(always)]
    pub fn key_source(&self) -> KEY_SOURCE_R {
        KEY_SOURCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - digital signature padding check bypass bit. \\\\ 1'b0: padding check enabled.\\\\ 1'b1: padding check skipped. \\\\"]
    #[inline(always)]
    pub fn skip_padding_check(&self) -> SKIP_PADDING_CHECK_R {
        SKIP_PADDING_CHECK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATIC")
            .field("key_source", &self.key_source())
            .field("skip_padding_check", &self.skip_padding_check())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - digital signature key source bit. \\\\ 1'b0: key is from hmac.\\\\ 1'b1: key is from key manager. \\\\"]
    #[inline(always)]
    pub fn key_source(&mut self) -> KEY_SOURCE_W<'_, STATIC_SPEC> {
        KEY_SOURCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - digital signature padding check bypass bit. \\\\ 1'b0: padding check enabled.\\\\ 1'b1: padding check skipped. \\\\"]
    #[inline(always)]
    pub fn skip_padding_check(&mut self) -> SKIP_PADDING_CHECK_W<'_, STATIC_SPEC> {
        SKIP_PADDING_CHECK_W::new(self, 1)
    }
}
#[doc = "DS configure key source register\n\nYou can [`read`](crate::Reg::read) this register and get [`static_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATIC_SPEC;
impl crate::RegisterSpec for STATIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static_::R`](R) reader structure"]
impl crate::Readable for STATIC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`static_::W`](W) writer structure"]
impl crate::Writable for STATIC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATIC to value 0"]
impl crate::Resettable for STATIC_SPEC {}
