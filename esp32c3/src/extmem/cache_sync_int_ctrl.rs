#[doc = "Register `CACHE_SYNC_INT_CTRL` reader"]
pub type R = crate::R<CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "Register `CACHE_SYNC_INT_CTRL` writer"]
pub type W = crate::W<CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "Field `ST` reader - The bit is used to indicate the interrupt by icache sync done."]
pub type ST_R = crate::BitReader;
#[doc = "Field `ENA` reader - The bit is used to enable the interrupt by icache sync done."]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - The bit is used to enable the interrupt by icache sync done."]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR` writer - The bit is used to clear the interrupt by icache sync done."]
pub type CLR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to indicate the interrupt by icache sync done."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache sync done."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_INT_CTRL")
            .field("st", &self.st().bit())
            .field("ena", &self.ena().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_SYNC_INT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache sync done."]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<CACHE_SYNC_INT_CTRL_SPEC> {
        ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to clear the interrupt by icache sync done."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<CACHE_SYNC_INT_CTRL_SPEC> {
        CLR_W::new(self, 2)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_sync_int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sync_int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SYNC_INT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sync_int_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_SYNC_INT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_sync_int_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_SYNC_INT_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x04;
}
#[doc = "`reset()` method sets CACHE_SYNC_INT_CTRL to value 0"]
impl crate::Resettable for CACHE_SYNC_INT_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
