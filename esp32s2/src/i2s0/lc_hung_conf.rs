#[doc = "Register `LC_HUNG_CONF` reader"]
pub type R = crate::R<LC_HUNG_CONF_SPEC>;
#[doc = "Register `LC_HUNG_CONF` writer"]
pub type W = crate::W<LC_HUNG_CONF_SPEC>;
#[doc = "Field `LC_FIFO_TIMEOUT` reader - I2S_TX_HUNG_INT interrupt or I2S_RX_HUNG_INT interrupt will be triggered when FIFO hung counter is equal to this value."]
pub type LC_FIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `LC_FIFO_TIMEOUT` writer - I2S_TX_HUNG_INT interrupt or I2S_RX_HUNG_INT interrupt will be triggered when FIFO hung counter is equal to this value."]
pub type LC_FIFO_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` reader - The bits are used to set the tick counter threshold. The tick counter is clocked by APB_CLK. The tick counter threshold is 88000/2^I2S_LC_FIFO_TIMEOUT_SHIFT. The tick counter is reset when it reaches the threshold."]
pub type LC_FIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `LC_FIFO_TIMEOUT_SHIFT` writer - The bits are used to set the tick counter threshold. The tick counter is clocked by APB_CLK. The tick counter threshold is 88000/2^I2S_LC_FIFO_TIMEOUT_SHIFT. The tick counter is reset when it reaches the threshold."]
pub type LC_FIFO_TIMEOUT_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` reader - The enable bit for FIFO timeout."]
pub type LC_FIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `LC_FIFO_TIMEOUT_ENA` writer - The enable bit for FIFO timeout."]
pub type LC_FIFO_TIMEOUT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - I2S_TX_HUNG_INT interrupt or I2S_RX_HUNG_INT interrupt will be triggered when FIFO hung counter is equal to this value."]
    #[inline(always)]
    pub fn lc_fifo_timeout(&self) -> LC_FIFO_TIMEOUT_R {
        LC_FIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - The bits are used to set the tick counter threshold. The tick counter is clocked by APB_CLK. The tick counter threshold is 88000/2^I2S_LC_FIFO_TIMEOUT_SHIFT. The tick counter is reset when it reaches the threshold."]
    #[inline(always)]
    pub fn lc_fifo_timeout_shift(&self) -> LC_FIFO_TIMEOUT_SHIFT_R {
        LC_FIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout."]
    #[inline(always)]
    pub fn lc_fifo_timeout_ena(&self) -> LC_FIFO_TIMEOUT_ENA_R {
        LC_FIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_HUNG_CONF")
            .field("lc_fifo_timeout", &self.lc_fifo_timeout())
            .field("lc_fifo_timeout_shift", &self.lc_fifo_timeout_shift())
            .field("lc_fifo_timeout_ena", &self.lc_fifo_timeout_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - I2S_TX_HUNG_INT interrupt or I2S_RX_HUNG_INT interrupt will be triggered when FIFO hung counter is equal to this value."]
    #[inline(always)]
    #[must_use]
    pub fn lc_fifo_timeout(&mut self) -> LC_FIFO_TIMEOUT_W<LC_HUNG_CONF_SPEC> {
        LC_FIFO_TIMEOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - The bits are used to set the tick counter threshold. The tick counter is clocked by APB_CLK. The tick counter threshold is 88000/2^I2S_LC_FIFO_TIMEOUT_SHIFT. The tick counter is reset when it reaches the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn lc_fifo_timeout_shift(&mut self) -> LC_FIFO_TIMEOUT_SHIFT_W<LC_HUNG_CONF_SPEC> {
        LC_FIFO_TIMEOUT_SHIFT_W::new(self, 8)
    }
    #[doc = "Bit 11 - The enable bit for FIFO timeout."]
    #[inline(always)]
    #[must_use]
    pub fn lc_fifo_timeout_ena(&mut self) -> LC_FIFO_TIMEOUT_ENA_W<LC_HUNG_CONF_SPEC> {
        LC_FIFO_TIMEOUT_ENA_W::new(self, 11)
    }
}
#[doc = "I2S Hung configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_hung_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_hung_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_HUNG_CONF_SPEC;
impl crate::RegisterSpec for LC_HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_hung_conf::R`](R) reader structure"]
impl crate::Readable for LC_HUNG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lc_hung_conf::W`](W) writer structure"]
impl crate::Writable for LC_HUNG_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LC_HUNG_CONF to value 0x0810"]
impl crate::Resettable for LC_HUNG_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0810;
}
