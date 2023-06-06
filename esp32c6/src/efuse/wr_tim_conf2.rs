#[doc = "Register `WR_TIM_CONF2` reader"]
pub struct R(crate::R<WR_TIM_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_TIM_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_TIM_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_TIM_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_TIM_CONF2` writer"]
pub struct W(crate::W<WR_TIM_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_TIM_CONF2_SPEC>;
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
impl From<crate::W<WR_TIM_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_TIM_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_OFF_NUM` reader - Configures the power outage time for VDDQ."]
pub type PWR_OFF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PWR_OFF_NUM` writer - Configures the power outage time for VDDQ."]
pub type PWR_OFF_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, WR_TIM_CONF2_SPEC, 16, O, u16>;
#[doc = "Field `TPGM` reader - Configures the active programming time."]
pub type TPGM_R = crate::FieldReader<u16>;
#[doc = "Field `TPGM` writer - Configures the active programming time."]
pub type TPGM_W<'a, const O: u8> = crate::FieldWriter<'a, WR_TIM_CONF2_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the power outage time for VDDQ."]
    #[inline(always)]
    pub fn pwr_off_num(&self) -> PWR_OFF_NUM_R {
        PWR_OFF_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the active programming time."]
    #[inline(always)]
    pub fn tpgm(&self) -> TPGM_R {
        TPGM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF2")
            .field(
                "pwr_off_num",
                &format_args!("{}", self.pwr_off_num().bits()),
            )
            .field("tpgm", &format_args!("{}", self.tpgm().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_TIM_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the power outage time for VDDQ."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_off_num(&mut self) -> PWR_OFF_NUM_W<0> {
        PWR_OFF_NUM_W::new(self)
    }
    #[doc = "Bits 16:31 - Configures the active programming time."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm(&mut self) -> TPGM_W<16> {
        TPGM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurarion register 2 of eFuse programming timing parameters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf2](index.html) module"]
pub struct WR_TIM_CONF2_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_tim_conf2::R](R) reader structure"]
impl crate::Readable for WR_TIM_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf2::W](W) writer structure"]
impl crate::Writable for WR_TIM_CONF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF2 to value 0x00c8_0190"]
impl crate::Resettable for WR_TIM_CONF2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00c8_0190;
}
