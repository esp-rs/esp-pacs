#[doc = "Register `SAR_HALL_CTRL` reader"]
pub struct R(crate::R<SAR_HALL_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_HALL_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_HALL_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_HALL_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_HALL_CTRL` writer"]
pub struct W(crate::W<SAR_HALL_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_HALL_CTRL_SPEC>;
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
impl From<crate::W<SAR_HALL_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_HALL_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_HALL` reader - Power on hall sensor and connect to VP and VN"]
pub type XPD_HALL_R = crate::BitReader;
#[doc = "Field `XPD_HALL` writer - Power on hall sensor and connect to VP and VN"]
pub type XPD_HALL_W<'a, const O: u8> = crate::BitWriter<'a, SAR_HALL_CTRL_SPEC, O>;
#[doc = "Field `XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_R = crate::BitReader;
#[doc = "Field `XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_HALL_CTRL_SPEC, O>;
#[doc = "Field `HALL_PHASE` reader - Reverse phase of hall sensor"]
pub type HALL_PHASE_R = crate::BitReader;
#[doc = "Field `HALL_PHASE` writer - Reverse phase of hall sensor"]
pub type HALL_PHASE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_HALL_CTRL_SPEC, O>;
#[doc = "Field `HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_R = crate::BitReader;
#[doc = "Field `HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_HALL_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 28 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&self) -> XPD_HALL_R {
        XPD_HALL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&self) -> HALL_PHASE_R {
        HALL_PHASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_HALL_CTRL")
            .field("xpd_hall", &format_args!("{}", self.xpd_hall().bit()))
            .field(
                "xpd_hall_force",
                &format_args!("{}", self.xpd_hall_force().bit()),
            )
            .field("hall_phase", &format_args!("{}", self.hall_phase().bit()))
            .field(
                "hall_phase_force",
                &format_args!("{}", self.hall_phase_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_HALL_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 28 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_hall(&mut self) -> XPD_HALL_W<28> {
        XPD_HALL_W::new(self)
    }
    #[doc = "Bit 29 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W<29> {
        XPD_HALL_FORCE_W::new(self)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    #[must_use]
    pub fn hall_phase(&mut self) -> HALL_PHASE_W<30> {
        HALL_PHASE_W::new(self)
    }
    #[doc = "Bit 31 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W<31> {
        HALL_PHASE_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hall control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_hall_ctrl](index.html) module"]
pub struct SAR_HALL_CTRL_SPEC;
impl crate::RegisterSpec for SAR_HALL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_hall_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_HALL_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_hall_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_HALL_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_HALL_CTRL to value 0xa000_0000"]
impl crate::Resettable for SAR_HALL_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xa000_0000;
}
