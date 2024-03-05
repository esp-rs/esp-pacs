#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `RX_START_INT_RAW` reader - a"]
pub type RX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_START_INT_RAW` writer - a"]
pub type RX_START_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_INT_RAW` reader - a"]
pub type TX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_START_INT_RAW` writer - a"]
pub type TX_START_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_RAW` reader - a"]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` writer - a"]
pub type RX_HUNG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_RAW` reader - a"]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` writer - a"]
pub type TX_HUNG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` reader - a"]
pub type SEND_S_REG_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q_INT_RAW` writer - a"]
pub type SEND_S_REG_Q_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` reader - a"]
pub type SEND_A_REG_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q_INT_RAW` writer - a"]
pub type SEND_A_REG_Q_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_RAW` reader - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` writer - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
pub type OUT_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0_INT_RAW` reader - Soft control int raw bit."]
pub type APP_CTRL0_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP_CTRL0_INT_RAW` writer - Soft control int raw bit."]
pub type APP_CTRL0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1_INT_RAW` reader - Soft control int raw bit."]
pub type APP_CTRL1_INT_RAW_R = crate::BitReader;
#[doc = "Field `APP_CTRL1_INT_RAW` writer - Soft control int raw bit."]
pub type APP_CTRL1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_start_int_raw(&mut self) -> RX_START_INT_RAW_W<INT_RAW_SPEC> {
        RX_START_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_int_raw(&mut self) -> TX_START_INT_RAW_W<INT_RAW_SPEC> {
        TX_START_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_raw(&mut self) -> RX_HUNG_INT_RAW_W<INT_RAW_SPEC> {
        RX_HUNG_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_raw(&mut self) -> TX_HUNG_INT_RAW_W<INT_RAW_SPEC> {
        TX_HUNG_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q_int_raw(&mut self) -> SEND_S_REG_Q_INT_RAW_W<INT_RAW_SPEC> {
        SEND_S_REG_Q_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q_int_raw(&mut self) -> SEND_A_REG_Q_INT_RAW_W<INT_RAW_SPEC> {
        SEND_A_REG_Q_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the"]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_raw(&mut self) -> OUT_EOF_INT_RAW_W<INT_RAW_SPEC> {
        OUT_EOF_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - Soft control int raw bit."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl0_int_raw(&mut self) -> APP_CTRL0_INT_RAW_W<INT_RAW_SPEC> {
        APP_CTRL0_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - Soft control int raw bit."]
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl1_int_raw(&mut self) -> APP_CTRL1_INT_RAW_W<INT_RAW_SPEC> {
        APP_CTRL1_INT_RAW_W::new(self, 8)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
