#[doc = "Register `SEC_KEY0` reader"]
pub type R = crate::R<SEC_KEY0_SPEC>;
#[doc = "Register `SEC_KEY0` writer"]
pub type W = crate::W<SEC_KEY0_SPEC>;
#[doc = "Field `SEC_KEY0` reader - "]
pub type SEC_KEY0_R = crate::FieldReader<u32>;
#[doc = "Field `SEC_KEY0` writer - "]
pub type SEC_KEY0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sec_key0(&self) -> SEC_KEY0_R {
        SEC_KEY0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_KEY0")
            .field("sec_key0", &self.sec_key0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sec_key0(&mut self) -> SEC_KEY0_W<SEC_KEY0_SPEC> {
        SEC_KEY0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_key0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_key0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_KEY0_SPEC;
impl crate::RegisterSpec for SEC_KEY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_key0::R`](R) reader structure"]
impl crate::Readable for SEC_KEY0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sec_key0::W`](W) writer structure"]
impl crate::Writable for SEC_KEY0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_KEY0 to value 0"]
impl crate::Resettable for SEC_KEY0_SPEC {
    const RESET_VALUE: u32 = 0;
}
