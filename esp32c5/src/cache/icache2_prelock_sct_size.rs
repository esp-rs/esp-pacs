#[doc = "Register `ICACHE2_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<ICACHE2_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Register `ICACHE2_PRELOCK_SCT_SIZE` writer"]
pub type W = crate::W<ICACHE2_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Field `ICACHE2_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_ADDR_REG"]
pub type ICACHE2_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ICACHE2_PRELOCK_SCT0_SIZE` writer - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_ADDR_REG"]
pub type ICACHE2_PRELOCK_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `ICACHE2_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT1_ADDR_REG"]
pub type ICACHE2_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ICACHE2_PRELOCK_SCT1_SIZE` writer - Those bits are used to configure the size of the second section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT1_ADDR_REG"]
pub type ICACHE2_PRELOCK_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct0_size(&self) -> ICACHE2_PRELOCK_SCT0_SIZE_R {
        ICACHE2_PRELOCK_SCT0_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct1_size(&self) -> ICACHE2_PRELOCK_SCT1_SIZE_R {
        ICACHE2_PRELOCK_SCT1_SIZE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOCK_SCT_SIZE")
            .field(
                "icache2_prelock_sct0_size",
                &self.icache2_prelock_sct0_size(),
            )
            .field(
                "icache2_prelock_sct1_size",
                &self.icache2_prelock_sct1_size(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct0_size(
        &mut self,
    ) -> ICACHE2_PRELOCK_SCT0_SIZE_W<'_, ICACHE2_PRELOCK_SCT_SIZE_SPEC> {
        ICACHE2_PRELOCK_SCT0_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct1_size(
        &mut self,
    ) -> ICACHE2_PRELOCK_SCT1_SIZE_W<'_, ICACHE2_PRELOCK_SCT_SIZE_SPEC> {
        ICACHE2_PRELOCK_SCT1_SIZE_W::new(self, 16)
    }
}
#[doc = "Instruction Cache 2 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_sct_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOCK_SCT_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache2_prelock_sct_size::W`](W) writer structure"]
impl crate::Writable for ICACHE2_PRELOCK_SCT_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE2_PRELOCK_SCT_SIZE to value 0"]
impl crate::Resettable for ICACHE2_PRELOCK_SCT_SIZE_SPEC {}
