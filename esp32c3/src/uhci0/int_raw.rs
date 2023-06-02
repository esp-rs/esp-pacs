#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_START_INT_RAW` reader - a"]
pub type RX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_START_INT_RAW` writer - a"]
pub type RX_START_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TX_START_INT_RAW` reader - a"]
pub type TX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_START_INT_RAW` writer - a"]
pub type TX_START_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `RX_HUNG_INT_RAW` reader - a"]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` writer - a"]
pub type RX_HUNG_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TX_HUNG_INT_RAW` reader - a"]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` writer - a"]
pub type TX_HUNG_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` reader - a"]
pub type SEND_S_REG_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` writer - a"]
pub type SEND_S_REG_Q_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` reader - a"]
pub type SEND_A_REG_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` writer - a"]
pub type SEND_A_REG_Q_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `OUT_EOF_INT_RAW` reader - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` writer - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
pub type OUT_EOF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APP_CTRL0_INT_RAW` reader - Soft control int raw bit."]
pub type APP_CTRL0_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP_CTRL0_INT_RAW` writer - Soft control int raw bit."]
pub type APP_CTRL0_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `APP_CTRL1_INT_RAW` reader - Soft control int raw bit."]
pub type APP_CTRL1_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP_CTRL1_INT_RAW` writer - Soft control int raw bit."]
pub type APP_CTRL1_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn rx_start_int_raw(&self) -> RX_START_INT_RAW_R {
        RX_START_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn tx_start_int_raw(&self) -> TX_START_INT_RAW_R {
        TX_START_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn send_s_reg_q_int_raw(&self) -> SEND_S_REG_Q_INT_RAW_R {
        SEND_S_REG_Q_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn send_a_reg_q_int_raw(&self) -> SEND_A_REG_Q_INT_RAW_R {
        SEND_A_REG_Q_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Soft control int raw bit."]
    #[inline(always)]
    pub fn app_ctrl0_int_raw(&self) -> APP_CTRL0_INT_RAW_R {
        APP_CTRL0_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Soft control int raw bit."]
    #[inline(always)]
    pub fn app_ctrl1_int_raw(&self) -> APP_CTRL1_INT_RAW_R {
        APP_CTRL1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rx_start_int_raw",
                &format_args!("{}", self.rx_start_int_raw().bit()),
            )
            .field(
                "tx_start_int_raw",
                &format_args!("{}", self.tx_start_int_raw().bit()),
            )
            .field(
                "rx_hung_int_raw",
                &format_args!("{}", self.rx_hung_int_raw().bit()),
            )
            .field(
                "tx_hung_int_raw",
                &format_args!("{}", self.tx_hung_int_raw().bit()),
            )
            .field(
                "send_s_reg_q_int_raw",
                &format_args!("{}", self.send_s_reg_q_int_raw().bit()),
            )
            .field(
                "send_a_reg_q_int_raw",
                &format_args!("{}", self.send_a_reg_q_int_raw().bit()),
            )
            .field(
                "out_eof_int_raw",
                &format_args!("{}", self.out_eof_int_raw().bit()),
            )
            .field(
                "app_ctrl0_int_raw",
                &format_args!("{}", self.app_ctrl0_int_raw().bit()),
            )
            .field(
                "app_ctrl1_int_raw",
                &format_args!("{}", self.app_ctrl1_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_int_raw(&mut self) -> RX_START_INT_RAW_W<0> {
        RX_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_int_raw(&mut self) -> TX_START_INT_RAW_W<1> {
        TX_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_raw(&mut self) -> RX_HUNG_INT_RAW_W<2> {
        RX_HUNG_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_raw(&mut self) -> TX_HUNG_INT_RAW_W<3> {
        TX_HUNG_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q_int_raw(&mut self) -> SEND_S_REG_Q_INT_RAW_W<4> {
        SEND_S_REG_Q_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q_int_raw(&mut self) -> SEND_A_REG_Q_INT_RAW_W<5> {
        SEND_A_REG_Q_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_raw(&mut self) -> OUT_EOF_INT_RAW_W<6> {
        OUT_EOF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 7 - Soft control int raw bit."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl0_int_raw(&mut self) -> APP_CTRL0_INT_RAW_W<7> {
        APP_CTRL0_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - Soft control int raw bit."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl1_int_raw(&mut self) -> APP_CTRL1_INT_RAW_W<8> {
        APP_CTRL1_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
