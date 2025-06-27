#[doc = "Register `SEC_KEY3` reader"]
pub type R = crate::R<SEC_KEY3_SPEC>;
#[doc = "Register `SEC_KEY3` writer"]
pub type W = crate::W<SEC_KEY3_SPEC>;
#[doc = "Field `SEC_KEY3` reader - "]
pub type SEC_KEY3_R = crate::FieldReader<u32>;
#[doc = "Field `SEC_KEY3` writer - "]
pub type SEC_KEY3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sec_key3(&self) -> SEC_KEY3_R {
        SEC_KEY3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_KEY3")
            .field("sec_key3", &self.sec_key3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sec_key3(&mut self) -> SEC_KEY3_W<SEC_KEY3_SPEC> {
        SEC_KEY3_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_key3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_key3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_KEY3_SPEC;
impl crate::RegisterSpec for SEC_KEY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_key3::R`](R) reader structure"]
impl crate::Readable for SEC_KEY3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sec_key3::W`](W) writer structure"]
impl crate::Writable for SEC_KEY3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC_KEY3 to value 0"]
impl crate::Resettable for SEC_KEY3_SPEC {}
