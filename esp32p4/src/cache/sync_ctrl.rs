#[doc = "Register `SYNC_CTRL` reader"]
pub type R = crate::R<SYNC_CTRL_SPEC>;
#[doc = "Register `SYNC_CTRL` writer"]
pub type W = crate::W<SYNC_CTRL_SPEC>;
#[doc = "Field `INVALIDATE_ENA` reader - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type INVALIDATE_ENA_R = crate::BitReader;
#[doc = "Field `INVALIDATE_ENA` writer - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type INVALIDATE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAN_ENA` reader - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CLEAN_ENA_R = crate::BitReader;
#[doc = "Field `CLEAN_ENA` writer - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CLEAN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEBACK_ENA` reader - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WRITEBACK_ENA_R = crate::BitReader;
#[doc = "Field `WRITEBACK_ENA` writer - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WRITEBACK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEBACK_INVALIDATE_ENA` reader - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WRITEBACK_INVALIDATE_ENA_R = crate::BitReader;
#[doc = "Field `WRITEBACK_INVALIDATE_ENA` writer - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type WRITEBACK_INVALIDATE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_DONE` reader - The bit is used to indicate whether sync operation (invalidate, clean, writeback, writeback_invalidate) is finished or not. 0: not finished. 1: finished."]
pub type SYNC_DONE_R = crate::BitReader;
#[doc = "Field `SYNC_RGID` reader - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
pub type SYNC_RGID_R = crate::FieldReader;
#[doc = "Field `SYNC_RGID` writer - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
pub type SYNC_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn invalidate_ena(&self) -> INVALIDATE_ENA_R {
        INVALIDATE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn clean_ena(&self) -> CLEAN_ENA_R {
        CLEAN_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn writeback_ena(&self) -> WRITEBACK_ENA_R {
        WRITEBACK_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn writeback_invalidate_ena(&self) -> WRITEBACK_INVALIDATE_ENA_R {
        WRITEBACK_INVALIDATE_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate whether sync operation (invalidate, clean, writeback, writeback_invalidate) is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn sync_done(&self) -> SYNC_DONE_R {
        SYNC_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
    #[inline(always)]
    pub fn sync_rgid(&self) -> SYNC_RGID_R {
        SYNC_RGID_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_CTRL")
            .field(
                "invalidate_ena",
                &format_args!("{}", self.invalidate_ena().bit()),
            )
            .field("clean_ena", &format_args!("{}", self.clean_ena().bit()))
            .field(
                "writeback_ena",
                &format_args!("{}", self.writeback_ena().bit()),
            )
            .field(
                "writeback_invalidate_ena",
                &format_args!("{}", self.writeback_invalidate_ena().bit()),
            )
            .field("sync_done", &format_args!("{}", self.sync_done().bit()))
            .field("sync_rgid", &format_args!("{}", self.sync_rgid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYNC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn invalidate_ena(&mut self) -> INVALIDATE_ENA_W<SYNC_CTRL_SPEC> {
        INVALIDATE_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn clean_ena(&mut self) -> CLEAN_ENA_W<SYNC_CTRL_SPEC> {
        CLEAN_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn writeback_ena(&mut self) -> WRITEBACK_ENA_W<SYNC_CTRL_SPEC> {
        WRITEBACK_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn writeback_invalidate_ena(&mut self) -> WRITEBACK_INVALIDATE_ENA_W<SYNC_CTRL_SPEC> {
        WRITEBACK_INVALIDATE_ENA_W::new(self, 3)
    }
    #[doc = "Bits 5:8 - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
    #[inline(always)]
    #[must_use]
    pub fn sync_rgid(&mut self) -> SYNC_RGID_W<SYNC_CTRL_SPEC> {
        SYNC_RGID_W::new(self, 5)
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
#[doc = "Sync-class operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_CTRL_SPEC;
impl crate::RegisterSpec for SYNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_ctrl::R`](R) reader structure"]
impl crate::Readable for SYNC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_ctrl::W`](W) writer structure"]
impl crate::Writable for SYNC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC_CTRL to value 0x01"]
impl crate::Resettable for SYNC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
