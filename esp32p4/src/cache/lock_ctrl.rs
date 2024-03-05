#[doc = "Register `LOCK_CTRL` reader"]
pub type R = crate::R<LOCK_CTRL_SPEC>;
#[doc = "Register `LOCK_CTRL` writer"]
pub type W = crate::W<LOCK_CTRL_SPEC>;
#[doc = "Field `LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done. Note that (1) this bit and unlock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) lock operation can be applied on LL1-ICache, L1-DCache and L2-Cache."]
pub type LOCK_ENA_R = crate::BitReader;
#[doc = "Field `LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done. Note that (1) this bit and unlock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) lock operation can be applied on LL1-ICache, L1-DCache and L2-Cache."]
pub type LOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done. Note that (1) this bit and lock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) unlock operation can be applied on L1-ICache, L1-DCache and L2-Cache."]
pub type UNLOCK_ENA_R = crate::BitReader;
#[doc = "Field `UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done. Note that (1) this bit and lock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) unlock operation can be applied on L1-ICache, L1-DCache and L2-Cache."]
pub type UNLOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK_DONE` reader - The bit is used to indicate whether unlock/lock operation is finished or not. 0: not finished. 1: finished."]
pub type LOCK_DONE_R = crate::BitReader;
#[doc = "Field `LOCK_RGID` reader - The bit is used to set the gid of cache lock/unlock."]
pub type LOCK_RGID_R = crate::FieldReader;
#[doc = "Field `LOCK_RGID` writer - The bit is used to set the gid of cache lock/unlock."]
pub type LOCK_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done. Note that (1) this bit and unlock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) lock operation can be applied on LL1-ICache, L1-DCache and L2-Cache."]
    #[inline(always)]
    pub fn lock_ena(&self) -> LOCK_ENA_R {
        LOCK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done. Note that (1) this bit and lock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) unlock operation can be applied on L1-ICache, L1-DCache and L2-Cache."]
    #[inline(always)]
    pub fn unlock_ena(&self) -> UNLOCK_ENA_R {
        UNLOCK_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate whether unlock/lock operation is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn lock_done(&self) -> LOCK_DONE_R {
        LOCK_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of cache lock/unlock."]
    #[inline(always)]
    pub fn lock_rgid(&self) -> LOCK_RGID_R {
        LOCK_RGID_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK_CTRL")
            .field("lock_ena", &format_args!("{}", self.lock_ena().bit()))
            .field("unlock_ena", &format_args!("{}", self.unlock_ena().bit()))
            .field("lock_done", &format_args!("{}", self.lock_done().bit()))
            .field("lock_rgid", &format_args!("{}", self.lock_rgid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOCK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done. Note that (1) this bit and unlock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) lock operation can be applied on LL1-ICache, L1-DCache and L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn lock_ena(&mut self) -> LOCK_ENA_W<LOCK_CTRL_SPEC> {
        LOCK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done. Note that (1) this bit and lock_ena bit are mutually exclusive, that is, those bits can not be set to 1 at the same time. (2) unlock operation can be applied on L1-ICache, L1-DCache and L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn unlock_ena(&mut self) -> UNLOCK_ENA_W<LOCK_CTRL_SPEC> {
        UNLOCK_ENA_W::new(self, 1)
    }
    #[doc = "Bits 3:6 - The bit is used to set the gid of cache lock/unlock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_rgid(&mut self) -> LOCK_RGID_W<LOCK_CTRL_SPEC> {
        LOCK_RGID_W::new(self, 3)
    }
}
#[doc = "Lock-class (manual lock) operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_CTRL_SPEC;
impl crate::RegisterSpec for LOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock_ctrl::R`](R) reader structure"]
impl crate::Readable for LOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock_ctrl::W`](W) writer structure"]
impl crate::Writable for LOCK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK_CTRL to value 0x04"]
impl crate::Resettable for LOCK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
