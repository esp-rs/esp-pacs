#[doc = "Register `RX_CLKM_DIV_CONF` reader"]
pub struct R(crate::R<RX_CLKM_DIV_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CLKM_DIV_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CLKM_DIV_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CLKM_DIV_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CLKM_DIV_CONF` writer"]
pub struct W(crate::W<RX_CLKM_DIV_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CLKM_DIV_CONF_SPEC>;
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
impl From<crate::W<RX_CLKM_DIV_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CLKM_DIV_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_CLKM_DIV_Z` reader - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
pub type RX_CLKM_DIV_Z_R = crate::FieldReader<u16>;
#[doc = "Field `RX_CLKM_DIV_Z` writer - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
pub type RX_CLKM_DIV_Z_W<'a, const O: u8> =
    crate::FieldWriter<'a, RX_CLKM_DIV_CONF_SPEC, 9, O, u16>;
#[doc = "Field `RX_CLKM_DIV_Y` reader - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
pub type RX_CLKM_DIV_Y_R = crate::FieldReader<u16>;
#[doc = "Field `RX_CLKM_DIV_Y` writer - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
pub type RX_CLKM_DIV_Y_W<'a, const O: u8> =
    crate::FieldWriter<'a, RX_CLKM_DIV_CONF_SPEC, 9, O, u16>;
#[doc = "Field `RX_CLKM_DIV_X` reader - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
pub type RX_CLKM_DIV_X_R = crate::FieldReader<u16>;
#[doc = "Field `RX_CLKM_DIV_X` writer - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
pub type RX_CLKM_DIV_X_W<'a, const O: u8> =
    crate::FieldWriter<'a, RX_CLKM_DIV_CONF_SPEC, 9, O, u16>;
#[doc = "Field `RX_CLKM_DIV_YN1` reader - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
pub type RX_CLKM_DIV_YN1_R = crate::BitReader;
#[doc = "Field `RX_CLKM_DIV_YN1` writer - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
pub type RX_CLKM_DIV_YN1_W<'a, const O: u8> = crate::BitWriter<'a, RX_CLKM_DIV_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:8 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
    #[inline(always)]
    pub fn rx_clkm_div_z(&self) -> RX_CLKM_DIV_Z_R {
        RX_CLKM_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
    #[inline(always)]
    pub fn rx_clkm_div_y(&self) -> RX_CLKM_DIV_Y_R {
        RX_CLKM_DIV_Y_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
    #[inline(always)]
    pub fn rx_clkm_div_x(&self) -> RX_CLKM_DIV_X_R {
        RX_CLKM_DIV_X_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
    #[inline(always)]
    pub fn rx_clkm_div_yn1(&self) -> RX_CLKM_DIV_YN1_R {
        RX_CLKM_DIV_YN1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CLKM_DIV_CONF")
            .field(
                "rx_clkm_div_z",
                &format_args!("{}", self.rx_clkm_div_z().bits()),
            )
            .field(
                "rx_clkm_div_y",
                &format_args!("{}", self.rx_clkm_div_y().bits()),
            )
            .field(
                "rx_clkm_div_x",
                &format_args!("{}", self.rx_clkm_div_x().bits()),
            )
            .field(
                "rx_clkm_div_yn1",
                &format_args!("{}", self.rx_clkm_div_yn1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CLKM_DIV_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Z is b. For b > a/2, the value of I2S_RX_CLKM_DIV_Z is (a-b)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clkm_div_z(&mut self) -> RX_CLKM_DIV_Z_W<0> {
        RX_CLKM_DIV_Z_W::new(self)
    }
    #[doc = "Bits 9:17 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_Y is (a%b) . For b > a/2, the value of I2S_RX_CLKM_DIV_Y is (a%(a-b))."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clkm_div_y(&mut self) -> RX_CLKM_DIV_Y_W<9> {
        RX_CLKM_DIV_Y_W::new(self)
    }
    #[doc = "Bits 18:26 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_X is (a/b) - 1. For b > a/2, the value of I2S_RX_CLKM_DIV_X is (a/(a-b)) - 1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clkm_div_x(&mut self) -> RX_CLKM_DIV_X_W<18> {
        RX_CLKM_DIV_X_W::new(self)
    }
    #[doc = "Bit 27 - For b &lt;= a/2, the value of I2S_RX_CLKM_DIV_YN1 is 0 . For b > a/2, the value of I2S_RX_CLKM_DIV_YN1 is 1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clkm_div_yn1(&mut self) -> RX_CLKM_DIV_YN1_W<27> {
        RX_CLKM_DIV_YN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX module clock divider configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_clkm_div_conf](index.html) module"]
pub struct RX_CLKM_DIV_CONF_SPEC;
impl crate::RegisterSpec for RX_CLKM_DIV_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_clkm_div_conf::R](R) reader structure"]
impl crate::Readable for RX_CLKM_DIV_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_clkm_div_conf::W](W) writer structure"]
impl crate::Writable for RX_CLKM_DIV_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CLKM_DIV_CONF to value 0x0200"]
impl crate::Resettable for RX_CLKM_DIV_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
