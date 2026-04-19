#[doc = "Register `ICACHE2_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Register `ICACHE2_AUTOLOAD_SCT1_SIZE` writer"]
pub type W = crate::W<ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Field `ICACHE2_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on ICache2. Note that it should be used together with ICACHE2_AUTOLOAD_SCT1_ADDR and ICACHE_AUTOLOAD_SCT1_ENA."]
pub type ICACHE2_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `ICACHE2_AUTOLOAD_SCT1_SIZE` writer - Those bits are used to configure the size of the second section for autoload operation on ICache2. Note that it should be used together with ICACHE2_AUTOLOAD_SCT1_ADDR and ICACHE_AUTOLOAD_SCT1_ENA."]
pub type ICACHE2_AUTOLOAD_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on ICache2. Note that it should be used together with ICACHE2_AUTOLOAD_SCT1_ADDR and ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn icache2_autoload_sct1_size(&self) -> ICACHE2_AUTOLOAD_SCT1_SIZE_R {
        ICACHE2_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_AUTOLOAD_SCT1_SIZE")
            .field(
                "icache2_autoload_sct1_size",
                &self.icache2_autoload_sct1_size(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on ICache2. Note that it should be used together with ICACHE2_AUTOLOAD_SCT1_ADDR and ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn icache2_autoload_sct1_size(
        &mut self,
    ) -> ICACHE2_AUTOLOAD_SCT1_SIZE_W<'_, ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC> {
        ICACHE2_AUTOLOAD_SCT1_SIZE_W::new(self, 0)
    }
}
#[doc = "Instruction Cache 2 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct1_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_autoload_sct1_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache2_autoload_sct1_size::W`](W) writer structure"]
impl crate::Writable for ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE2_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC {}
