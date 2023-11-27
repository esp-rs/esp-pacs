#[doc = "Register `L1_ICACHE0_PRELOAD_SIZE` reader"]
pub type R = crate::R<L1_ICACHE0_PRELOAD_SIZE_SPEC>;
#[doc = "Register `L1_ICACHE0_PRELOAD_SIZE` writer"]
pub type W = crate::W<L1_ICACHE0_PRELOAD_SIZE_SPEC>;
#[doc = "Field `L1_ICACHE0_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
pub type L1_ICACHE0_PRELOAD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `L1_ICACHE0_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
pub type L1_ICACHE0_PRELOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache0_preload_size(&self) -> L1_ICACHE0_PRELOAD_SIZE_R {
        L1_ICACHE0_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE0_PRELOAD_SIZE")
            .field(
                "l1_icache0_preload_size",
                &format_args!("{}", self.l1_icache0_preload_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE0_PRELOAD_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_preload_size(
        &mut self,
    ) -> L1_ICACHE0_PRELOAD_SIZE_W<L1_ICACHE0_PRELOAD_SIZE_SPEC> {
        L1_ICACHE0_PRELOAD_SIZE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "L1 instruction Cache 0 preload size configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_preload_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_preload_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_ICACHE0_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_preload_size::R`](R) reader structure"]
impl crate::Readable for L1_ICACHE0_PRELOAD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_icache0_preload_size::W`](W) writer structure"]
impl crate::Writable for L1_ICACHE0_PRELOAD_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_ICACHE0_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L1_ICACHE0_PRELOAD_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
