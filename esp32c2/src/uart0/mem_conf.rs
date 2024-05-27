///Register `MEM_CONF` reader
pub type R = crate::R<MEM_CONF_SPEC>;
///Register `MEM_CONF` writer
pub type W = crate::W<MEM_CONF_SPEC>;
///Field `RX_SIZE` reader - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes.
pub type RX_SIZE_R = crate::FieldReader;
///Field `RX_SIZE` writer - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes.
pub type RX_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TX_SIZE` reader - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes.
pub type TX_SIZE_R = crate::FieldReader;
///Field `TX_SIZE` writer - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes.
pub type TX_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works.
pub type RX_FLOW_THRHD_R = crate::FieldReader<u16>;
///Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works.
pub type RX_FLOW_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `RX_TOUT_THRHD` reader - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
pub type RX_TOUT_THRHD_R = crate::FieldReader<u16>;
///Field `RX_TOUT_THRHD` writer - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
pub type RX_TOUT_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `MEM_FORCE_PD` reader - Set this bit to force power down UART memory.
pub type MEM_FORCE_PD_R = crate::BitReader;
///Field `MEM_FORCE_PD` writer - Set this bit to force power down UART memory.
pub type MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_FORCE_PU` reader - Set this bit to force power up UART memory.
pub type MEM_FORCE_PU_R = crate::BitReader;
///Field `MEM_FORCE_PU` writer - Set this bit to force power up UART memory.
pub type MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes.
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes.
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:15 - This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    ///Bits 16:25 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 26 - Set this bit to force power down UART memory.
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Set this bit to force power up UART memory.
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field("rx_size", &self.rx_size())
            .field("tx_size", &self.tx_size())
            .field("rx_flow_thrhd", &self.rx_flow_thrhd())
            .field("rx_tout_thrhd", &self.rx_tout_thrhd())
            .field("mem_force_pd", &self.mem_force_pd())
            .field("mem_force_pu", &self.mem_force_pu())
            .finish()
    }
}
impl W {
    ///Bits 1:3 - This register is used to configure the amount of mem allocated for receive-FIFO. The default number is 128 bytes.
    #[inline(always)]
    #[must_use]
    pub fn rx_size(&mut self) -> RX_SIZE_W<MEM_CONF_SPEC> {
        RX_SIZE_W::new(self, 1)
    }
    ///Bits 4:6 - This register is used to configure the amount of mem allocated for transmit-FIFO. The default number is 128 bytes.
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TX_SIZE_W<MEM_CONF_SPEC> {
        TX_SIZE_W::new(self, 4)
    }
    ///Bits 7:15 - This register is used to configure the maximum amount of data that can be received when hardware flow control works.
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<MEM_CONF_SPEC> {
        RX_FLOW_THRHD_W::new(self, 7)
    }
    ///Bits 16:25 - This register is used to configure the threshold time that receiver takes to receive one byte. The rxfifo_tout_int interrupt will be trigger when the receiver takes more time to receive one byte with rx_tout_en set to 1.
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<MEM_CONF_SPEC> {
        RX_TOUT_THRHD_W::new(self, 16)
    }
    ///Bit 26 - Set this bit to force power down UART memory.
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<MEM_CONF_SPEC> {
        MEM_FORCE_PD_W::new(self, 26)
    }
    ///Bit 27 - Set this bit to force power up UART memory.
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<MEM_CONF_SPEC> {
        MEM_FORCE_PU_W::new(self, 27)
    }
}
/**UART threshold and allocation configuration

You can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_conf::R`](R) reader structure
impl crate::Readable for MEM_CONF_SPEC {}
///`write(|w| ..)` method takes [`mem_conf::W`](W) writer structure
impl crate::Writable for MEM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_CONF to value 0x000a_0012
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x000a_0012;
}
