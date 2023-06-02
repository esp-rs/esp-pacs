#[doc = "Register `SAR_IO_MUX_CONF` reader"]
pub struct R(crate::R<SAR_IO_MUX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_IO_MUX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_IO_MUX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_IO_MUX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_IO_MUX_CONF` writer"]
pub struct W(crate::W<SAR_IO_MUX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_IO_MUX_CONF_SPEC>;
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
impl From<crate::W<SAR_IO_MUX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_IO_MUX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOMUX_RESET` reader - Reset IO MUX by software"]
pub type IOMUX_RESET_R = crate::BitReader;
#[doc = "Field `IOMUX_RESET` writer - Reset IO MUX by software"]
pub type IOMUX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SAR_IO_MUX_CONF_SPEC, O>;
#[doc = "Field `IOMUX_CLK_GATE_EN` reader - IO MUX clock gate enable bit"]
pub type IOMUX_CLK_GATE_EN_R = crate::BitReader;
#[doc = "Field `IOMUX_CLK_GATE_EN` writer - IO MUX clock gate enable bit"]
pub type IOMUX_CLK_GATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_IO_MUX_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 30 - Reset IO MUX by software"]
    #[inline(always)]
    pub fn iomux_reset(&self) -> IOMUX_RESET_R {
        IOMUX_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IO MUX clock gate enable bit"]
    #[inline(always)]
    pub fn iomux_clk_gate_en(&self) -> IOMUX_CLK_GATE_EN_R {
        IOMUX_CLK_GATE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_IO_MUX_CONF")
            .field("iomux_reset", &format_args!("{}", self.iomux_reset().bit()))
            .field(
                "iomux_clk_gate_en",
                &format_args!("{}", self.iomux_clk_gate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_IO_MUX_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - Reset IO MUX by software"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_reset(&mut self) -> IOMUX_RESET_W<30> {
        IOMUX_RESET_W::new(self)
    }
    #[doc = "Bit 31 - IO MUX clock gate enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_clk_gate_en(&mut self) -> IOMUX_CLK_GATE_EN_W<31> {
        IOMUX_CLK_GATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure and reset IO MUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_io_mux_conf](index.html) module"]
pub struct SAR_IO_MUX_CONF_SPEC;
impl crate::RegisterSpec for SAR_IO_MUX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_io_mux_conf::R](R) reader structure"]
impl crate::Readable for SAR_IO_MUX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_io_mux_conf::W](W) writer structure"]
impl crate::Writable for SAR_IO_MUX_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_IO_MUX_CONF to value 0"]
impl crate::Resettable for SAR_IO_MUX_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
