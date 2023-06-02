#[doc = "Register `SCL_SP_CONF` reader"]
pub struct R(crate::R<SCL_SP_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_SP_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_SP_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_SP_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_SP_CONF` writer"]
pub struct W(crate::W<SCL_SP_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_SP_CONF_SPEC>;
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
impl From<crate::W<SCL_SP_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_SP_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_RST_SLV_EN` reader - reg_scl_rst_slv_en"]
pub type SCL_RST_SLV_EN_R = crate::BitReader;
#[doc = "Field `SCL_RST_SLV_EN` writer - reg_scl_rst_slv_en"]
pub type SCL_RST_SLV_EN_W<'a, const O: u8> = crate::BitWriter<'a, SCL_SP_CONF_SPEC, O>;
#[doc = "Field `SCL_RST_SLV_NUM` reader - reg_scl_rst_slv_num"]
pub type SCL_RST_SLV_NUM_R = crate::FieldReader;
#[doc = "Field `SCL_RST_SLV_NUM` writer - reg_scl_rst_slv_num"]
pub type SCL_RST_SLV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SCL_SP_CONF_SPEC, 5, O>;
#[doc = "Field `SCL_PD_EN` reader - reg_scl_pd_en"]
pub type SCL_PD_EN_R = crate::BitReader;
#[doc = "Field `SCL_PD_EN` writer - reg_scl_pd_en"]
pub type SCL_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, SCL_SP_CONF_SPEC, O>;
#[doc = "Field `SDA_PD_EN` reader - reg_sda_pd_en"]
pub type SDA_PD_EN_R = crate::BitReader;
#[doc = "Field `SDA_PD_EN` writer - reg_sda_pd_en"]
pub type SDA_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, SCL_SP_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - reg_scl_rst_slv_en"]
    #[inline(always)]
    pub fn scl_rst_slv_en(&self) -> SCL_RST_SLV_EN_R {
        SCL_RST_SLV_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - reg_scl_rst_slv_num"]
    #[inline(always)]
    pub fn scl_rst_slv_num(&self) -> SCL_RST_SLV_NUM_R {
        SCL_RST_SLV_NUM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - reg_scl_pd_en"]
    #[inline(always)]
    pub fn scl_pd_en(&self) -> SCL_PD_EN_R {
        SCL_PD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_sda_pd_en"]
    #[inline(always)]
    pub fn sda_pd_en(&self) -> SDA_PD_EN_R {
        SDA_PD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_SP_CONF")
            .field(
                "scl_rst_slv_en",
                &format_args!("{}", self.scl_rst_slv_en().bit()),
            )
            .field(
                "scl_rst_slv_num",
                &format_args!("{}", self.scl_rst_slv_num().bits()),
            )
            .field("scl_pd_en", &format_args!("{}", self.scl_pd_en().bit()))
            .field("sda_pd_en", &format_args!("{}", self.sda_pd_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_SP_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_scl_rst_slv_en"]
    #[inline(always)]
    #[must_use]
    pub fn scl_rst_slv_en(&mut self) -> SCL_RST_SLV_EN_W<0> {
        SCL_RST_SLV_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - reg_scl_rst_slv_num"]
    #[inline(always)]
    #[must_use]
    pub fn scl_rst_slv_num(&mut self) -> SCL_RST_SLV_NUM_W<1> {
        SCL_RST_SLV_NUM_W::new(self)
    }
    #[doc = "Bit 6 - reg_scl_pd_en"]
    #[inline(always)]
    #[must_use]
    pub fn scl_pd_en(&mut self) -> SCL_PD_EN_W<6> {
        SCL_PD_EN_W::new(self)
    }
    #[doc = "Bit 7 - reg_sda_pd_en"]
    #[inline(always)]
    #[must_use]
    pub fn sda_pd_en(&mut self) -> SDA_PD_EN_W<7> {
        SDA_PD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_SCL_SP_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_sp_conf](index.html) module"]
pub struct SCL_SP_CONF_SPEC;
impl crate::RegisterSpec for SCL_SP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_sp_conf::R](R) reader structure"]
impl crate::Readable for SCL_SP_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_sp_conf::W](W) writer structure"]
impl crate::Writable for SCL_SP_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_SP_CONF to value 0"]
impl crate::Resettable for SCL_SP_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
