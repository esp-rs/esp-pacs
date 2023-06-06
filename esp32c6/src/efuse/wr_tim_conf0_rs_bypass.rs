#[doc = "Register `WR_TIM_CONF0_RS_BYPASS` reader"]
pub struct R(crate::R<WR_TIM_CONF0_RS_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_TIM_CONF0_RS_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_TIM_CONF0_RS_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_TIM_CONF0_RS_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_TIM_CONF0_RS_BYPASS` writer"]
pub struct W(crate::W<WR_TIM_CONF0_RS_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_TIM_CONF0_RS_BYPASS_SPEC>;
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
impl From<crate::W<WR_TIM_CONF0_RS_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_TIM_CONF0_RS_BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS_RS_CORRECTION` reader - Set this bit to bypass reed solomon correction step."]
pub type BYPASS_RS_CORRECTION_R = crate::BitReader;
#[doc = "Field `BYPASS_RS_CORRECTION` writer - Set this bit to bypass reed solomon correction step."]
pub type BYPASS_RS_CORRECTION_W<'a, const O: u8> =
    crate::BitWriter<'a, WR_TIM_CONF0_RS_BYPASS_SPEC, O>;
#[doc = "Field `BYPASS_RS_BLK_NUM` reader - Configures block number of programming twice operation."]
pub type BYPASS_RS_BLK_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `BYPASS_RS_BLK_NUM` writer - Configures block number of programming twice operation."]
pub type BYPASS_RS_BLK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, WR_TIM_CONF0_RS_BYPASS_SPEC, 11, O, u16>;
#[doc = "Field `UPDATE` writer - Set this bit to update multi-bit register signals."]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, WR_TIM_CONF0_RS_BYPASS_SPEC, O>;
#[doc = "Field `TPGM_INACTIVE` reader - Configures the inactive programming time."]
pub type TPGM_INACTIVE_R = crate::FieldReader;
#[doc = "Field `TPGM_INACTIVE` writer - Configures the inactive programming time."]
pub type TPGM_INACTIVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, WR_TIM_CONF0_RS_BYPASS_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to bypass reed solomon correction step."]
    #[inline(always)]
    pub fn bypass_rs_correction(&self) -> BYPASS_RS_CORRECTION_R {
        BYPASS_RS_CORRECTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - Configures block number of programming twice operation."]
    #[inline(always)]
    pub fn bypass_rs_blk_num(&self) -> BYPASS_RS_BLK_NUM_R {
        BYPASS_RS_BLK_NUM_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 13:20 - Configures the inactive programming time."]
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> TPGM_INACTIVE_R {
        TPGM_INACTIVE_R::new(((self.bits >> 13) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF0_RS_BYPASS")
            .field(
                "bypass_rs_correction",
                &format_args!("{}", self.bypass_rs_correction().bit()),
            )
            .field(
                "bypass_rs_blk_num",
                &format_args!("{}", self.bypass_rs_blk_num().bits()),
            )
            .field(
                "tpgm_inactive",
                &format_args!("{}", self.tpgm_inactive().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_TIM_CONF0_RS_BYPASS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to bypass reed solomon correction step."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_rs_correction(&mut self) -> BYPASS_RS_CORRECTION_W<0> {
        BYPASS_RS_CORRECTION_W::new(self)
    }
    #[doc = "Bits 1:11 - Configures block number of programming twice operation."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_rs_blk_num(&mut self) -> BYPASS_RS_BLK_NUM_W<1> {
        BYPASS_RS_BLK_NUM_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to update multi-bit register signals."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<12> {
        UPDATE_W::new(self)
    }
    #[doc = "Bits 13:20 - Configures the inactive programming time."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm_inactive(&mut self) -> TPGM_INACTIVE_W<13> {
        TPGM_INACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurarion register0 of eFuse programming time parameters and rs bypass operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf0_rs_bypass](index.html) module"]
pub struct WR_TIM_CONF0_RS_BYPASS_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF0_RS_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_tim_conf0_rs_bypass::R](R) reader structure"]
impl crate::Readable for WR_TIM_CONF0_RS_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf0_rs_bypass::W](W) writer structure"]
impl crate::Writable for WR_TIM_CONF0_RS_BYPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF0_RS_BYPASS to value 0x2000"]
impl crate::Resettable for WR_TIM_CONF0_RS_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
