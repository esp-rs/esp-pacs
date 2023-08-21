#[doc = "Register `CLK_CONF_POWER_ST` reader"]
pub type R = crate::R<CLK_CONF_POWER_ST_SPEC>;
#[doc = "Register `CLK_CONF_POWER_ST` writer"]
pub type W = crate::W<CLK_CONF_POWER_ST_SPEC>;
#[doc = "Field `CLK_WIFIPWR_ST_MAP` reader - "]
pub type CLK_WIFIPWR_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_WIFIPWR_ST_MAP` writer - "]
pub type CLK_WIFIPWR_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_COEX_ST_MAP` reader - "]
pub type CLK_COEX_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_COEX_ST_MAP` writer - "]
pub type CLK_COEX_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_I2C_MST_ST_MAP` reader - "]
pub type CLK_I2C_MST_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_I2C_MST_ST_MAP` writer - "]
pub type CLK_I2C_MST_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_LP_APB_ST_MAP` reader - "]
pub type CLK_LP_APB_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_LP_APB_ST_MAP` writer - "]
pub type CLK_LP_APB_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
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
    pub fn clk_wifipwr_st_map(&mut self) -> CLK_WIFIPWR_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 16> {
        CLK_WIFIPWR_ST_MAP_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_st_map(&mut self) -> CLK_COEX_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 20> {
        CLK_COEX_ST_MAP_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_st_map(&mut self) -> CLK_I2C_MST_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 24> {
        CLK_I2C_MST_ST_MAP_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_apb_st_map(&mut self) -> CLK_LP_APB_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 28> {
        CLK_LP_APB_ST_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf_power_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf_power_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_POWER_ST_SPEC;
impl crate::RegisterSpec for CLK_CONF_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf_power_st::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_POWER_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf_power_st::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_POWER_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF_POWER_ST to value 0"]
impl crate::Resettable for CLK_CONF_POWER_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
