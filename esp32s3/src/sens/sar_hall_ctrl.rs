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
#[doc = "Field `SAR_XPD_HALL` reader - Power on hall sensor and connect to VP and VN"]
pub type SAR_XPD_HALL_R = crate::BitReader<bool>;
#[doc = "Field `SAR_XPD_HALL` writer - Power on hall sensor and connect to VP and VN"]
pub type SAR_XPD_HALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_HALL_CTRL_SPEC, bool, O>;
#[doc = "Field `SAR_XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type SAR_XPD_HALL_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `SAR_XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type SAR_XPD_HALL_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_HALL_CTRL_SPEC, bool, O>;
#[doc = "Field `SAR_HALL_PHASE` reader - Reverse phase of hall sensor"]
pub type SAR_HALL_PHASE_R = crate::BitReader<bool>;
#[doc = "Field `SAR_HALL_PHASE` writer - Reverse phase of hall sensor"]
pub type SAR_HALL_PHASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_HALL_CTRL_SPEC, bool, O>;
#[doc = "Field `SAR_HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type SAR_HALL_PHASE_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `SAR_HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type SAR_HALL_PHASE_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_HALL_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn sar_xpd_hall(&self) -> SAR_XPD_HALL_R {
        SAR_XPD_HALL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sar_xpd_hall_force(&self) -> SAR_XPD_HALL_FORCE_R {
        SAR_XPD_HALL_FORCE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn sar_hall_phase(&self) -> SAR_HALL_PHASE_R {
        SAR_HALL_PHASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sar_hall_phase_force(&self) -> SAR_HALL_PHASE_FORCE_R {
        SAR_HALL_PHASE_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn sar_xpd_hall(&mut self) -> SAR_XPD_HALL_W<28> {
        SAR_XPD_HALL_W::new(self)
    }
    #[doc = "Bit 29 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sar_xpd_hall_force(&mut self) -> SAR_XPD_HALL_FORCE_W<29> {
        SAR_XPD_HALL_FORCE_W::new(self)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn sar_hall_phase(&mut self) -> SAR_HALL_PHASE_W<30> {
        SAR_HALL_PHASE_W::new(self)
    }
    #[doc = "Bit 31 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn sar_hall_phase_force(&mut self) -> SAR_HALL_PHASE_FORCE_W<31> {
        SAR_HALL_PHASE_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_hall_ctrl](index.html) module"]
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
}
#[doc = "`reset()` method sets SAR_HALL_CTRL to value 0xa000_0000"]
impl crate::Resettable for SAR_HALL_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa000_0000
    }
}
