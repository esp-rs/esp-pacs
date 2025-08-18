#[doc = "Register `REF_CNT_RST` writer"]
pub type W = crate::W<REF_CNT_RST_SPEC>;
#[doc = "Field `CH(0-7)` writer - This register is used to reset the clock divider of CHANNEL%s."]
pub type CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "This register is used to reset the clock divider of CHANNEL(0-7)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0` field.</div>"]
    #[inline(always)]
    pub fn ch(&mut self, n: u8) -> CH_W<'_, REF_CNT_RST_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_W::new(self, n)
    }
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL1."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL2."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL3."]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - This register is used to reset the clock divider of CHANNEL4."]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - This register is used to reset the clock divider of CHANNEL5."]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 5)
    }
    #[doc = "Bit 6 - This register is used to reset the clock divider of CHANNEL6."]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 6)
    }
    #[doc = "Bit 7 - This register is used to reset the clock divider of CHANNEL7."]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH_W<'_, REF_CNT_RST_SPEC> {
        CH_W::new(self, 7)
    }
}
#[doc = "RMT clock divider reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {}
