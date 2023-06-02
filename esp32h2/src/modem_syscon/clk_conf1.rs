#[doc = "Register `CLK_CONF1` reader"]
pub struct R(crate::R<CLK_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF1` writer"]
pub struct W(crate::W<CLK_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF1_SPEC>;
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
impl From<crate::W<CLK_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_FE_16M_EN` reader - ."]
pub type CLK_FE_16M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_16M_EN` writer - ."]
pub type CLK_FE_16M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_32M_EN` reader - ."]
pub type CLK_FE_32M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_32M_EN` writer - ."]
pub type CLK_FE_32M_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_SDM_EN` reader - ."]
pub type CLK_FE_SDM_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_SDM_EN` writer - ."]
pub type CLK_FE_SDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_ADC_EN` reader - ."]
pub type CLK_FE_ADC_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_ADC_EN` writer - ."]
pub type CLK_FE_ADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_FE_APB_EN` reader - ."]
pub type CLK_FE_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_APB_EN` writer - ."]
pub type CLK_FE_APB_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_BT_APB_EN` reader - ."]
pub type CLK_BT_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_BT_APB_EN` writer - ."]
pub type CLK_BT_APB_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
#[doc = "Field `CLK_BT_EN` reader - ."]
pub type CLK_BT_EN_R = crate::BitReader;
#[doc = "Field `CLK_BT_EN` writer - ."]
pub type CLK_BT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CLK_CONF1_SPEC, O>;
impl R {
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    pub fn clk_fe_16m_en(&self) -> CLK_FE_16M_EN_R {
        CLK_FE_16M_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    pub fn clk_fe_32m_en(&self) -> CLK_FE_32M_EN_R {
        CLK_FE_32M_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    pub fn clk_fe_sdm_en(&self) -> CLK_FE_SDM_EN_R {
        CLK_FE_SDM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ."]
    #[inline(always)]
    pub fn clk_fe_adc_en(&self) -> CLK_FE_ADC_EN_R {
        CLK_FE_ADC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ."]
    #[inline(always)]
    pub fn clk_fe_apb_en(&self) -> CLK_FE_APB_EN_R {
        CLK_FE_APB_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ."]
    #[inline(always)]
    pub fn clk_bt_apb_en(&self) -> CLK_BT_APB_EN_R {
        CLK_BT_APB_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ."]
    #[inline(always)]
    pub fn clk_bt_en(&self) -> CLK_BT_EN_R {
        CLK_BT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF1")
            .field(
                "clk_fe_16m_en",
                &format_args!("{}", self.clk_fe_16m_en().bit()),
            )
            .field(
                "clk_fe_32m_en",
                &format_args!("{}", self.clk_fe_32m_en().bit()),
            )
            .field(
                "clk_fe_sdm_en",
                &format_args!("{}", self.clk_fe_sdm_en().bit()),
            )
            .field(
                "clk_fe_adc_en",
                &format_args!("{}", self.clk_fe_adc_en().bit()),
            )
            .field(
                "clk_fe_apb_en",
                &format_args!("{}", self.clk_fe_apb_en().bit()),
            )
            .field(
                "clk_bt_apb_en",
                &format_args!("{}", self.clk_bt_apb_en().bit()),
            )
            .field("clk_bt_en", &format_args!("{}", self.clk_bt_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 12 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_16m_en(&mut self) -> CLK_FE_16M_EN_W<12> {
        CLK_FE_16M_EN_W::new(self)
    }
    #[doc = "Bit 13 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_32m_en(&mut self) -> CLK_FE_32M_EN_W<13> {
        CLK_FE_32M_EN_W::new(self)
    }
    #[doc = "Bit 14 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_sdm_en(&mut self) -> CLK_FE_SDM_EN_W<14> {
        CLK_FE_SDM_EN_W::new(self)
    }
    #[doc = "Bit 15 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_adc_en(&mut self) -> CLK_FE_ADC_EN_W<15> {
        CLK_FE_ADC_EN_W::new(self)
    }
    #[doc = "Bit 16 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_apb_en(&mut self) -> CLK_FE_APB_EN_W<16> {
        CLK_FE_APB_EN_W::new(self)
    }
    #[doc = "Bit 17 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_apb_en(&mut self) -> CLK_BT_APB_EN_W<17> {
        CLK_BT_APB_EN_W::new(self)
    }
    #[doc = "Bit 18 - ."]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_en(&mut self) -> CLK_BT_EN_W<18> {
        CLK_BT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf1](index.html) module"]
pub struct CLK_CONF1_SPEC;
impl crate::RegisterSpec for CLK_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf1::R](R) reader structure"]
impl crate::Readable for CLK_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf1::W](W) writer structure"]
impl crate::Writable for CLK_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF1 to value 0"]
impl crate::Resettable for CLK_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
