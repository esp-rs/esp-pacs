#[doc = "Register `SAR_COCPU_INT_ENA` reader"]
pub struct R(crate::R<SAR_COCPU_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_COCPU_INT_ENA` writer"]
pub struct W(crate::W<SAR_COCPU_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_COCPU_INT_ENA_SPEC>;
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
impl From<crate::W<SAR_COCPU_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_COCPU_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COCPU_TOUCH_DONE_INT_ENA` reader - TOUCH_DONE_INT interrupt enable bit"]
pub type COCPU_TOUCH_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_DONE_INT_ENA` writer - TOUCH_DONE_INT interrupt enable bit"]
pub type COCPU_TOUCH_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_ENA` reader - TOUCH_INACTIVE_INT interrupt enable bit"]
pub type COCPU_TOUCH_INACTIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_INACTIVE_INT_ENA` writer - TOUCH_INACTIVE_INT interrupt enable bit"]
pub type COCPU_TOUCH_INACTIVE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_ENA` reader - TOUCH_ACTIVE_INT interrupt enable bit"]
pub type COCPU_TOUCH_ACTIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_TOUCH_ACTIVE_INT_ENA` writer - TOUCH_ACTIVE_INT interrupt enable bit"]
pub type COCPU_TOUCH_ACTIVE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_SARADC1_INT_ENA` reader - SARADC1_DONE_INT interrupt enable bit"]
pub type COCPU_SARADC1_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_INT_ENA` writer - SARADC1_DONE_INT interrupt enable bit"]
pub type COCPU_SARADC1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_SARADC2_INT_ENA` reader - SARADC2_DONE_INT interrupt enable bit"]
pub type COCPU_SARADC2_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_ENA` writer - SARADC2_DONE_INT interrupt enable bit"]
pub type COCPU_SARADC2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_TSENS_INT_ENA` reader - TSENS_DONE_INT interrupt enable bit"]
pub type COCPU_TSENS_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_TSENS_INT_ENA` writer - TSENS_DONE_INT interrupt enable bit"]
pub type COCPU_TSENS_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_START_INT_ENA` reader - RISCV_START_INT interrupt enable bit"]
pub type COCPU_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_START_INT_ENA` writer - RISCV_START_INT interrupt enable bit"]
pub type COCPU_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_SW_INT_ENA` reader - SW_INT interrupt enable bit"]
pub type COCPU_SW_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SW_INT_ENA` writer - SW_INT interrupt enable bit"]
pub type COCPU_SW_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
#[doc = "Field `COCPU_SWD_INT_ENA` reader - SWD_INT interrupt enable bit"]
pub type COCPU_SWD_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SWD_INT_ENA` writer - SWD_INT interrupt enable bit"]
pub type COCPU_SWD_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, SAR_COCPU_INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_done_int_ena(&self) -> COCPU_TOUCH_DONE_INT_ENA_R {
        COCPU_TOUCH_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_inactive_int_ena(&self) -> COCPU_TOUCH_INACTIVE_INT_ENA_R {
        COCPU_TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_touch_active_int_ena(&self) -> COCPU_TOUCH_ACTIVE_INT_ENA_R {
        COCPU_TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena(&self) -> COCPU_SARADC1_INT_ENA_R {
        COCPU_SARADC1_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena(&self) -> COCPU_SARADC2_INT_ENA_R {
        COCPU_SARADC2_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_tsens_int_ena(&self) -> COCPU_TSENS_INT_ENA_R {
        COCPU_TSENS_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_start_int_ena(&self) -> COCPU_START_INT_ENA_R {
        COCPU_START_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SW_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_sw_int_ena(&self) -> COCPU_SW_INT_ENA_R {
        COCPU_SW_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWD_INT interrupt enable bit"]
    #[inline(always)]
    pub fn cocpu_swd_int_ena(&self) -> COCPU_SWD_INT_ENA_R {
        COCPU_SWD_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_COCPU_INT_ENA")
            .field(
                "cocpu_touch_done_int_ena",
                &format_args!("{}", self.cocpu_touch_done_int_ena().bit()),
            )
            .field(
                "cocpu_touch_inactive_int_ena",
                &format_args!("{}", self.cocpu_touch_inactive_int_ena().bit()),
            )
            .field(
                "cocpu_touch_active_int_ena",
                &format_args!("{}", self.cocpu_touch_active_int_ena().bit()),
            )
            .field(
                "cocpu_saradc1_int_ena",
                &format_args!("{}", self.cocpu_saradc1_int_ena().bit()),
            )
            .field(
                "cocpu_saradc2_int_ena",
                &format_args!("{}", self.cocpu_saradc2_int_ena().bit()),
            )
            .field(
                "cocpu_tsens_int_ena",
                &format_args!("{}", self.cocpu_tsens_int_ena().bit()),
            )
            .field(
                "cocpu_start_int_ena",
                &format_args!("{}", self.cocpu_start_int_ena().bit()),
            )
            .field(
                "cocpu_sw_int_ena",
                &format_args!("{}", self.cocpu_sw_int_ena().bit()),
            )
            .field(
                "cocpu_swd_int_ena",
                &format_args!("{}", self.cocpu_swd_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - TOUCH_DONE_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_done_int_ena(&mut self) -> COCPU_TOUCH_DONE_INT_ENA_W<0> {
        COCPU_TOUCH_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - TOUCH_INACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_inactive_int_ena(&mut self) -> COCPU_TOUCH_INACTIVE_INT_ENA_W<1> {
        COCPU_TOUCH_INACTIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - TOUCH_ACTIVE_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_touch_active_int_ena(&mut self) -> COCPU_TOUCH_ACTIVE_INT_ENA_W<2> {
        COCPU_TOUCH_ACTIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - SARADC1_DONE_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc1_int_ena(&mut self) -> COCPU_SARADC1_INT_ENA_W<3> {
        COCPU_SARADC1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - SARADC2_DONE_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc2_int_ena(&mut self) -> COCPU_SARADC2_INT_ENA_W<4> {
        COCPU_SARADC2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - TSENS_DONE_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_tsens_int_ena(&mut self) -> COCPU_TSENS_INT_ENA_W<5> {
        COCPU_TSENS_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - RISCV_START_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_start_int_ena(&mut self) -> COCPU_START_INT_ENA_W<6> {
        COCPU_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - SW_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_sw_int_ena(&mut self) -> COCPU_SW_INT_ENA_W<7> {
        COCPU_SW_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - SWD_INT interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_swd_int_ena(&mut self) -> COCPU_SWD_INT_ENA_W<8> {
        COCPU_SWD_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bit of ULP-RISCV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_ena](index.html) module"]
pub struct SAR_COCPU_INT_ENA_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_int_ena::R](R) reader structure"]
impl crate::Readable for SAR_COCPU_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_cocpu_int_ena::W](W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_ENA to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
