#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FIFO_CONF_SPEC>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FIFO_CONF_SPEC>;
#[doc = "Field `RX_DATA_NUM` reader - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
pub type RX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `RX_DATA_NUM` writer - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
pub type RX_DATA_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TX_DATA_NUM` reader - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
pub type TX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `TX_DATA_NUM` writer - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
pub type TX_DATA_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DSCR_EN` reader - Set this bit to enable I2S DMA mode."]
pub type DSCR_EN_R = crate::BitReader;
#[doc = "Field `DSCR_EN` writer - Set this bit to enable I2S DMA mode."]
pub type DSCR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_MOD` reader - Transmitter FIFO mode configuration bits"]
pub type TX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `TX_FIFO_MOD` writer - Transmitter FIFO mode configuration bits"]
pub type TX_FIFO_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_FIFO_MOD` reader - Receiver FIFO mode configuration bits"]
pub type RX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `RX_FIFO_MOD` writer - Receiver FIFO mode configuration bits"]
pub type RX_FIFO_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` reader - The bit should always be set to 1"]
pub type TX_FIFO_MOD_FORCE_EN_R = crate::BitReader;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` writer - The bit should always be set to 1"]
pub type TX_FIFO_MOD_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` reader - The bit should always be set to 1"]
pub type RX_FIFO_MOD_FORCE_EN_R = crate::BitReader;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` writer - The bit should always be set to 1"]
pub type RX_FIFO_MOD_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_SYNC` reader - force write back rx data to memory"]
pub type RX_FIFO_SYNC_R = crate::BitReader;
#[doc = "Field `RX_FIFO_SYNC` writer - force write back rx data to memory"]
pub type RX_FIFO_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_24MSB_EN` reader - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type RX_24MSB_EN_R = crate::BitReader;
#[doc = "Field `RX_24MSB_EN` writer - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type RX_24MSB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_24MSB_EN` reader - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type TX_24MSB_EN_R = crate::BitReader;
#[doc = "Field `TX_24MSB_EN` writer - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type TX_24MSB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
    #[inline(always)]
    pub fn rx_data_num(&self) -> RX_DATA_NUM_R {
        RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
    #[inline(always)]
    pub fn tx_data_num(&self) -> TX_DATA_NUM_R {
        TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to enable I2S DMA mode."]
    #[inline(always)]
    pub fn dscr_en(&self) -> DSCR_EN_R {
        DSCR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Transmitter FIFO mode configuration bits"]
    #[inline(always)]
    pub fn tx_fifo_mod(&self) -> TX_FIFO_MOD_R {
        TX_FIFO_MOD_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Receiver FIFO mode configuration bits"]
    #[inline(always)]
    pub fn rx_fifo_mod(&self) -> RX_FIFO_MOD_R {
        RX_FIFO_MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&self) -> TX_FIFO_MOD_FORCE_EN_R {
        TX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&self) -> RX_FIFO_MOD_FORCE_EN_R {
        RX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - force write back rx data to memory"]
    #[inline(always)]
    pub fn rx_fifo_sync(&self) -> RX_FIFO_SYNC_R {
        RX_FIFO_SYNC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn rx_24msb_en(&self) -> RX_24MSB_EN_R {
        RX_24MSB_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn tx_24msb_en(&self) -> TX_24MSB_EN_R {
        TX_24MSB_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field(
                "rx_data_num",
                &format_args!("{}", self.rx_data_num().bits()),
            )
            .field(
                "tx_data_num",
                &format_args!("{}", self.tx_data_num().bits()),
            )
            .field("dscr_en", &format_args!("{}", self.dscr_en().bit()))
            .field(
                "tx_fifo_mod",
                &format_args!("{}", self.tx_fifo_mod().bits()),
            )
            .field(
                "rx_fifo_mod",
                &format_args!("{}", self.rx_fifo_mod().bits()),
            )
            .field(
                "tx_fifo_mod_force_en",
                &format_args!("{}", self.tx_fifo_mod_force_en().bit()),
            )
            .field(
                "rx_fifo_mod_force_en",
                &format_args!("{}", self.rx_fifo_mod_force_en().bit()),
            )
            .field(
                "rx_fifo_sync",
                &format_args!("{}", self.rx_fifo_sync().bit()),
            )
            .field("rx_24msb_en", &format_args!("{}", self.rx_24msb_en().bit()))
            .field("tx_24msb_en", &format_args!("{}", self.tx_24msb_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_num(&mut self) -> RX_DATA_NUM_W<FIFO_CONF_SPEC> {
        RX_DATA_NUM_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_num(&mut self) -> TX_DATA_NUM_W<FIFO_CONF_SPEC> {
        TX_DATA_NUM_W::new(self, 6)
    }
    #[doc = "Bit 12 - Set this bit to enable I2S DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn dscr_en(&mut self) -> DSCR_EN_W<FIFO_CONF_SPEC> {
        DSCR_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Transmitter FIFO mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_mod(&mut self) -> TX_FIFO_MOD_W<FIFO_CONF_SPEC> {
        TX_FIFO_MOD_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - Receiver FIFO mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mod(&mut self) -> RX_FIFO_MOD_W<FIFO_CONF_SPEC> {
        RX_FIFO_MOD_W::new(self, 16)
    }
    #[doc = "Bit 19 - The bit should always be set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_mod_force_en(&mut self) -> TX_FIFO_MOD_FORCE_EN_W<FIFO_CONF_SPEC> {
        TX_FIFO_MOD_FORCE_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - The bit should always be set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mod_force_en(&mut self) -> RX_FIFO_MOD_FORCE_EN_W<FIFO_CONF_SPEC> {
        RX_FIFO_MOD_FORCE_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - force write back rx data to memory"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_sync(&mut self) -> RX_FIFO_SYNC_W<FIFO_CONF_SPEC> {
        RX_FIFO_SYNC_W::new(self, 21)
    }
    #[doc = "Bit 22 - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    #[must_use]
    pub fn rx_24msb_en(&mut self) -> RX_24MSB_EN_W<FIFO_CONF_SPEC> {
        RX_24MSB_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    #[must_use]
    pub fn tx_24msb_en(&mut self) -> TX_24MSB_EN_W<FIFO_CONF_SPEC> {
        TX_24MSB_EN_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S FIFO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_conf::R`](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x1820"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1820;
}
