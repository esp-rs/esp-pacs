#[doc = "Register `SAR_TOUCH_CHN_ST` reader"]
pub type R = crate::R<SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Register `SAR_TOUCH_CHN_ST` writer"]
pub type W = crate::W<SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Field `SAR_TOUCH_PAD_ACTIVE` reader - touch active status"]
pub type SAR_TOUCH_PAD_ACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_TOUCH_CHANNEL_CLR` writer - Clear touch channel"]
pub type SAR_TOUCH_CHANNEL_CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `SAR_TOUCH_MEAS_DONE` reader - get touch meas done"]
pub type SAR_TOUCH_MEAS_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - touch active status"]
    #[inline(always)]
    pub fn sar_touch_pad_active(&self) -> SAR_TOUCH_PAD_ACTIVE_R {
        SAR_TOUCH_PAD_ACTIVE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - get touch meas done"]
    #[inline(always)]
    pub fn sar_touch_meas_done(&self) -> SAR_TOUCH_MEAS_DONE_R {
        SAR_TOUCH_MEAS_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CHN_ST")
            .field(
                "sar_touch_pad_active",
                &format_args!("{}", self.sar_touch_pad_active().bits()),
            )
            .field(
                "sar_touch_meas_done",
                &format_args!("{}", self.sar_touch_meas_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_CHN_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 15:29 - Clear touch channel"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_channel_clr(&mut self) -> SAR_TOUCH_CHANNEL_CLR_W<SAR_TOUCH_CHN_ST_SPEC, 15> {
        SAR_TOUCH_CHANNEL_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Get touch channel status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_chn_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_chn_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_CHN_ST_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CHN_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_chn_st::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CHN_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_chn_st::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CHN_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_CHN_ST to value 0"]
impl crate::Resettable for SAR_TOUCH_CHN_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
