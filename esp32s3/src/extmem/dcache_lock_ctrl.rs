#[doc = "Register `DCACHE_LOCK_CTRL` reader"]
pub type R = crate::R<DCACHE_LOCK_CTRL_SPEC>;
#[doc = "Register `DCACHE_LOCK_CTRL` writer"]
pub type W = crate::W<DCACHE_LOCK_CTRL_SPEC>;
#[doc = "Field `DCACHE_LOCK_ENA` reader - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type DCACHE_LOCK_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_LOCK_ENA` writer - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
pub type DCACHE_LOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_UNLOCK_ENA` reader - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type DCACHE_UNLOCK_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_UNLOCK_ENA` writer - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
pub type DCACHE_UNLOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_LOCK_DONE` reader - The bit is used to indicate unlock/lock operation is finished."]
pub type DCACHE_LOCK_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    pub fn dcache_lock_ena(&self) -> DCACHE_LOCK_ENA_R {
        DCACHE_LOCK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    pub fn dcache_unlock_ena(&self) -> DCACHE_UNLOCK_ENA_R {
        DCACHE_UNLOCK_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate unlock/lock operation is finished."]
    #[inline(always)]
    pub fn dcache_lock_done(&self) -> DCACHE_LOCK_DONE_R {
        DCACHE_LOCK_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_LOCK_CTRL")
            .field("dcache_lock_ena", &self.dcache_lock_ena())
            .field("dcache_unlock_ena", &self.dcache_unlock_ena())
            .field("dcache_lock_done", &self.dcache_lock_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable lock operation. It will be cleared by hardware after lock operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_lock_ena(&mut self) -> DCACHE_LOCK_ENA_W<DCACHE_LOCK_CTRL_SPEC> {
        DCACHE_LOCK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable unlock operation. It will be cleared by hardware after unlock operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_unlock_ena(&mut self) -> DCACHE_UNLOCK_ENA_W<DCACHE_LOCK_CTRL_SPEC> {
        DCACHE_UNLOCK_ENA_W::new(self, 1)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_lock_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_lock_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_LOCK_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_LOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_lock_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_LOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_lock_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_LOCK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_LOCK_CTRL to value 0x04"]
impl crate::Resettable for DCACHE_LOCK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
