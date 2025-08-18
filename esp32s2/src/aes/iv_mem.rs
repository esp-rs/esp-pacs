#[doc = "Register `IV_MEM[%s]` reader"]
pub type R = crate::R<IV_MEM_SPEC>;
#[doc = "Register `IV_MEM[%s]` writer"]
pub type W = crate::W<IV_MEM_SPEC>;
#[doc = "Field `IV` reader - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
pub type IV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_MEM").field("iv", &self.iv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<'_, IV_MEM_SPEC> {
        IV_W::new(self, 0)
    }
}
#[doc = "initialization vector\n\nYou can [`read`](crate::Reg::read) this register and get [`iv_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV_MEM_SPEC;
impl crate::RegisterSpec for IV_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv_mem::R`](R) reader structure"]
impl crate::Readable for IV_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iv_mem::W`](W) writer structure"]
impl crate::Writable for IV_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IV_MEM[%s] to value 0"]
impl crate::Resettable for IV_MEM_SPEC {}
