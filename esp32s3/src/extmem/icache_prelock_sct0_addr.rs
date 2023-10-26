#[doc = "Register `ICACHE_PRELOCK_SCT0_ADDR` reader"]
pub type R = crate::R<ICACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "Register `ICACHE_PRELOCK_SCT0_ADDR` writer"]
pub type W = crate::W<ICACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "Field `ICACHE_PRELOCK_SCT0_ADDR` reader - The bits are used to configure the first start virtual address of data prelock, which is combined with ICACHE_PRELOCK_SCT0_SIZE_REG"]
pub type ICACHE_PRELOCK_SCT0_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ICACHE_PRELOCK_SCT0_ADDR` writer - The bits are used to configure the first start virtual address of data prelock, which is combined with ICACHE_PRELOCK_SCT0_SIZE_REG"]
pub type ICACHE_PRELOCK_SCT0_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the first start virtual address of data prelock, which is combined with ICACHE_PRELOCK_SCT0_SIZE_REG"]
    #[inline(always)]
    pub fn icache_prelock_sct0_addr(&self) -> ICACHE_PRELOCK_SCT0_ADDR_R {
        ICACHE_PRELOCK_SCT0_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_PRELOCK_SCT0_ADDR")
            .field(
                "icache_prelock_sct0_addr",
                &format_args!("{}", self.icache_prelock_sct0_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_PRELOCK_SCT0_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the first start virtual address of data prelock, which is combined with ICACHE_PRELOCK_SCT0_SIZE_REG"]
    #[inline(always)]
    #[must_use]
    pub fn icache_prelock_sct0_addr(
        &mut self,
    ) -> ICACHE_PRELOCK_SCT0_ADDR_W<ICACHE_PRELOCK_SCT0_ADDR_SPEC, 0> {
        ICACHE_PRELOCK_SCT0_ADDR_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_sct0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_sct0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_PRELOCK_SCT0_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE_PRELOCK_SCT0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_prelock_sct0_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE_PRELOCK_SCT0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_prelock_sct0_addr::W`](W) writer structure"]
impl crate::Writable for ICACHE_PRELOCK_SCT0_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_PRELOCK_SCT0_ADDR to value 0"]
impl crate::Resettable for ICACHE_PRELOCK_SCT0_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
