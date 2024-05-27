///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `TX_FIFO_REMPTY` reader - Set this bit to enable TX_FIFO_REMPTY_INT.
pub type TX_FIFO_REMPTY_R = crate::BitReader;
///Field `TX_FIFO_REMPTY` writer - Set this bit to enable TX_FIFO_REMPTY_INT.
pub type TX_FIFO_REMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_FIFO_WOVF` reader - Set this bit to enable RX_FIFO_WOVF_INT.
pub type RX_FIFO_WOVF_R = crate::BitReader;
///Field `RX_FIFO_WOVF` writer - Set this bit to enable RX_FIFO_WOVF_INT.
pub type RX_FIFO_WOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_EOF` reader - Set this bit to enable TX_EOF_INT.
pub type TX_EOF_R = crate::BitReader;
///Field `TX_EOF` writer - Set this bit to enable TX_EOF_INT.
pub type TX_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit to enable TX_FIFO_REMPTY_INT.
    #[inline(always)]
    pub fn tx_fifo_rempty(&self) -> TX_FIFO_REMPTY_R {
        TX_FIFO_REMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to enable RX_FIFO_WOVF_INT.
    #[inline(always)]
    pub fn rx_fifo_wovf(&self) -> RX_FIFO_WOVF_R {
        RX_FIFO_WOVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to enable TX_EOF_INT.
    #[inline(always)]
    pub fn tx_eof(&self) -> TX_EOF_R {
        TX_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("tx_fifo_rempty", &self.tx_fifo_rempty())
            .field("rx_fifo_wovf", &self.rx_fifo_wovf())
            .field("tx_eof", &self.tx_eof())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to enable TX_FIFO_REMPTY_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rempty(&mut self) -> TX_FIFO_REMPTY_W<INT_ENA_SPEC> {
        TX_FIFO_REMPTY_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to enable RX_FIFO_WOVF_INT.
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_wovf(&mut self) -> RX_FIFO_WOVF_W<INT_ENA_SPEC> {
        RX_FIFO_WOVF_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to enable TX_EOF_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_eof(&mut self) -> TX_EOF_W<INT_ENA_SPEC> {
        TX_EOF_W::new(self, 2)
    }
}
/**Parallel IO interrupt enable singal configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
