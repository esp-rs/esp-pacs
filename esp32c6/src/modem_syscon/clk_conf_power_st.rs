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
#[doc = "Field `CLK_ZB_ST_MAP` reader - "]
pub type CLK_ZB_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_ZB_ST_MAP` writer - "]
pub type CLK_ZB_ST_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_FE_ST_MAP` reader - "]
pub type CLK_FE_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_FE_ST_MAP` writer - "]
pub type CLK_FE_ST_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_BT_ST_MAP` reader - "]
pub type CLK_BT_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_BT_ST_MAP` writer - "]
pub type CLK_BT_ST_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_WIFI_ST_MAP` reader - "]
pub type CLK_WIFI_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_WIFI_ST_MAP` writer - "]
pub type CLK_WIFI_ST_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_MODEM_PERI_ST_MAP` reader - "]
pub type CLK_MODEM_PERI_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_PERI_ST_MAP` writer - "]
pub type CLK_MODEM_PERI_ST_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
#[doc = "Field `CLK_MODEM_APB_ST_MAP` reader - "]
pub type CLK_MODEM_APB_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_APB_ST_MAP` writer - "]
pub type CLK_MODEM_APB_ST_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CLK_CONF_POWER_ST_SPEC, 4, O>;
impl R {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clk_zb_st_map(&self) -> CLK_ZB_ST_MAP_R {
        CLK_ZB_ST_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn clk_fe_st_map(&self) -> CLK_FE_ST_MAP_R {
        CLK_FE_ST_MAP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn clk_bt_st_map(&self) -> CLK_BT_ST_MAP_R {
        CLK_BT_ST_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn clk_wifi_st_map(&self) -> CLK_WIFI_ST_MAP_R {
        CLK_WIFI_ST_MAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn clk_modem_peri_st_map(&self) -> CLK_MODEM_PERI_ST_MAP_R {
        CLK_MODEM_PERI_ST_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn clk_modem_apb_st_map(&self) -> CLK_MODEM_APB_ST_MAP_R {
        CLK_MODEM_APB_ST_MAP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_POWER_ST")
            .field(
                "clk_zb_st_map",
                &format_args!("{}", self.clk_zb_st_map().bits()),
            )
            .field(
                "clk_fe_st_map",
                &format_args!("{}", self.clk_fe_st_map().bits()),
            )
            .field(
                "clk_bt_st_map",
                &format_args!("{}", self.clk_bt_st_map().bits()),
            )
            .field(
                "clk_wifi_st_map",
                &format_args!("{}", self.clk_wifi_st_map().bits()),
            )
            .field(
                "clk_modem_peri_st_map",
                &format_args!("{}", self.clk_modem_peri_st_map().bits()),
            )
            .field(
                "clk_modem_apb_st_map",
                &format_args!("{}", self.clk_modem_apb_st_map().bits()),
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
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_zb_st_map(&mut self) -> CLK_ZB_ST_MAP_W<8> {
        CLK_ZB_ST_MAP_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_st_map(&mut self) -> CLK_FE_ST_MAP_W<12> {
        CLK_FE_ST_MAP_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_st_map(&mut self) -> CLK_BT_ST_MAP_W<16> {
        CLK_BT_ST_MAP_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_st_map(&mut self) -> CLK_WIFI_ST_MAP_W<20> {
        CLK_WIFI_ST_MAP_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_peri_st_map(&mut self) -> CLK_MODEM_PERI_ST_MAP_W<24> {
        CLK_MODEM_PERI_ST_MAP_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_apb_st_map(&mut self) -> CLK_MODEM_APB_ST_MAP_W<28> {
        CLK_MODEM_APB_ST_MAP_W::new(self)
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
