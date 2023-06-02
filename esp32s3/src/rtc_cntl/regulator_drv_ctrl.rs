#[doc = "Register `REGULATOR_DRV_CTRL` reader"]
pub struct R(crate::R<REGULATOR_DRV_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGULATOR_DRV_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGULATOR_DRV_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGULATOR_DRV_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGULATOR_DRV_CTRL` writer"]
pub struct W(crate::W<REGULATOR_DRV_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGULATOR_DRV_CTRL_SPEC>;
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
impl From<crate::W<REGULATOR_DRV_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGULATOR_DRV_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGULATOR_DRV_B_MONITOR` reader - No public"]
pub type REGULATOR_DRV_B_MONITOR_R = crate::FieldReader;
#[doc = "Field `REGULATOR_DRV_B_MONITOR` writer - No public"]
pub type REGULATOR_DRV_B_MONITOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGULATOR_DRV_CTRL_SPEC, 6, O>;
#[doc = "Field `REGULATOR_DRV_B_SLP` reader - No public"]
pub type REGULATOR_DRV_B_SLP_R = crate::FieldReader;
#[doc = "Field `REGULATOR_DRV_B_SLP` writer - No public"]
pub type REGULATOR_DRV_B_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGULATOR_DRV_CTRL_SPEC, 6, O>;
#[doc = "Field `DG_VDD_DRV_B_SLP` reader - No public"]
pub type DG_VDD_DRV_B_SLP_R = crate::FieldReader;
#[doc = "Field `DG_VDD_DRV_B_SLP` writer - No public"]
pub type DG_VDD_DRV_B_SLP_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGULATOR_DRV_CTRL_SPEC, 8, O>;
#[doc = "Field `DG_VDD_DRV_B_MONITOR` reader - No public"]
pub type DG_VDD_DRV_B_MONITOR_R = crate::FieldReader;
#[doc = "Field `DG_VDD_DRV_B_MONITOR` writer - No public"]
pub type DG_VDD_DRV_B_MONITOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGULATOR_DRV_CTRL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:5 - No public"]
    #[inline(always)]
    pub fn regulator_drv_b_monitor(&self) -> REGULATOR_DRV_B_MONITOR_R {
        REGULATOR_DRV_B_MONITOR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - No public"]
    #[inline(always)]
    pub fn regulator_drv_b_slp(&self) -> REGULATOR_DRV_B_SLP_R {
        REGULATOR_DRV_B_SLP_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&self) -> DG_VDD_DRV_B_SLP_R {
        DG_VDD_DRV_B_SLP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - No public"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_monitor(&self) -> DG_VDD_DRV_B_MONITOR_R {
        DG_VDD_DRV_B_MONITOR_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGULATOR_DRV_CTRL")
            .field(
                "regulator_drv_b_monitor",
                &format_args!("{}", self.regulator_drv_b_monitor().bits()),
            )
            .field(
                "regulator_drv_b_slp",
                &format_args!("{}", self.regulator_drv_b_slp().bits()),
            )
            .field(
                "dg_vdd_drv_b_slp",
                &format_args!("{}", self.dg_vdd_drv_b_slp().bits()),
            )
            .field(
                "dg_vdd_drv_b_monitor",
                &format_args!("{}", self.dg_vdd_drv_b_monitor().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGULATOR_DRV_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_drv_b_monitor(&mut self) -> REGULATOR_DRV_B_MONITOR_W<0> {
        REGULATOR_DRV_B_MONITOR_W::new(self)
    }
    #[doc = "Bits 6:11 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_drv_b_slp(&mut self) -> REGULATOR_DRV_B_SLP_W<6> {
        REGULATOR_DRV_B_SLP_W::new(self)
    }
    #[doc = "Bits 12:19 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W<12> {
        DG_VDD_DRV_B_SLP_W::new(self)
    }
    #[doc = "Bits 20:27 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn dg_vdd_drv_b_monitor(&mut self) -> DG_VDD_DRV_B_MONITOR_W<20> {
        DG_VDD_DRV_B_MONITOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regulator_drv_ctrl](index.html) module"]
pub struct REGULATOR_DRV_CTRL_SPEC;
impl crate::RegisterSpec for REGULATOR_DRV_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regulator_drv_ctrl::R](R) reader structure"]
impl crate::Readable for REGULATOR_DRV_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regulator_drv_ctrl::W](W) writer structure"]
impl crate::Writable for REGULATOR_DRV_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGULATOR_DRV_CTRL to value 0"]
impl crate::Resettable for REGULATOR_DRV_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
