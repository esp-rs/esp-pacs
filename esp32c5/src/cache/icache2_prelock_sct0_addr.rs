#[doc = "Register `ICACHE2_PRELOCK_SCT0_ADDR` reader"]
pub type R = crate::R<ICACHE2_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "Register `ICACHE2_PRELOCK_SCT0_ADDR` writer"]
pub type W = crate::W<ICACHE2_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "Field `ICACHE2_PRELOCK_SCT0_ADDR` reader - Those bits are used to configure the start address of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_SIZE_REG"]
pub type ICACHE2_PRELOCK_SCT0_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ICACHE2_PRELOCK_SCT0_ADDR` writer - Those bits are used to configure the start address of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_SIZE_REG"]
pub type ICACHE2_PRELOCK_SCT0_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_SIZE_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct0_addr(&self) -> ICACHE2_PRELOCK_SCT0_ADDR_R {
        ICACHE2_PRELOCK_SCT0_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOCK_SCT0_ADDR")
            .field(
                "icache2_prelock_sct0_addr",
                &self.icache2_prelock_sct0_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the first section of prelock on ICache2, which should be used together with ICACHE2_PRELOCK_SCT0_SIZE_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct0_addr(
        &mut self,
    ) -> ICACHE2_PRELOCK_SCT0_ADDR_W<'_, ICACHE2_PRELOCK_SCT0_ADDR_SPEC> {
        ICACHE2_PRELOCK_SCT0_ADDR_W::new(self, 0)
    }
}
#[doc = "Instruction Cache 2 prelock section0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache2_prelock_sct0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOCK_SCT0_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOCK_SCT0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_prelock_sct0_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOCK_SCT0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache2_prelock_sct0_addr::W`](W) writer structure"]
impl crate::Writable for ICACHE2_PRELOCK_SCT0_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICACHE2_PRELOCK_SCT0_ADDR to value 0"]
impl crate::Resettable for ICACHE2_PRELOCK_SCT0_ADDR_SPEC {}
