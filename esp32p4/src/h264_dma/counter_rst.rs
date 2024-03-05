#[doc = "Register `COUNTER_RST` reader"]
pub type R = crate::R<COUNTER_RST_SPEC>;
#[doc = "Register `COUNTER_RST` writer"]
pub type W = crate::W<COUNTER_RST_SPEC>;
#[doc = "Field `RX_CH0_EXTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch0 counter."]
pub type RX_CH0_EXTER_COUNTER_RST_R = crate::BitReader;
#[doc = "Field `RX_CH0_EXTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch0 counter."]
pub type RX_CH0_EXTER_COUNTER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH1_EXTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch1 counter."]
pub type RX_CH1_EXTER_COUNTER_RST_R = crate::BitReader;
#[doc = "Field `RX_CH1_EXTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch1 counter."]
pub type RX_CH1_EXTER_COUNTER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH2_INTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch2 counter."]
pub type RX_CH2_INTER_COUNTER_RST_R = crate::BitReader;
#[doc = "Field `RX_CH2_INTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch2 counter."]
pub type RX_CH2_INTER_COUNTER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH5_INTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch5 counter."]
pub type RX_CH5_INTER_COUNTER_RST_R = crate::BitReader;
#[doc = "Field `RX_CH5_INTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch5 counter."]
pub type RX_CH5_INTER_COUNTER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 then write 0 to this bit to reset rx ch0 counter."]
    #[inline(always)]
    pub fn rx_ch0_exter_counter_rst(&self) -> RX_CH0_EXTER_COUNTER_RST_R {
        RX_CH0_EXTER_COUNTER_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset rx ch1 counter."]
    #[inline(always)]
    pub fn rx_ch1_exter_counter_rst(&self) -> RX_CH1_EXTER_COUNTER_RST_R {
        RX_CH1_EXTER_COUNTER_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 then write 0 to this bit to reset rx ch2 counter."]
    #[inline(always)]
    pub fn rx_ch2_inter_counter_rst(&self) -> RX_CH2_INTER_COUNTER_RST_R {
        RX_CH2_INTER_COUNTER_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 then write 0 to this bit to reset rx ch5 counter."]
    #[inline(always)]
    pub fn rx_ch5_inter_counter_rst(&self) -> RX_CH5_INTER_COUNTER_RST_R {
        RX_CH5_INTER_COUNTER_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNTER_RST")
            .field(
                "rx_ch0_exter_counter_rst",
                &format_args!("{}", self.rx_ch0_exter_counter_rst().bit()),
            )
            .field(
                "rx_ch1_exter_counter_rst",
                &format_args!("{}", self.rx_ch1_exter_counter_rst().bit()),
            )
            .field(
                "rx_ch2_inter_counter_rst",
                &format_args!("{}", self.rx_ch2_inter_counter_rst().bit()),
            )
            .field(
                "rx_ch5_inter_counter_rst",
                &format_args!("{}", self.rx_ch5_inter_counter_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COUNTER_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 then write 0 to this bit to reset rx ch0 counter."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch0_exter_counter_rst(&mut self) -> RX_CH0_EXTER_COUNTER_RST_W<COUNTER_RST_SPEC> {
        RX_CH0_EXTER_COUNTER_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset rx ch1 counter."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch1_exter_counter_rst(&mut self) -> RX_CH1_EXTER_COUNTER_RST_W<COUNTER_RST_SPEC> {
        RX_CH1_EXTER_COUNTER_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 then write 0 to this bit to reset rx ch2 counter."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch2_inter_counter_rst(&mut self) -> RX_CH2_INTER_COUNTER_RST_W<COUNTER_RST_SPEC> {
        RX_CH2_INTER_COUNTER_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 then write 0 to this bit to reset rx ch5 counter."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch5_inter_counter_rst(&mut self) -> RX_CH5_INTER_COUNTER_RST_W<COUNTER_RST_SPEC> {
        RX_CH5_INTER_COUNTER_RST_W::new(self, 3)
    }
}
#[doc = "counter reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter_rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter_rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNTER_RST_SPEC;
impl crate::RegisterSpec for COUNTER_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter_rst::R`](R) reader structure"]
impl crate::Readable for COUNTER_RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`counter_rst::W`](W) writer structure"]
impl crate::Writable for COUNTER_RST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COUNTER_RST to value 0"]
impl crate::Resettable for COUNTER_RST_SPEC {
    const RESET_VALUE: u32 = 0;
}
