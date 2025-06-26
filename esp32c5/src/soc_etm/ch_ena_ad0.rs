#[doc = "Register `CH_ENA_AD0` reader"]
pub type R = crate::R<CH_ENA_AD0_SPEC>;
#[doc = "Register `CH_ENA_AD0` writer"]
pub type W = crate::W<CH_ENA_AD0_SPEC>;
#[doc = "Field `CH_ENABLED(0-31)` reader - Represents ch%s enable status.\\\\0: Disable\\\\1: Enable"]
pub type CH_ENABLED_R = crate::BitReader;
#[doc = "Field `CH_ENABLED(0-31)` writer - Represents ch%s enable status.\\\\0: Disable\\\\1: Enable"]
pub type CH_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Represents ch(0-31) enable status.\\\\0: Disable\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_ENABLED0` field.</div>"]
    #[inline(always)]
    pub fn ch_enabled(&self, n: u8) -> CH_ENABLED_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_ENABLED_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Represents ch(0-31) enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled_iter(&self) -> impl Iterator<Item = CH_ENABLED_R> + '_ {
        (0..32).map(move |n| CH_ENABLED_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Represents ch0 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled0(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents ch1 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled1(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents ch2 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled2(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents ch3 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled3(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents ch4 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled4(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents ch5 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled5(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents ch6 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled6(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents ch7 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled7(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents ch8 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled8(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents ch9 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled9(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents ch10 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled10(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents ch11 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled11(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents ch12 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled12(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents ch13 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled13(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents ch14 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled14(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents ch15 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled15(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents ch16 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled16(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents ch17 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled17(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents ch18 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled18(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents ch19 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled19(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents ch20 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled20(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents ch21 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled21(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents ch22 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled22(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents ch23 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled23(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents ch24 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled24(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents ch25 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled25(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents ch26 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled26(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents ch27 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled27(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents ch28 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled28(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents ch29 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled29(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents ch30 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled30(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents ch31 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled31(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_ENA_AD0")
            .field("ch_enabled0", &self.ch_enabled0())
            .field("ch_enabled1", &self.ch_enabled1())
            .field("ch_enabled2", &self.ch_enabled2())
            .field("ch_enabled3", &self.ch_enabled3())
            .field("ch_enabled4", &self.ch_enabled4())
            .field("ch_enabled5", &self.ch_enabled5())
            .field("ch_enabled6", &self.ch_enabled6())
            .field("ch_enabled7", &self.ch_enabled7())
            .field("ch_enabled8", &self.ch_enabled8())
            .field("ch_enabled9", &self.ch_enabled9())
            .field("ch_enabled10", &self.ch_enabled10())
            .field("ch_enabled11", &self.ch_enabled11())
            .field("ch_enabled12", &self.ch_enabled12())
            .field("ch_enabled13", &self.ch_enabled13())
            .field("ch_enabled14", &self.ch_enabled14())
            .field("ch_enabled15", &self.ch_enabled15())
            .field("ch_enabled16", &self.ch_enabled16())
            .field("ch_enabled17", &self.ch_enabled17())
            .field("ch_enabled18", &self.ch_enabled18())
            .field("ch_enabled19", &self.ch_enabled19())
            .field("ch_enabled20", &self.ch_enabled20())
            .field("ch_enabled21", &self.ch_enabled21())
            .field("ch_enabled22", &self.ch_enabled22())
            .field("ch_enabled23", &self.ch_enabled23())
            .field("ch_enabled24", &self.ch_enabled24())
            .field("ch_enabled25", &self.ch_enabled25())
            .field("ch_enabled26", &self.ch_enabled26())
            .field("ch_enabled27", &self.ch_enabled27())
            .field("ch_enabled28", &self.ch_enabled28())
            .field("ch_enabled29", &self.ch_enabled29())
            .field("ch_enabled30", &self.ch_enabled30())
            .field("ch_enabled31", &self.ch_enabled31())
            .finish()
    }
}
impl W {
    #[doc = "Represents ch(0-31) enable status.\\\\0: Disable\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_ENABLED0` field.</div>"]
    #[inline(always)]
    pub fn ch_enabled(&mut self, n: u8) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_ENABLED_W::new(self, n)
    }
    #[doc = "Bit 0 - Represents ch0 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled0(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents ch1 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled1(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents ch2 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled2(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents ch3 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled3(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents ch4 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled4(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents ch5 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled5(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents ch6 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled6(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents ch7 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled7(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents ch8 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled8(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents ch9 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled9(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents ch10 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled10(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents ch11 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled11(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents ch12 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled12(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents ch13 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled13(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents ch14 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled14(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents ch15 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled15(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents ch16 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled16(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents ch17 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled17(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents ch18 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled18(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents ch19 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled19(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents ch20 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled20(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents ch21 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled21(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents ch22 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled22(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 22)
    }
    #[doc = "Bit 23 - Represents ch23 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled23(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 23)
    }
    #[doc = "Bit 24 - Represents ch24 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled24(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 24)
    }
    #[doc = "Bit 25 - Represents ch25 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled25(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 25)
    }
    #[doc = "Bit 26 - Represents ch26 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled26(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 26)
    }
    #[doc = "Bit 27 - Represents ch27 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled27(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 27)
    }
    #[doc = "Bit 28 - Represents ch28 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled28(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 28)
    }
    #[doc = "Bit 29 - Represents ch29 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled29(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 29)
    }
    #[doc = "Bit 30 - Represents ch30 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled30(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents ch31 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled31(&mut self) -> CH_ENABLED_W<CH_ENA_AD0_SPEC> {
        CH_ENABLED_W::new(self, 31)
    }
}
#[doc = "Channel enable status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ena_ad0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD0_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ena_ad0::R`](R) reader structure"]
impl crate::Readable for CH_ENA_AD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad0::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD0 to value 0"]
impl crate::Resettable for CH_ENA_AD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
