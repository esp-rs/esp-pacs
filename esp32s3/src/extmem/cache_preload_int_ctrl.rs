#[doc = "Register `CACHE_PRELOAD_INT_CTRL` reader"]
pub type R = crate::R<CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "Register `CACHE_PRELOAD_INT_CTRL` writer"]
pub type W = crate::W<CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "Field `ICACHE_PRELOAD_INT_ST` reader - The bit is used to indicate the interrupt by icache pre-load done."]
pub type ICACHE_PRELOAD_INT_ST_R = crate::BitReader;
#[doc = "Field `ICACHE_PRELOAD_INT_ENA` reader - The bit is used to enable the interrupt by icache pre-load done."]
pub type ICACHE_PRELOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_PRELOAD_INT_ENA` writer - The bit is used to enable the interrupt by icache pre-load done."]
pub type ICACHE_PRELOAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_PRELOAD_INT_CLR` writer - The bit is used to clear the interrupt by icache pre-load done."]
pub type ICACHE_PRELOAD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_PRELOAD_INT_ST` reader - The bit is used to indicate the interrupt by dcache pre-load done."]
pub type DCACHE_PRELOAD_INT_ST_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_INT_ENA` reader - The bit is used to enable the interrupt by dcache pre-load done."]
pub type DCACHE_PRELOAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_PRELOAD_INT_ENA` writer - The bit is used to enable the interrupt by dcache pre-load done."]
pub type DCACHE_PRELOAD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_PRELOAD_INT_CLR` writer - The bit is used to clear the interrupt by dcache pre-load done."]
pub type DCACHE_PRELOAD_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to indicate the interrupt by icache pre-load done."]
    #[inline(always)]
    pub fn icache_preload_int_st(&self) -> ICACHE_PRELOAD_INT_ST_R {
        ICACHE_PRELOAD_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache pre-load done."]
    #[inline(always)]
    pub fn icache_preload_int_ena(&self) -> ICACHE_PRELOAD_INT_ENA_R {
        ICACHE_PRELOAD_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate the interrupt by dcache pre-load done."]
    #[inline(always)]
    pub fn dcache_preload_int_st(&self) -> DCACHE_PRELOAD_INT_ST_R {
        DCACHE_PRELOAD_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt by dcache pre-load done."]
    #[inline(always)]
    pub fn dcache_preload_int_ena(&self) -> DCACHE_PRELOAD_INT_ENA_R {
        DCACHE_PRELOAD_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_PRELOAD_INT_CTRL")
            .field(
                "icache_preload_int_st",
                &format_args!("{}", self.icache_preload_int_st().bit()),
            )
            .field(
                "icache_preload_int_ena",
                &format_args!("{}", self.icache_preload_int_ena().bit()),
            )
            .field(
                "dcache_preload_int_st",
                &format_args!("{}", self.dcache_preload_int_st().bit()),
            )
            .field(
                "dcache_preload_int_ena",
                &format_args!("{}", self.dcache_preload_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_PRELOAD_INT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn icache_preload_int_ena(
        &mut self,
    ) -> ICACHE_PRELOAD_INT_ENA_W<CACHE_PRELOAD_INT_CTRL_SPEC> {
        ICACHE_PRELOAD_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to clear the interrupt by icache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn icache_preload_int_clr(
        &mut self,
    ) -> ICACHE_PRELOAD_INT_CLR_W<CACHE_PRELOAD_INT_CTRL_SPEC> {
        ICACHE_PRELOAD_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt by dcache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_int_ena(
        &mut self,
    ) -> DCACHE_PRELOAD_INT_ENA_W<CACHE_PRELOAD_INT_CTRL_SPEC> {
        DCACHE_PRELOAD_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to clear the interrupt by dcache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_preload_int_clr(
        &mut self,
    ) -> DCACHE_PRELOAD_INT_CLR_W<CACHE_PRELOAD_INT_CTRL_SPEC> {
        DCACHE_PRELOAD_INT_CLR_W::new(self, 5)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_preload_int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_preload_int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_PRELOAD_INT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_PRELOAD_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_preload_int_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_PRELOAD_INT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_preload_int_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_PRELOAD_INT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_PRELOAD_INT_CTRL to value 0"]
impl crate::Resettable for CACHE_PRELOAD_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
