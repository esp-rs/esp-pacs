#[doc = "Register `DCACHE_SYNC_CTRL` reader"]
pub type R = crate::R<DCACHE_SYNC_CTRL_SPEC>;
#[doc = "Register `DCACHE_SYNC_CTRL` writer"]
pub type W = crate::W<DCACHE_SYNC_CTRL_SPEC>;
#[doc = "Field `DCACHE_INVALIDATE_ENA` reader - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
pub type DCACHE_INVALIDATE_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_INVALIDATE_ENA` writer - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
pub type DCACHE_INVALIDATE_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCACHE_WRITEBACK_ENA` reader - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done."]
pub type DCACHE_WRITEBACK_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_WRITEBACK_ENA` writer - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done."]
pub type DCACHE_WRITEBACK_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCACHE_CLEAN_ENA` reader - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done."]
pub type DCACHE_CLEAN_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_CLEAN_ENA` writer - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done."]
pub type DCACHE_CLEAN_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCACHE_SYNC_DONE` reader - The bit is used to indicate clean/writeback/invalidate operation is finished."]
pub type DCACHE_SYNC_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    #[inline(always)]
    pub fn dcache_invalidate_ena(&self) -> DCACHE_INVALIDATE_ENA_R {
        DCACHE_INVALIDATE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done."]
    #[inline(always)]
    pub fn dcache_writeback_ena(&self) -> DCACHE_WRITEBACK_ENA_R {
        DCACHE_WRITEBACK_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done."]
    #[inline(always)]
    pub fn dcache_clean_ena(&self) -> DCACHE_CLEAN_ENA_R {
        DCACHE_CLEAN_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate clean/writeback/invalidate operation is finished."]
    #[inline(always)]
    pub fn dcache_sync_done(&self) -> DCACHE_SYNC_DONE_R {
        DCACHE_SYNC_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_SYNC_CTRL")
            .field(
                "dcache_invalidate_ena",
                &format_args!("{}", self.dcache_invalidate_ena().bit()),
            )
            .field(
                "dcache_writeback_ena",
                &format_args!("{}", self.dcache_writeback_ena().bit()),
            )
            .field(
                "dcache_clean_ena",
                &format_args!("{}", self.dcache_clean_ena().bit()),
            )
            .field(
                "dcache_sync_done",
                &format_args!("{}", self.dcache_sync_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_SYNC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable invalidate operation. It will be cleared by hardware after invalidate operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_invalidate_ena(&mut self) -> DCACHE_INVALIDATE_ENA_W<DCACHE_SYNC_CTRL_SPEC, 0> {
        DCACHE_INVALIDATE_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable writeback operation. It will be cleared by hardware after writeback operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_writeback_ena(&mut self) -> DCACHE_WRITEBACK_ENA_W<DCACHE_SYNC_CTRL_SPEC, 1> {
        DCACHE_WRITEBACK_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to enable clean operation. It will be cleared by hardware after clean operation done."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_clean_ena(&mut self) -> DCACHE_CLEAN_ENA_W<DCACHE_SYNC_CTRL_SPEC, 2> {
        DCACHE_CLEAN_ENA_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_sync_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_sync_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_SYNC_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_SYNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_sync_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_SYNC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_sync_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_SYNC_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_SYNC_CTRL to value 0x01"]
impl crate::Resettable for DCACHE_SYNC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
