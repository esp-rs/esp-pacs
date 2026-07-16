#[doc = "Register `REGISTER3_HASHTABLELOWREGISTER` reader"]
pub type R = crate::R<REGISTER3_HASHTABLELOWREGISTER_SPEC>;
#[doc = "Register `REGISTER3_HASHTABLELOWREGISTER` writer"]
pub type W = crate::W<REGISTER3_HASHTABLELOWREGISTER_SPEC>;
#[doc = "Field `HTL` reader - Hash Table Low This field contains the lower 32 bits of the Hash table"]
pub type HTL_R = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash Table Low This field contains the lower 32 bits of the Hash table"]
pub type HTL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Table Low This field contains the lower 32 bits of the Hash table"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER3_HASHTABLELOWREGISTER")
            .field("htl", &self.htl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low This field contains the lower 32 bits of the Hash table"]
    #[inline(always)]
    pub fn htl(&mut self) -> HTL_W<'_, REGISTER3_HASHTABLELOWREGISTER_SPEC> {
        HTL_W::new(self, 0)
    }
}
#[doc = "Contains the lower 32 bits of the Multicast Hash table This register is present only when you select the Hash filter function in coreConsultant _See Table 79_\n\nYou can [`read`](crate::Reg::read) this register and get [`register3_hashtablelowregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register3_hashtablelowregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER3_HASHTABLELOWREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER3_HASHTABLELOWREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register3_hashtablelowregister::R`](R) reader structure"]
impl crate::Readable for REGISTER3_HASHTABLELOWREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register3_hashtablelowregister::W`](W) writer structure"]
impl crate::Writable for REGISTER3_HASHTABLELOWREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER3_HASHTABLELOWREGISTER to value 0"]
impl crate::Resettable for REGISTER3_HASHTABLELOWREGISTER_SPEC {}
