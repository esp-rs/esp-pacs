#[doc = "Register `SAMPLE_RATE_CONF` reader"]
pub type R = crate::R<SAMPLE_RATE_CONF_SPEC>;
#[doc = "Register `SAMPLE_RATE_CONF` writer"]
pub type W = crate::W<SAMPLE_RATE_CONF_SPEC>;
#[doc = "Field `TX_BCK_DIV_NUM` reader - Bit clock configuration bits in transmitter mode."]
pub type TX_BCK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `TX_BCK_DIV_NUM` writer - Bit clock configuration bits in transmitter mode."]
pub type TX_BCK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_BCK_DIV_NUM` reader - Bit clock configuration bits in receiver mode."]
pub type RX_BCK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `RX_BCK_DIV_NUM` writer - Bit clock configuration bits in receiver mode."]
pub type RX_BCK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TX_BITS_MOD` reader - Set the bits to configure bit length of I2S transmitter channel, the value of which can only be 8, 16, 24 and 32."]
pub type TX_BITS_MOD_R = crate::FieldReader;
#[doc = "Field `TX_BITS_MOD` writer - Set the bits to configure bit length of I2S transmitter channel, the value of which can only be 8, 16, 24 and 32."]
pub type TX_BITS_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RX_BITS_MOD` reader - Set the bits to configure bit length of I2S receiver channel, the value of which can only be 8, 16, 24 and 32."]
pub type RX_BITS_MOD_R = crate::FieldReader;
#[doc = "Field `RX_BITS_MOD` writer - Set the bits to configure bit length of I2S receiver channel, the value of which can only be 8, 16, 24 and 32."]
pub type RX_BITS_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Bit clock configuration bits in transmitter mode."]
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> TX_BCK_DIV_NUM_R {
        TX_BCK_DIV_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Bit clock configuration bits in receiver mode."]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RX_BCK_DIV_NUM_R {
        RX_BCK_DIV_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Set the bits to configure bit length of I2S transmitter channel, the value of which can only be 8, 16, 24 and 32."]
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> TX_BITS_MOD_R {
        TX_BITS_MOD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Set the bits to configure bit length of I2S receiver channel, the value of which can only be 8, 16, 24 and 32."]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAMPLE_RATE_CONF")
            .field("tx_bck_div_num", &self.tx_bck_div_num())
            .field("rx_bck_div_num", &self.rx_bck_div_num())
            .field("tx_bits_mod", &self.tx_bits_mod())
            .field("rx_bits_mod", &self.rx_bits_mod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Bit clock configuration bits in transmitter mode."]
    #[inline(always)]
    pub fn tx_bck_div_num(&mut self) -> TX_BCK_DIV_NUM_W<SAMPLE_RATE_CONF_SPEC> {
        TX_BCK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Bit clock configuration bits in receiver mode."]
    #[inline(always)]
    pub fn rx_bck_div_num(&mut self) -> RX_BCK_DIV_NUM_W<SAMPLE_RATE_CONF_SPEC> {
        RX_BCK_DIV_NUM_W::new(self, 6)
    }
    #[doc = "Bits 12:17 - Set the bits to configure bit length of I2S transmitter channel, the value of which can only be 8, 16, 24 and 32."]
    #[inline(always)]
    pub fn tx_bits_mod(&mut self) -> TX_BITS_MOD_W<SAMPLE_RATE_CONF_SPEC> {
        TX_BITS_MOD_W::new(self, 12)
    }
    #[doc = "Bits 18:23 - Set the bits to configure bit length of I2S receiver channel, the value of which can only be 8, 16, 24 and 32."]
    #[inline(always)]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W<SAMPLE_RATE_CONF_SPEC> {
        RX_BITS_MOD_W::new(self, 18)
    }
}
#[doc = "I2S sample rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_rate_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_rate_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAMPLE_RATE_CONF_SPEC;
impl crate::RegisterSpec for SAMPLE_RATE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_rate_conf::R`](R) reader structure"]
impl crate::Readable for SAMPLE_RATE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sample_rate_conf::W`](W) writer structure"]
impl crate::Writable for SAMPLE_RATE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPLE_RATE_CONF to value 0x0041_0186"]
impl crate::Resettable for SAMPLE_RATE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0041_0186;
}
