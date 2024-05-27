///Register `CH_ENA_AD0_CLR` writer
pub type W = crate::W<CH_ENA_AD0_CLR_SPEC>;
///Field `CH_CLR(0-31)` writer - ch%s clear
pub type CH_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///ch(0-31) clear
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH_CLR0` field
    #[inline(always)]
    #[must_use]
    pub fn ch_clr(&mut self, n: u8) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_CLR_W::new(self, n)
    }
    ///Bit 0 - ch0 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr0(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 0)
    }
    ///Bit 1 - ch1 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr1(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 1)
    }
    ///Bit 2 - ch2 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr2(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 2)
    }
    ///Bit 3 - ch3 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr3(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 3)
    }
    ///Bit 4 - ch4 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr4(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 4)
    }
    ///Bit 5 - ch5 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr5(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 5)
    }
    ///Bit 6 - ch6 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr6(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 6)
    }
    ///Bit 7 - ch7 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr7(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 7)
    }
    ///Bit 8 - ch8 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr8(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 8)
    }
    ///Bit 9 - ch9 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr9(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 9)
    }
    ///Bit 10 - ch10 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr10(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 10)
    }
    ///Bit 11 - ch11 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr11(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 11)
    }
    ///Bit 12 - ch12 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr12(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 12)
    }
    ///Bit 13 - ch13 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr13(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 13)
    }
    ///Bit 14 - ch14 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr14(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 14)
    }
    ///Bit 15 - ch15 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr15(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 15)
    }
    ///Bit 16 - ch16 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr16(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 16)
    }
    ///Bit 17 - ch17 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr17(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 17)
    }
    ///Bit 18 - ch18 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr18(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 18)
    }
    ///Bit 19 - ch19 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr19(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 19)
    }
    ///Bit 20 - ch20 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr20(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 20)
    }
    ///Bit 21 - ch21 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr21(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 21)
    }
    ///Bit 22 - ch22 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr22(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 22)
    }
    ///Bit 23 - ch23 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr23(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 23)
    }
    ///Bit 24 - ch24 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr24(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 24)
    }
    ///Bit 25 - ch25 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr25(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 25)
    }
    ///Bit 26 - ch26 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr26(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 26)
    }
    ///Bit 27 - ch27 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr27(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 27)
    }
    ///Bit 28 - ch28 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr28(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 28)
    }
    ///Bit 29 - ch29 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr29(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 29)
    }
    ///Bit 30 - ch30 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr30(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 30)
    }
    ///Bit 31 - ch31 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr31(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 31)
    }
}
/**channel enable clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_ENA_AD0_CLR_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ch_ena_ad0_clr::W`](W) writer structure
impl crate::Writable for CH_ENA_AD0_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH_ENA_AD0_CLR to value 0
impl crate::Resettable for CH_ENA_AD0_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
