#[doc = "Register `CACHE_SYNC_CTRL` reader"]
pub struct R(crate::R<CACHE_SYNC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SYNC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SYNC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SYNC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SYNC_CTRL` writer"]
pub struct W(crate::W<CACHE_SYNC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SYNC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CACHE_SYNC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SYNC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_INVALIDATE_ENA` reader - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_INVALIDATE_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_INVALIDATE_ENA` writer - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_INVALIDATE_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SYNC_CTRL_SPEC, O>;
#[doc = "Field `CACHE_CLEAN_ENA` reader - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_CLEAN_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_CLEAN_ENA` writer - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_CLEAN_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SYNC_CTRL_SPEC, O>;
#[doc = "Field `CACHE_WRITEBACK_ENA` reader - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_WRITEBACK_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_WRITEBACK_ENA` writer - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_WRITEBACK_ENA_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SYNC_CTRL_SPEC, O>;
#[doc = "Field `CACHE_WRITEBACK_INVALIDATE_ENA` reader - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_WRITEBACK_INVALIDATE_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_WRITEBACK_INVALIDATE_ENA` writer - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
pub type CACHE_WRITEBACK_INVALIDATE_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_SYNC_CTRL_SPEC, O>;
#[doc = "Field `CACHE_SYNC_DONE` reader - The bit is used to indicate whether sync operation (invalidate, clean, writeback, writeback_invalidate) is finished or not. 0: not finished. 1: finished."]
pub type CACHE_SYNC_DONE_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_RGID` reader - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
pub type CACHE_SYNC_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn cache_invalidate_ena(&self) -> CACHE_INVALIDATE_ENA_R {
        CACHE_INVALIDATE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn cache_clean_ena(&self) -> CACHE_CLEAN_ENA_R {
        CACHE_CLEAN_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn cache_writeback_ena(&self) -> CACHE_WRITEBACK_ENA_R {
        CACHE_WRITEBACK_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    pub fn cache_writeback_invalidate_ena(&self) -> CACHE_WRITEBACK_INVALIDATE_ENA_R {
        CACHE_WRITEBACK_INVALIDATE_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to indicate whether sync operation (invalidate, clean, writeback, writeback_invalidate) is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn cache_sync_done(&self) -> CACHE_SYNC_DONE_R {
        CACHE_SYNC_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - The bit is used to set the gid of cache sync operation (invalidate, clean, writeback, writeback_invalidate)"]
    #[inline(always)]
    pub fn cache_sync_rgid(&self) -> CACHE_SYNC_RGID_R {
        CACHE_SYNC_RGID_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_CTRL")
            .field(
                "cache_invalidate_ena",
                &format_args!("{}", self.cache_invalidate_ena().bit()),
            )
            .field(
                "cache_clean_ena",
                &format_args!("{}", self.cache_clean_ena().bit()),
            )
            .field(
                "cache_writeback_ena",
                &format_args!("{}", self.cache_writeback_ena().bit()),
            )
            .field(
                "cache_writeback_invalidate_ena",
                &format_args!("{}", self.cache_writeback_invalidate_ena().bit()),
            )
            .field(
                "cache_sync_done",
                &format_args!("{}", self.cache_sync_done().bit()),
            )
            .field(
                "cache_sync_rgid",
                &format_args!("{}", self.cache_sync_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_SYNC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done. Note that this bit and the other sync-bits (clean_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn cache_invalidate_ena(&mut self) -> CACHE_INVALIDATE_ENA_W<0> {
        CACHE_INVALIDATE_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done. Note that this bit and the other sync-bits (invalidate_ena, writeback_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn cache_clean_ena(&mut self) -> CACHE_CLEAN_ENA_W<1> {
        CACHE_CLEAN_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_invalidate_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn cache_writeback_ena(&mut self) -> CACHE_WRITEBACK_ENA_W<2> {
        CACHE_WRITEBACK_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to enable writeback-invalidate operation. It will be cleared by hardware after writeback-invalidate operation done. Note that this bit and the other sync-bits (invalidate_ena, clean_ena, writeback_ena) are mutually exclusive, that is, those bits can not be set to 1 at the same time."]
    #[inline(always)]
    #[must_use]
    pub fn cache_writeback_invalidate_ena(&mut self) -> CACHE_WRITEBACK_INVALIDATE_ENA_W<3> {
        CACHE_WRITEBACK_INVALIDATE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sync-class operation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_sync_ctrl](index.html) module"]
pub struct CACHE_SYNC_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_sync_ctrl::R](R) reader structure"]
impl crate::Readable for CACHE_SYNC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_sync_ctrl::W](W) writer structure"]
impl crate::Writable for CACHE_SYNC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_SYNC_CTRL to value 0x01"]
impl crate::Resettable for CACHE_SYNC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
