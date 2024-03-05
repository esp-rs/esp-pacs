#[doc = "Register `ICACHE_LOCK_CTRL` reader"]
pub type R = crate::R<ICACHE_LOCK_CTRL_SPEC>;
#[doc = "Register `ICACHE_LOCK_CTRL` writer"]
pub type W = crate::W<ICACHE_LOCK_CTRL_SPEC>;
#[doc = "Field `ICACHE_LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type ICACHE_LOCK_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type ICACHE_LOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type ICACHE_UNLOCK_ENA_R = crate::BitReader;
#[doc = "Field `ICACHE_UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type ICACHE_UNLOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE_LOCK_DONE` reader - The bit is used to indicate unlock/lock operation is finished."]
pub type ICACHE_LOCK_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn icache_lock_ena(&self) -> ICACHE_LOCK_ENA_R {
        ICACHE_LOCK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn icache_unlock_ena(&self) -> ICACHE_UNLOCK_ENA_R {
        ICACHE_UNLOCK_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate unlock/lock operation is finished."]
    #[inline(always)]
    pub fn icache_lock_done(&self) -> ICACHE_LOCK_DONE_R {
        ICACHE_LOCK_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_LOCK_CTRL")
            .field(
                "icache_lock_ena",
                &format_args!("{}", self.icache_lock_ena().bit()),
            )
            .field(
                "icache_unlock_ena",
                &format_args!("{}", self.icache_unlock_ena().bit()),
            )
            .field(
                "icache_lock_done",
                &format_args!("{}", self.icache_lock_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_LOCK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    #[must_use]
    pub fn icache_lock_ena(&mut self) -> ICACHE_LOCK_ENA_W<ICACHE_LOCK_CTRL_SPEC> {
        ICACHE_LOCK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    #[must_use]
    pub fn icache_unlock_ena(&mut self) -> ICACHE_UNLOCK_ENA_W<ICACHE_LOCK_CTRL_SPEC> {
        ICACHE_UNLOCK_ENA_W::new(self, 1)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icache_lock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_lock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_LOCK_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_LOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_lock_ctrl::R`](R) reader structure"]
impl crate::Readable for ICACHE_LOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icache_lock_ctrl::W`](W) writer structure"]
impl crate::Writable for ICACHE_LOCK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICACHE_LOCK_CTRL to value 0x04"]
impl crate::Resettable for ICACHE_LOCK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
