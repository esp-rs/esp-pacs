#[doc = "Register `QZ_MEM%s` reader"]
pub type R = crate::R<QZ_MEM_SPEC>;
#[doc = "Register `QZ_MEM%s` writer"]
pub type W = crate::W<QZ_MEM_SPEC>;
#[doc = "Field `WORD` reader - "]
pub type WORD_R = crate::FieldReader<u32>;
#[doc = "Field `WORD` writer - "]
pub type WORD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QZ_MEM")
            .field("word", &self.word())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W<'_, QZ_MEM_SPEC> {
        WORD_W::new(self, 0)
    }
}
#[doc = "ECC Qz coordinate RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`qz_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qz_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QZ_MEM_SPEC;
impl crate::RegisterSpec for QZ_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qz_mem::R`](R) reader structure"]
impl crate::Readable for QZ_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qz_mem::W`](W) writer structure"]
impl crate::Writable for QZ_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QZ_MEM%s to value 0"]
impl crate::Resettable for QZ_MEM_SPEC {}
