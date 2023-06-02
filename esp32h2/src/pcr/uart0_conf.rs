#[doc = "Register `UART0_CONF` reader"]
pub struct R(crate::R<UART0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART0_CONF` writer"]
pub struct W(crate::W<UART0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_CONF_SPEC>;
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
impl From<crate::W<UART0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0_CLK_EN` reader - Set 1 to enable uart0 apb clock"]
pub type UART0_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART0_CLK_EN` writer - Set 1 to enable uart0 apb clock"]
pub type UART0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, UART0_CONF_SPEC, O>;
#[doc = "Field `UART0_RST_EN` reader - Set 0 to reset uart0 module"]
pub type UART0_RST_EN_R = crate::BitReader;
#[doc = "Field `UART0_RST_EN` writer - Set 0 to reset uart0 module"]
pub type UART0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, UART0_CONF_SPEC, O>;
#[doc = "Field `UART0_READY` reader - Query this field after reset uart0 module"]
pub type UART0_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable uart0 apb clock"]
    #[inline(always)]
    pub fn uart0_clk_en(&self) -> UART0_CLK_EN_R {
        UART0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset uart0 module"]
    #[inline(always)]
    pub fn uart0_rst_en(&self) -> UART0_RST_EN_R {
        UART0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset uart0 module"]
    #[inline(always)]
    pub fn uart0_ready(&self) -> UART0_READY_R {
        UART0_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0_CONF")
            .field(
                "uart0_clk_en",
                &format_args!("{}", self.uart0_clk_en().bit()),
            )
            .field(
                "uart0_rst_en",
                &format_args!("{}", self.uart0_rst_en().bit()),
            )
            .field("uart0_ready", &format_args!("{}", self.uart0_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable uart0 apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_clk_en(&mut self) -> UART0_CLK_EN_W<0> {
        UART0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset uart0 module"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_rst_en(&mut self) -> UART0_RST_EN_W<1> {
        UART0_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_conf](index.html) module"]
pub struct UART0_CONF_SPEC;
impl crate::RegisterSpec for UART0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart0_conf::R](R) reader structure"]
impl crate::Readable for UART0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_conf::W](W) writer structure"]
impl crate::Writable for UART0_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART0_CONF to value 0x05"]
impl crate::Resettable for UART0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
