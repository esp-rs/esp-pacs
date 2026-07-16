#[doc = "Register `DCMEM_VALID_3` reader"]
pub type R = crate::R<DCMEM_VALID_3_SPEC>;
#[doc = "Register `DCMEM_VALID_3` writer"]
pub type W = crate::W<DCMEM_VALID_3_SPEC>;
#[doc = "Field `DCMEM_VALID_3` reader - "]
pub type DCMEM_VALID_3_R = crate::FieldReader<u32>;
#[doc = "Field `DCMEM_VALID_3` writer - "]
pub type DCMEM_VALID_3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcmem_valid_3(&self) -> DCMEM_VALID_3_R {
        DCMEM_VALID_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCMEM_VALID_3")
            .field("dcmem_valid_3", &self.dcmem_valid_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcmem_valid_3(&mut self) -> DCMEM_VALID_3_W<'_, DCMEM_VALID_3_SPEC> {
        DCMEM_VALID_3_W::new(self, 0)
    }
}
#[doc = "DCMEM_VALID_3\n\nYou can [`read`](crate::Reg::read) this register and get [`dcmem_valid_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmem_valid_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMEM_VALID_3_SPEC;
impl crate::RegisterSpec for DCMEM_VALID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmem_valid_3::R`](R) reader structure"]
impl crate::Readable for DCMEM_VALID_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcmem_valid_3::W`](W) writer structure"]
impl crate::Writable for DCMEM_VALID_3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCMEM_VALID_3 to value 0"]
impl crate::Resettable for DCMEM_VALID_3_SPEC {}
