#[doc = "Register `L1_CACHE_AUTOLOAD_CTRL` reader"]
pub struct R(crate::R<L1_CACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_AUTOLOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_AUTOLOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_AUTOLOAD_CTRL` writer"]
pub struct W(crate::W<L1_CACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_AUTOLOAD_CTRL_SPEC>;
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
impl From<crate::W<L1_CACHE_AUTOLOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
pub type L1_CACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
pub type L1_CACHE_AUTOLOAD_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_AUTOLOAD_CTRL_SPEC, O>;
#[doc = "Field `L1_CACHE_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-Cache is finished or not. 0: not finished. 1: finished."]
pub type L1_CACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
pub type L1_CACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_ORDER` writer - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
pub type L1_CACHE_AUTOLOAD_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_AUTOLOAD_CTRL_SPEC, O>;
#[doc = "Field `L1_CACHE_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_CACHE_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_TRIGGER_MODE` writer - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L1_CACHE_AUTOLOAD_TRIGGER_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, L1_CACHE_AUTOLOAD_CTRL_SPEC, 2, O>;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-Cache."]
pub type L1_CACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT0_ENA` writer - The bit is used to enable the first section for autoload operation on L1-Cache."]
pub type L1_CACHE_AUTOLOAD_SCT0_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_AUTOLOAD_CTRL_SPEC, O>;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-Cache."]
pub type L1_CACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT1_ENA` writer - The bit is used to enable the second section for autoload operation on L1-Cache."]
pub type L1_CACHE_AUTOLOAD_SCT1_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_AUTOLOAD_CTRL_SPEC, O>;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT2_ENA` reader - The bit is used to enable the third section for autoload operation on L1-Cache."]
pub type L1_CACHE_AUTOLOAD_SCT2_ENA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT3_ENA` reader - The bit is used to enable the fourth section for autoload operation on L1-Cache."]
pub type L1_CACHE_AUTOLOAD_SCT3_ENA_R = crate::BitReader;
#[doc = "Field `L1_CACHE_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 cache autoload."]
pub type L1_CACHE_AUTOLOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l1_cache_autoload_ena(&self) -> L1_CACHE_AUTOLOAD_ENA_R {
        L1_CACHE_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l1_cache_autoload_done(&self) -> L1_CACHE_AUTOLOAD_DONE_R {
        L1_CACHE_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l1_cache_autoload_order(&self) -> L1_CACHE_AUTOLOAD_ORDER_R {
        L1_CACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l1_cache_autoload_trigger_mode(&self) -> L1_CACHE_AUTOLOAD_TRIGGER_MODE_R {
        L1_CACHE_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_autoload_sct0_ena(&self) -> L1_CACHE_AUTOLOAD_SCT0_ENA_R {
        L1_CACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_autoload_sct1_ena(&self) -> L1_CACHE_AUTOLOAD_SCT1_ENA_R {
        L1_CACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable the third section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_autoload_sct2_ena(&self) -> L1_CACHE_AUTOLOAD_SCT2_ENA_R {
        L1_CACHE_AUTOLOAD_SCT2_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable the fourth section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn l1_cache_autoload_sct3_ena(&self) -> L1_CACHE_AUTOLOAD_SCT3_ENA_R {
        L1_CACHE_AUTOLOAD_SCT3_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - The bit is used to set the gid of l1 cache autoload."]
    #[inline(always)]
    pub fn l1_cache_autoload_rgid(&self) -> L1_CACHE_AUTOLOAD_RGID_R {
        L1_CACHE_AUTOLOAD_RGID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_AUTOLOAD_CTRL")
            .field(
                "l1_cache_autoload_ena",
                &format_args!("{}", self.l1_cache_autoload_ena().bit()),
            )
            .field(
                "l1_cache_autoload_done",
                &format_args!("{}", self.l1_cache_autoload_done().bit()),
            )
            .field(
                "l1_cache_autoload_order",
                &format_args!("{}", self.l1_cache_autoload_order().bit()),
            )
            .field(
                "l1_cache_autoload_trigger_mode",
                &format_args!("{}", self.l1_cache_autoload_trigger_mode().bits()),
            )
            .field(
                "l1_cache_autoload_sct0_ena",
                &format_args!("{}", self.l1_cache_autoload_sct0_ena().bit()),
            )
            .field(
                "l1_cache_autoload_sct1_ena",
                &format_args!("{}", self.l1_cache_autoload_sct1_ena().bit()),
            )
            .field(
                "l1_cache_autoload_sct2_ena",
                &format_args!("{}", self.l1_cache_autoload_sct2_ena().bit()),
            )
            .field(
                "l1_cache_autoload_sct3_ena",
                &format_args!("{}", self.l1_cache_autoload_sct3_ena().bit()),
            )
            .field(
                "l1_cache_autoload_rgid",
                &format_args!("{}", self.l1_cache_autoload_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_AUTOLOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_autoload_ena(&mut self) -> L1_CACHE_AUTOLOAD_ENA_W<0> {
        L1_CACHE_AUTOLOAD_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_autoload_order(&mut self) -> L1_CACHE_AUTOLOAD_ORDER_W<2> {
        L1_CACHE_AUTOLOAD_ORDER_W::new(self)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_autoload_trigger_mode(&mut self) -> L1_CACHE_AUTOLOAD_TRIGGER_MODE_W<3> {
        L1_CACHE_AUTOLOAD_TRIGGER_MODE_W::new(self)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_autoload_sct0_ena(&mut self) -> L1_CACHE_AUTOLOAD_SCT0_ENA_W<8> {
        L1_CACHE_AUTOLOAD_SCT0_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_autoload_sct1_ena(&mut self) -> L1_CACHE_AUTOLOAD_SCT1_ENA_W<9> {
        L1_CACHE_AUTOLOAD_SCT1_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "L1 Cache autoload-operation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_autoload_ctrl](index.html) module"]
pub struct L1_CACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_autoload_ctrl::R](R) reader structure"]
impl crate::Readable for L1_CACHE_AUTOLOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_autoload_ctrl::W](W) writer structure"]
impl crate::Writable for L1_CACHE_AUTOLOAD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for L1_CACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
