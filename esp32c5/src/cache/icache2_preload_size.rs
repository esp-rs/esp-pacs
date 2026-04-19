#[doc = "Register `ICACHE2_PRELOAD_SIZE` reader"]
pub type R = crate::R<ICACHE2_PRELOAD_SIZE_SPEC>;
#[doc = "Register `ICACHE2_PRELOAD_SIZE` writer"]
pub type W = crate::W<ICACHE2_PRELOAD_SIZE_SPEC>;
#[doc = "Field `ICACHE2_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOAD_ADDR_REG"]
pub type ICACHE2_PRELOAD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ICACHE2_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOAD_ADDR_REG"]
pub type ICACHE2_PRELOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_preload_size(&self) -> ICACHE2_PRELOAD_SIZE_R {
        ICACHE2_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOAD_SIZE")
            .field("icache2_preload_size", &self.icache2_preload_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_preload_size(
        &mut self,
    ) -> ICACHE2_PRELOAD_SIZE_W<'_, ICACHE2_PRELOAD_SIZE_SPEC> {
        ICACHE2_PRELOAD_SIZE_W::new(self, 0)
    }
}
#[doc = "Instruction Cache 2 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_preload_size::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOAD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache2_preload_size::W`](W) writer structure"]
impl crate::Writable for ICACHE2_PRELOAD_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE2_PRELOAD_SIZE to value 0"]
impl crate::Resettable for ICACHE2_PRELOAD_SIZE_SPEC {}
