#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_DONE_INT_ENA` reader - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_DONE_INT_ENA` writer - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE_INT_ENA` reader - The interrupt enable bit for the i2s_tx_done_int interrupt"]
pub type TX_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_DONE_INT_ENA` writer - The interrupt enable bit for the i2s_tx_done_int interrupt"]
pub type TX_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ENA` writer - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_ENA` reader - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
pub type TX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_ENA` writer - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
pub type TX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done_int_ena(&self) -> RX_DONE_INT_ENA_R {
        RX_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    pub fn tx_done_int_ena(&self) -> TX_DONE_INT_ENA_R {
        TX_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "rx_done_int_ena",
                &format_args!("{}", self.rx_done_int_ena().bit()),
            )
            .field(
                "tx_done_int_ena",
                &format_args!("{}", self.tx_done_int_ena().bit()),
            )
            .field(
                "rx_hung_int_ena",
                &format_args!("{}", self.rx_hung_int_ena().bit()),
            )
            .field(
                "tx_hung_int_ena",
                &format_args!("{}", self.tx_hung_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done_int_ena(&mut self) -> RX_DONE_INT_ENA_W<INT_ENA_SPEC> {
        RX_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_tx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_done_int_ena(&mut self) -> TX_DONE_INT_ENA_W<INT_ENA_SPEC> {
        TX_DONE_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        RX_HUNG_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the i2s_tx_hung_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        TX_HUNG_INT_ENA_W::new(self, 3)
    }
}
#[doc = "I2S interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
