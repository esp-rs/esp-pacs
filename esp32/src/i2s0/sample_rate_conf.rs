#[doc = "Register `SAMPLE_RATE_CONF` reader"]
pub struct R(crate::R<SAMPLE_RATE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_RATE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_RATE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_RATE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_RATE_CONF` writer"]
pub struct W(crate::W<SAMPLE_RATE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_RATE_CONF_SPEC>;
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
impl From<crate::W<SAMPLE_RATE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_RATE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` reader - "]
pub type TX_BCK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `TX_BCK_DIV_NUM` writer - "]
pub type TX_BCK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SAMPLE_RATE_CONF_SPEC, 6, O>;
#[doc = "Field `RX_BCK_DIV_NUM` reader - "]
pub type RX_BCK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `RX_BCK_DIV_NUM` writer - "]
pub type RX_BCK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SAMPLE_RATE_CONF_SPEC, 6, O>;
#[doc = "Field `TX_BITS_MOD` reader - "]
pub type TX_BITS_MOD_R = crate::FieldReader;
#[doc = "Field `TX_BITS_MOD` writer - "]
pub type TX_BITS_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, SAMPLE_RATE_CONF_SPEC, 6, O>;
#[doc = "Field `RX_BITS_MOD` reader - "]
pub type RX_BITS_MOD_R = crate::FieldReader;
#[doc = "Field `RX_BITS_MOD` writer - "]
pub type RX_BITS_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, SAMPLE_RATE_CONF_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> TX_BCK_DIV_NUM_R {
        TX_BCK_DIV_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RX_BCK_DIV_NUM_R {
        RX_BCK_DIV_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> TX_BITS_MOD_R {
        TX_BITS_MOD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAMPLE_RATE_CONF")
            .field(
                "tx_bck_div_num",
                &format_args!("{}", self.tx_bck_div_num().bits()),
            )
            .field(
                "rx_bck_div_num",
                &format_args!("{}", self.rx_bck_div_num().bits()),
            )
            .field(
                "tx_bits_mod",
                &format_args!("{}", self.tx_bits_mod().bits()),
            )
            .field(
                "rx_bits_mod",
                &format_args!("{}", self.rx_bits_mod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAMPLE_RATE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_bck_div_num(&mut self) -> TX_BCK_DIV_NUM_W<0> {
        TX_BCK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bck_div_num(&mut self) -> RX_BCK_DIV_NUM_W<6> {
        RX_BCK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn tx_bits_mod(&mut self) -> TX_BITS_MOD_W<12> {
        TX_BITS_MOD_W::new(self)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W<18> {
        RX_BITS_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_rate_conf](index.html) module"]
pub struct SAMPLE_RATE_CONF_SPEC;
impl crate::RegisterSpec for SAMPLE_RATE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_rate_conf::R](R) reader structure"]
impl crate::Readable for SAMPLE_RATE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_rate_conf::W](W) writer structure"]
impl crate::Writable for SAMPLE_RATE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLE_RATE_CONF to value 0x0041_0186"]
impl crate::Resettable for SAMPLE_RATE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0041_0186;
}
