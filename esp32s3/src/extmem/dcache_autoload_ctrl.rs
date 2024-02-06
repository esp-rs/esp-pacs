#[doc = "Register `DCACHE_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<DCACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Register `DCACHE_AUTOLOAD_CTRL` writer"]
pub type W = crate::W<DCACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the first section for autoload operation."]
pub type DCACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the first section for autoload operation."]
pub type DCACHE_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the second section for autoload operation."]
pub type DCACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the second section for autoload operation."]
pub type DCACHE_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
pub type DCACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
pub type DCACHE_AUTOLOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_AUTOLOAD_DONE` reader - The bit is used to indicate autoload operation is finished."]
pub type DCACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `DCACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub type DCACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `DCACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
pub type DCACHE_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type DCACHE_AUTOLOAD_RQST_R = crate::FieldReader;
#[doc = "Field `DCACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
pub type DCACHE_AUTOLOAD_RQST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
pub type DCACHE_AUTOLOAD_SIZE_R = crate::FieldReader;
#[doc = "Field `DCACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
pub type DCACHE_AUTOLOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCACHE_AUTOLOAD_BUFFER_CLEAR` reader - The bit is used to clear autoload buffer in dcache."]
pub type DCACHE_AUTOLOAD_BUFFER_CLEAR_R = crate::BitReader;
#[doc = "Field `DCACHE_AUTOLOAD_BUFFER_CLEAR` writer - The bit is used to clear autoload buffer in dcache."]
pub type DCACHE_AUTOLOAD_BUFFER_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_sct0_ena(&self) -> DCACHE_AUTOLOAD_SCT0_ENA_R {
        DCACHE_AUTOLOAD_SCT0_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_sct1_ena(&self) -> DCACHE_AUTOLOAD_SCT1_ENA_R {
        DCACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn dcache_autoload_ena(&self) -> DCACHE_AUTOLOAD_ENA_R {
        DCACHE_AUTOLOAD_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate autoload operation is finished."]
    #[inline(always)]
    pub fn dcache_autoload_done(&self) -> DCACHE_AUTOLOAD_DONE_R {
        DCACHE_AUTOLOAD_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn dcache_autoload_order(&self) -> DCACHE_AUTOLOAD_ORDER_R {
        DCACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    pub fn dcache_autoload_rqst(&self) -> DCACHE_AUTOLOAD_RQST_R {
        DCACHE_AUTOLOAD_RQST_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
    #[inline(always)]
    pub fn dcache_autoload_size(&self) -> DCACHE_AUTOLOAD_SIZE_R {
        DCACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - The bit is used to clear autoload buffer in dcache."]
    #[inline(always)]
    pub fn dcache_autoload_buffer_clear(&self) -> DCACHE_AUTOLOAD_BUFFER_CLEAR_R {
        DCACHE_AUTOLOAD_BUFFER_CLEAR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_AUTOLOAD_CTRL")
            .field(
                "dcache_autoload_sct0_ena",
                &format_args!("{}", self.dcache_autoload_sct0_ena().bit()),
            )
            .field(
                "dcache_autoload_sct1_ena",
                &format_args!("{}", self.dcache_autoload_sct1_ena().bit()),
            )
            .field(
                "dcache_autoload_ena",
                &format_args!("{}", self.dcache_autoload_ena().bit()),
            )
            .field(
                "dcache_autoload_done",
                &format_args!("{}", self.dcache_autoload_done().bit()),
            )
            .field(
                "dcache_autoload_order",
                &format_args!("{}", self.dcache_autoload_order().bit()),
            )
            .field(
                "dcache_autoload_rqst",
                &format_args!("{}", self.dcache_autoload_rqst().bits()),
            )
            .field(
                "dcache_autoload_size",
                &format_args!("{}", self.dcache_autoload_size().bits()),
            )
            .field(
                "dcache_autoload_buffer_clear",
                &format_args!("{}", self.dcache_autoload_buffer_clear().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_AUTOLOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bits are used to enable the first section for autoload operation."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_sct0_ena(
        &mut self,
    ) -> DCACHE_AUTOLOAD_SCT0_ENA_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_SCT0_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bits are used to enable the second section for autoload operation."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_sct1_ena(
        &mut self,
    ) -> DCACHE_AUTOLOAD_SCT1_ENA_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_SCT1_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to enable and disable autoload operation. It is combined with dcache_autoload_done. 1: enable, 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_ena(&mut self) -> DCACHE_AUTOLOAD_ENA_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_ENA_W::new(self, 2)
    }
    #[doc = "Bit 4 - The bits are used to configure the direction of autoload. 1: descending, 0: ascending."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_order(&mut self) -> DCACHE_AUTOLOAD_ORDER_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_ORDER_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - The bits are used to configure trigger conditions for autoload. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_rqst(&mut self) -> DCACHE_AUTOLOAD_RQST_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_RQST_W::new(self, 5)
    }
    #[doc = "Bits 7:8 - The bits are used to configure the numbers of the cache block for the issuing autoload operation."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_size(&mut self) -> DCACHE_AUTOLOAD_SIZE_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_SIZE_W::new(self, 7)
    }
    #[doc = "Bit 9 - The bit is used to clear autoload buffer in dcache."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_buffer_clear(
        &mut self,
    ) -> DCACHE_AUTOLOAD_BUFFER_CLEAR_W<DCACHE_AUTOLOAD_CTRL_SPEC> {
        DCACHE_AUTOLOAD_BUFFER_CLEAR_W::new(self, 9)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_AUTOLOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_autoload_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_AUTOLOAD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_AUTOLOAD_CTRL to value 0x08"]
impl crate::Resettable for DCACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
