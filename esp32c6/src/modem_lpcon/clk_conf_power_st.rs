#[doc = "Register `CLK_CONF_POWER_ST` reader"]
pub struct R(crate::R<CLK_CONF_POWER_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_POWER_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_POWER_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_POWER_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF_POWER_ST` writer"]
pub struct W(crate::W<CLK_CONF_POWER_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_POWER_ST_SPEC>;
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
impl From<crate::W<CLK_CONF_POWER_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_POWER_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_WIFIPWR_ST_MAP` reader - "]
pub type CLK_WIFIPWR_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_WIFIPWR_ST_MAP` writer - "]
pub type CLK_WIFIPWR_ST_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_COEX_ST_MAP` reader - "]
pub type CLK_COEX_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_COEX_ST_MAP` writer - "]
pub type CLK_COEX_ST_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_I2C_MST_ST_MAP` reader - "]
pub type CLK_I2C_MST_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_I2C_MST_ST_MAP` writer - "]
pub type CLK_I2C_MST_ST_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_LP_APB_ST_MAP` reader - "]
pub type CLK_LP_APB_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_LP_APB_ST_MAP` writer - "]
pub type CLK_LP_APB_ST_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
impl R {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn clk_wifipwr_st_map(&self) -> CLK_WIFIPWR_ST_MAP_R {
        CLK_WIFIPWR_ST_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn clk_coex_st_map(&self) -> CLK_COEX_ST_MAP_R {
        CLK_COEX_ST_MAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn clk_i2c_mst_st_map(&self) -> CLK_I2C_MST_ST_MAP_R {
        CLK_I2C_MST_ST_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn clk_lp_apb_st_map(&self) -> CLK_LP_APB_ST_MAP_R {
        CLK_LP_APB_ST_MAP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_POWER_ST")
            .field(
                "clk_wifipwr_st_map",
                &format_args!("{}", self.clk_wifipwr_st_map().bits()),
            )
            .field(
                "clk_coex_st_map",
                &format_args!("{}", self.clk_coex_st_map().bits()),
            )
            .field(
                "clk_i2c_mst_st_map",
                &format_args!("{}", self.clk_i2c_mst_st_map().bits()),
            )
            .field(
                "clk_lp_apb_st_map",
                &format_args!("{}", self.clk_lp_apb_st_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_POWER_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifipwr_st_map(&mut self) -> CLK_WIFIPWR_ST_MAP_W<16> {
        CLK_WIFIPWR_ST_MAP_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_st_map(&mut self) -> CLK_COEX_ST_MAP_W<20> {
        CLK_COEX_ST_MAP_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_st_map(&mut self) -> CLK_I2C_MST_ST_MAP_W<24> {
        CLK_I2C_MST_ST_MAP_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_apb_st_map(&mut self) -> CLK_LP_APB_ST_MAP_W<28> {
        CLK_LP_APB_ST_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf_power_st](index.html) module"]
pub struct CLK_CONF_POWER_ST_SPEC;
impl crate::RegisterSpec for CLK_CONF_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf_power_st::R](R) reader structure"]
impl crate::Readable for CLK_CONF_POWER_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf_power_st::W](W) writer structure"]
impl crate::Writable for CLK_CONF_POWER_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF_POWER_ST to value 0"]
impl crate::Resettable for CLK_CONF_POWER_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
