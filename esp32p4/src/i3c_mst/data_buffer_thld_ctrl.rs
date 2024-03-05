#[doc = "Register `DATA_BUFFER_THLD_CTRL` reader"]
pub type R = crate::R<DATA_BUFFER_THLD_CTRL_SPEC>;
#[doc = "Register `DATA_BUFFER_THLD_CTRL` writer"]
pub type W = crate::W<DATA_BUFFER_THLD_CTRL_SPEC>;
#[doc = "Field `REG_TX_DATA_BUF_THLD` reader - Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31"]
pub type REG_TX_DATA_BUF_THLD_R = crate::FieldReader;
#[doc = "Field `REG_TX_DATA_BUF_THLD` writer - Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31"]
pub type REG_TX_DATA_BUF_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_RX_DATA_BUF_THLD` reader - Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31"]
pub type REG_RX_DATA_BUF_THLD_R = crate::FieldReader;
#[doc = "Field `REG_RX_DATA_BUF_THLD` writer - Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31"]
pub type REG_RX_DATA_BUF_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31"]
    #[inline(always)]
    pub fn reg_tx_data_buf_thld(&self) -> REG_TX_DATA_BUF_THLD_R {
        REG_TX_DATA_BUF_THLD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31"]
    #[inline(always)]
    pub fn reg_rx_data_buf_thld(&self) -> REG_RX_DATA_BUF_THLD_R {
        REG_RX_DATA_BUF_THLD_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_BUFFER_THLD_CTRL")
            .field(
                "reg_tx_data_buf_thld",
                &format_args!("{}", self.reg_tx_data_buf_thld().bits()),
            )
            .field(
                "reg_rx_data_buf_thld",
                &format_args!("{}", self.reg_rx_data_buf_thld().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_BUFFER_THLD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit Buffer Threshold Value. This field controls the number of empty locations in the Transmit FIFO that trigger the TX_THLD_STAT interrupt. Supports values: 000:2 001:4 010:8 011:16 100:31, else:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tx_data_buf_thld(&mut self) -> REG_TX_DATA_BUF_THLD_W<DATA_BUFFER_THLD_CTRL_SPEC> {
        REG_TX_DATA_BUF_THLD_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Receive Buffer Threshold Value. This field controls the number of empty locations in the Receive FIFO that trigger the RX_THLD_STAT interrupt. Supports: 000:2 001:4 010:8 011:16 100:31, else:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_data_buf_thld(&mut self) -> REG_RX_DATA_BUF_THLD_W<DATA_BUFFER_THLD_CTRL_SPEC> {
        REG_RX_DATA_BUF_THLD_W::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer_thld_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_buffer_thld_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_BUFFER_THLD_CTRL_SPEC;
impl crate::RegisterSpec for DATA_BUFFER_THLD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buffer_thld_ctrl::R`](R) reader structure"]
impl crate::Readable for DATA_BUFFER_THLD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_buffer_thld_ctrl::W`](W) writer structure"]
impl crate::Writable for DATA_BUFFER_THLD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_BUFFER_THLD_CTRL to value 0x09"]
impl crate::Resettable for DATA_BUFFER_THLD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x09;
}
