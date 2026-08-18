#[doc = "Register `FMEM_PMS%s_SIZE` reader"]
pub type R = crate::R<FMEM_PMS_SIZE_SPEC>;
#[doc = "Register `FMEM_PMS%s_SIZE` writer"]
pub type W = crate::W<FMEM_PMS_SIZE_SPEC>;
#[doc = "Field `SIZE` reader - "]
pub type SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `SIZE` writer - "]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMEM_PMS_SIZE")
            .field("size", &self.size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<'_, FMEM_PMS_SIZE_SPEC> {
        SIZE_W::new(self, 0)
    }
}
#[doc = "SPI1 flash PMS section 0 start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmem_pms_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmem_pms_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMEM_PMS_SIZE_SPEC;
impl crate::RegisterSpec for FMEM_PMS_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmem_pms_size::R`](R) reader structure"]
impl crate::Readable for FMEM_PMS_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmem_pms_size::W`](W) writer structure"]
impl crate::Writable for FMEM_PMS_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMEM_PMS%s_SIZE to value 0x1000"]
impl crate::Resettable for FMEM_PMS_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
