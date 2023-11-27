#[doc = "Register `L1_DCACHE_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<L1_DCACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Register `L1_DCACHE_AUTOLOAD_CTRL` writer"]
pub type W = crate::W<L1_DCACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-DCache. 1: enable, 0: disable."]
pub type L1_DCACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation on L1-DCache. 1: enable, 0: disable."]
pub type L1_DCACHE_AUTOLOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-DCache is finished or not. 0: not finished. 1: finished."]
pub type L1_DCACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-DCache. 0: ascending. 1: descending."]
pub type L1_DCACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_ORDER` writer - The bit is used to configure the direction of autoload operation on L1-DCache. 0: ascending. 1: descending."]
pub type L1_DCACHE_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-DCache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_DCACHE_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_TRIGGER_MODE` writer - The field is used to configure trigger mode of autoload operation on L1-DCache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_DCACHE_AUTOLOAD_TRIGGER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT0_ENA` writer - The bit is used to enable the first section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT1_ENA` writer - The bit is used to enable the second section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT2_ENA` reader - The bit is used to enable the third section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT2_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT2_ENA` writer - The bit is used to enable the third section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT2_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT3_ENA` reader - The bit is used to enable the fourth section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT3_ENA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT3_ENA` writer - The bit is used to enable the fourth section for autoload operation on L1-DCache."]
pub type L1_DCACHE_AUTOLOAD_SCT3_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 dcache autoload."]
pub type L1_DCACHE_AUTOLOAD_RGID_R = crate::FieldReader;
#[doc = "Field `L1_DCACHE_AUTOLOAD_RGID` writer - The bit is used to set the gid of l1 dcache autoload."]
pub type L1_DCACHE_AUTOLOAD_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-DCache. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l1_dcache_autoload_ena(&self) -> L1_DCACHE_AUTOLOAD_ENA_R {
        L1_DCACHE_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-DCache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_dcache_autoload_done(&self) -> L1_DCACHE_AUTOLOAD_DONE_R {
        L1_DCACHE_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-DCache. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l1_dcache_autoload_order(&self) -> L1_DCACHE_AUTOLOAD_ORDER_R {
        L1_DCACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-DCache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l1_dcache_autoload_trigger_mode(&self) -> L1_DCACHE_AUTOLOAD_TRIGGER_MODE_R {
        L1_DCACHE_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct0_ena(&self) -> L1_DCACHE_AUTOLOAD_SCT0_ENA_R {
        L1_DCACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct1_ena(&self) -> L1_DCACHE_AUTOLOAD_SCT1_ENA_R {
        L1_DCACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable the third section for autoload operation on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct2_ena(&self) -> L1_DCACHE_AUTOLOAD_SCT2_ENA_R {
        L1_DCACHE_AUTOLOAD_SCT2_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable the fourth section for autoload operation on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct3_ena(&self) -> L1_DCACHE_AUTOLOAD_SCT3_ENA_R {
        L1_DCACHE_AUTOLOAD_SCT3_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - The bit is used to set the gid of l1 dcache autoload."]
    #[inline(always)]
    pub fn l1_dcache_autoload_rgid(&self) -> L1_DCACHE_AUTOLOAD_RGID_R {
        L1_DCACHE_AUTOLOAD_RGID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DCACHE_AUTOLOAD_CTRL")
            .field(
                "l1_dcache_autoload_ena",
                &format_args!("{}", self.l1_dcache_autoload_ena().bit()),
            )
            .field(
                "l1_dcache_autoload_done",
                &format_args!("{}", self.l1_dcache_autoload_done().bit()),
            )
            .field(
                "l1_dcache_autoload_order",
                &format_args!("{}", self.l1_dcache_autoload_order().bit()),
            )
            .field(
                "l1_dcache_autoload_trigger_mode",
                &format_args!("{}", self.l1_dcache_autoload_trigger_mode().bits()),
            )
            .field(
                "l1_dcache_autoload_sct0_ena",
                &format_args!("{}", self.l1_dcache_autoload_sct0_ena().bit()),
            )
            .field(
                "l1_dcache_autoload_sct1_ena",
                &format_args!("{}", self.l1_dcache_autoload_sct1_ena().bit()),
            )
            .field(
                "l1_dcache_autoload_sct2_ena",
                &format_args!("{}", self.l1_dcache_autoload_sct2_ena().bit()),
            )
            .field(
                "l1_dcache_autoload_sct3_ena",
                &format_args!("{}", self.l1_dcache_autoload_sct3_ena().bit()),
            )
            .field(
                "l1_dcache_autoload_rgid",
                &format_args!("{}", self.l1_dcache_autoload_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-DCache. 1: enable, 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_ena(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_ENA_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_ENA_W::new(self, 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-DCache. 0: ascending. 1: descending."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_order(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_ORDER_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_ORDER_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-DCache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_trigger_mode(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_TRIGGER_MODE_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_TRIGGER_MODE_W::new(self, 3)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_sct0_ena(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_SCT0_ENA_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_SCT0_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_sct1_ena(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_SCT1_ENA_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_SCT1_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The bit is used to enable the third section for autoload operation on L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_sct2_ena(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_SCT2_ENA_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_SCT2_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The bit is used to enable the fourth section for autoload operation on L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_sct3_ena(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_SCT3_ENA_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_SCT3_ENA_W::new(self, 11)
    }
    #[doc = "Bits 12:15 - The bit is used to set the gid of l1 dcache autoload."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_rgid(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_RGID_W<L1_DCACHE_AUTOLOAD_CTRL_SPEC> {
        L1_DCACHE_AUTOLOAD_RGID_W::new(self, 12)
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
#[doc = "L1 data Cache autoload-operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_DCACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_DCACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_DCACHE_AUTOLOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_autoload_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_DCACHE_AUTOLOAD_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_DCACHE_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for L1_DCACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
