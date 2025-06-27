#[doc = "Register `EXT_ADDR` reader"]
pub type R = crate::R<EXT_ADDR_SPEC>;
#[doc = "Register `EXT_ADDR` writer"]
pub type W = crate::W<EXT_ADDR_SPEC>;
#[doc = "Field `EXT_ADDR` reader - The register are the higher 32bits in the 64 bits address mode."]
pub type EXT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `EXT_ADDR` writer - The register are the higher 32bits in the 64 bits address mode."]
pub type EXT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The register are the higher 32bits in the 64 bits address mode."]
    #[inline(always)]
    pub fn ext_addr(&self) -> EXT_ADDR_R {
        EXT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_ADDR")
            .field("ext_addr", &self.ext_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The register are the higher 32bits in the 64 bits address mode."]
    #[inline(always)]
    pub fn ext_addr(&mut self) -> EXT_ADDR_W<EXT_ADDR_SPEC> {
        EXT_ADDR_W::new(self, 0)
    }
}
#[doc = "SPI1 extended address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_ADDR_SPEC;
impl crate::RegisterSpec for EXT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr::R`](R) reader structure"]
impl crate::Readable for EXT_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_addr::W`](W) writer structure"]
impl crate::Writable for EXT_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_ADDR to value 0"]
impl crate::Resettable for EXT_ADDR_SPEC {}
