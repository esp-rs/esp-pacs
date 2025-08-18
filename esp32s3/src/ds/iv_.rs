#[doc = "Register `IV_%s` reader"]
pub type R = crate::R<IV__SPEC>;
#[doc = "Register `IV_%s` writer"]
pub type W = crate::W<IV__SPEC>;
#[doc = "Field `IV` reader - Stores IV block data"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - Stores IV block data"]
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores IV block data"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_").field("iv", &self.iv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores IV block data"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IV__SPEC> {
        IV_W::new(self, 0)
    }
}
#[doc = "IV block data\n\nYou can [`read`](crate::Reg::read) this register and get [`iv_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv_::R`](R) reader structure"]
impl crate::Readable for IV__SPEC {}
#[doc = "`write(|w| ..)` method takes [`iv_::W`](W) writer structure"]
impl crate::Writable for IV__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IV_%s to value 0"]
impl crate::Resettable for IV__SPEC {}
