#[doc = "Register `SDIO_SLAVE_ECO_CONF` reader"]
pub struct R(crate::R<SDIO_SLAVE_ECO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_SLAVE_ECO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_SLAVE_ECO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_SLAVE_ECO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_SLAVE_ECO_CONF` writer"]
pub struct W(crate::W<SDIO_SLAVE_ECO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_SLAVE_ECO_CONF_SPEC>;
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
impl From<crate::W<SDIO_SLAVE_ECO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_SLAVE_ECO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_SLAVE_RDN_RESULT` reader - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_RDN_RESULT_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_SLAVE_RDN_ENA` reader - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_RDN_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_SLAVE_RDN_ENA` writer - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_RDN_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIO_SLAVE_ECO_CONF_SPEC, bool, O>;
#[doc = "Field `SDIO_SLAVE_SDIO_CLK_RDN_RESULT` reader - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_SDIO_CLK_RDN_RESULT_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_SLAVE_SDIO_CLK_RDN_ENA` reader - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_SDIO_CLK_RDN_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_SLAVE_SDIO_CLK_RDN_ENA` writer - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_SDIO_CLK_RDN_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIO_SLAVE_ECO_CONF_SPEC, bool, O>;
#[doc = "Field `SDIO_SLAVE_SDCLK_PAD_RDN_RESULT` reader - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_SDCLK_PAD_RDN_RESULT_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_SLAVE_SDCLK_PAD_RDN_ENA` reader - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_SDCLK_PAD_RDN_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_SLAVE_SDCLK_PAD_RDN_ENA` writer - redundant registers for sdio_slave"]
pub type SDIO_SLAVE_SDCLK_PAD_RDN_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIO_SLAVE_ECO_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn sdio_slave_rdn_result(&self) -> SDIO_SLAVE_RDN_RESULT_R {
        SDIO_SLAVE_RDN_RESULT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn sdio_slave_rdn_ena(&self) -> SDIO_SLAVE_RDN_ENA_R {
        SDIO_SLAVE_RDN_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn sdio_slave_sdio_clk_rdn_result(&self) -> SDIO_SLAVE_SDIO_CLK_RDN_RESULT_R {
        SDIO_SLAVE_SDIO_CLK_RDN_RESULT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn sdio_slave_sdio_clk_rdn_ena(&self) -> SDIO_SLAVE_SDIO_CLK_RDN_ENA_R {
        SDIO_SLAVE_SDIO_CLK_RDN_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn sdio_slave_sdclk_pad_rdn_result(&self) -> SDIO_SLAVE_SDCLK_PAD_RDN_RESULT_R {
        SDIO_SLAVE_SDCLK_PAD_RDN_RESULT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - redundant registers for sdio_slave"]
    #[inline(always)]
    pub fn sdio_slave_sdclk_pad_rdn_ena(&self) -> SDIO_SLAVE_SDCLK_PAD_RDN_ENA_R {
        SDIO_SLAVE_SDCLK_PAD_RDN_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - redundant registers for sdio_slave"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_slave_rdn_ena(&mut self) -> SDIO_SLAVE_RDN_ENA_W<1> {
        SDIO_SLAVE_RDN_ENA_W::new(self)
    }
    #[doc = "Bit 3 - redundant registers for sdio_slave"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_slave_sdio_clk_rdn_ena(&mut self) -> SDIO_SLAVE_SDIO_CLK_RDN_ENA_W<3> {
        SDIO_SLAVE_SDIO_CLK_RDN_ENA_W::new(self)
    }
    #[doc = "Bit 5 - redundant registers for sdio_slave"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_slave_sdclk_pad_rdn_ena(&mut self) -> SDIO_SLAVE_SDCLK_PAD_RDN_ENA_W<5> {
        SDIO_SLAVE_SDCLK_PAD_RDN_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdio_slave redundant control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_slave_eco_conf](index.html) module"]
pub struct SDIO_SLAVE_ECO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_SLAVE_ECO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_slave_eco_conf::R](R) reader structure"]
impl crate::Readable for SDIO_SLAVE_ECO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_slave_eco_conf::W](W) writer structure"]
impl crate::Writable for SDIO_SLAVE_ECO_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_SLAVE_ECO_CONF to value 0"]
impl crate::Resettable for SDIO_SLAVE_ECO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
