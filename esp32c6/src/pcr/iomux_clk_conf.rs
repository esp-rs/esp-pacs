#[doc = "Register `IOMUX_CLK_CONF` reader"]
pub struct R(crate::R<IOMUX_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOMUX_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOMUX_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOMUX_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOMUX_CLK_CONF` writer"]
pub struct W(crate::W<IOMUX_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOMUX_CLK_CONF_SPEC>;
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
impl From<crate::W<IOMUX_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOMUX_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOMUX_FUNC_CLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
pub type IOMUX_FUNC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `IOMUX_FUNC_CLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
pub type IOMUX_FUNC_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, IOMUX_CLK_CONF_SPEC, 2, O>;
#[doc = "Field `IOMUX_FUNC_CLK_EN` reader - Set 1 to enable iomux function clock"]
pub type IOMUX_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `IOMUX_FUNC_CLK_EN` writer - Set 1 to enable iomux function clock"]
pub type IOMUX_FUNC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, IOMUX_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
    #[inline(always)]
    pub fn iomux_func_clk_sel(&self) -> IOMUX_FUNC_CLK_SEL_R {
        IOMUX_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable iomux function clock"]
    #[inline(always)]
    pub fn iomux_func_clk_en(&self) -> IOMUX_FUNC_CLK_EN_R {
        IOMUX_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUX_CLK_CONF")
            .field(
                "iomux_func_clk_sel",
                &format_args!("{}", self.iomux_func_clk_sel().bits()),
            )
            .field(
                "iomux_func_clk_en",
                &format_args!("{}", self.iomux_func_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IOMUX_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn iomux_func_clk_sel(&mut self) -> IOMUX_FUNC_CLK_SEL_W<20> {
        IOMUX_FUNC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable iomux function clock"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_func_clk_en(&mut self) -> IOMUX_FUNC_CLK_EN_W<22> {
        IOMUX_FUNC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IOMUX_CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomux_clk_conf](index.html) module"]
pub struct IOMUX_CLK_CONF_SPEC;
impl crate::RegisterSpec for IOMUX_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iomux_clk_conf::R](R) reader structure"]
impl crate::Readable for IOMUX_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iomux_clk_conf::W](W) writer structure"]
impl crate::Writable for IOMUX_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOMUX_CLK_CONF to value 0x0070_0000"]
impl crate::Resettable for IOMUX_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0000;
}
