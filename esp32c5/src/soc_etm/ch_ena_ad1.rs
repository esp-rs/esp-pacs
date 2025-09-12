#[doc = "Register `CH_ENA_AD1` reader"]
pub type R = crate::R<CH_ENA_AD1_SPEC>;
#[doc = "Register `CH_ENA_AD1` writer"]
pub type W = crate::W<CH_ENA_AD1_SPEC>;
#[doc = "Field `CH_ENABLED(32-49)` reader - Represents ch%s enable status.\\\\0: Disable\\\\1: Enable"]
pub type CH_ENABLED_R = crate::BitReader;
#[doc = "Field `CH_ENABLED(32-49)` writer - Represents ch%s enable status.\\\\0: Disable\\\\1: Enable"]
pub type CH_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Represents ch(32-49) enable status.\\\\0: Disable\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_ENABLED32` field.</div>"]
    #[inline(always)]
    pub fn ch_enabled(&self, n: u8) -> CH_ENABLED_R {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        CH_ENABLED_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Represents ch(32-49) enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled_iter(&self) -> impl Iterator<Item = CH_ENABLED_R> + '_ {
        (0..18).map(move |n| CH_ENABLED_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Represents ch32 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled32(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents ch33 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled33(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents ch34 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled34(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents ch35 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled35(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents ch36 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled36(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents ch37 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled37(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents ch38 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled38(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents ch39 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled39(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents ch40 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled40(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents ch41 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled41(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents ch42 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled42(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents ch43 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled43(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents ch44 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled44(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents ch45 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled45(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents ch46 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled46(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents ch47 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled47(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents ch48 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled48(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents ch49 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled49(&self) -> CH_ENABLED_R {
        CH_ENABLED_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_ENA_AD1")
            .field("ch_enabled32", &self.ch_enabled32())
            .field("ch_enabled33", &self.ch_enabled33())
            .field("ch_enabled34", &self.ch_enabled34())
            .field("ch_enabled35", &self.ch_enabled35())
            .field("ch_enabled36", &self.ch_enabled36())
            .field("ch_enabled37", &self.ch_enabled37())
            .field("ch_enabled38", &self.ch_enabled38())
            .field("ch_enabled39", &self.ch_enabled39())
            .field("ch_enabled40", &self.ch_enabled40())
            .field("ch_enabled41", &self.ch_enabled41())
            .field("ch_enabled42", &self.ch_enabled42())
            .field("ch_enabled43", &self.ch_enabled43())
            .field("ch_enabled44", &self.ch_enabled44())
            .field("ch_enabled45", &self.ch_enabled45())
            .field("ch_enabled46", &self.ch_enabled46())
            .field("ch_enabled47", &self.ch_enabled47())
            .field("ch_enabled48", &self.ch_enabled48())
            .field("ch_enabled49", &self.ch_enabled49())
            .finish()
    }
}
impl W {
    #[doc = "Represents ch(32-49) enable status.\\\\0: Disable\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_ENABLED32` field.</div>"]
    #[inline(always)]
    pub fn ch_enabled(&mut self, n: u8) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        CH_ENABLED_W::new(self, n)
    }
    #[doc = "Bit 0 - Represents ch32 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled32(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents ch33 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled33(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents ch34 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled34(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents ch35 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled35(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents ch36 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled36(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents ch37 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled37(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents ch38 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled38(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents ch39 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled39(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents ch40 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled40(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents ch41 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled41(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents ch42 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled42(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents ch43 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled43(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents ch44 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled44(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents ch45 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled45(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents ch46 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled46(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents ch47 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled47(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents ch48 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled48(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents ch49 enable status.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enabled49(&mut self) -> CH_ENABLED_W<'_, CH_ENA_AD1_SPEC> {
        CH_ENABLED_W::new(self, 17)
    }
}
#[doc = "Channel enable status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ena_ad1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD1_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ena_ad1::R`](R) reader structure"]
impl crate::Readable for CH_ENA_AD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad1::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_ENA_AD1 to value 0"]
impl crate::Resettable for CH_ENA_AD1_SPEC {}
