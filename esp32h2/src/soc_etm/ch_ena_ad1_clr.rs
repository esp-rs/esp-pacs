///Register `CH_ENA_AD1_CLR` writer
pub type W = crate::W<CH_ENA_AD1_CLR_SPEC>;
///Field `CH_CLR(32-49)` writer - ch%s clear
pub type CH_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///ch(32-49) clear
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH_CLR32` field
    #[inline(always)]
    #[must_use]
    pub fn ch_clr(&mut self, n: u8) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        CH_CLR_W::new(self, n)
    }
    ///Bit 0 - ch32 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr32(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 0)
    }
    ///Bit 1 - ch33 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr33(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 1)
    }
    ///Bit 2 - ch34 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr34(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 2)
    }
    ///Bit 3 - ch35 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr35(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 3)
    }
    ///Bit 4 - ch36 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr36(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 4)
    }
    ///Bit 5 - ch37 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr37(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 5)
    }
    ///Bit 6 - ch38 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr38(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 6)
    }
    ///Bit 7 - ch39 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr39(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 7)
    }
    ///Bit 8 - ch40 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr40(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 8)
    }
    ///Bit 9 - ch41 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr41(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 9)
    }
    ///Bit 10 - ch42 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr42(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 10)
    }
    ///Bit 11 - ch43 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr43(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 11)
    }
    ///Bit 12 - ch44 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr44(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 12)
    }
    ///Bit 13 - ch45 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr45(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 13)
    }
    ///Bit 14 - ch46 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr46(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 14)
    }
    ///Bit 15 - ch47 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr47(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 15)
    }
    ///Bit 16 - ch48 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr48(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 16)
    }
    ///Bit 17 - ch49 clear
    #[inline(always)]
    #[must_use]
    pub fn ch_clr49(&mut self) -> CH_CLR_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR_W::new(self, 17)
    }
}
/**channel enable clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_ENA_AD1_CLR_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ch_ena_ad1_clr::W`](W) writer structure
impl crate::Writable for CH_ENA_AD1_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH_ENA_AD1_CLR to value 0
impl crate::Resettable for CH_ENA_AD1_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
