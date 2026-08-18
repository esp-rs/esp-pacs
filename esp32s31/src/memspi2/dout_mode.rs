#[doc = "Register `DOUT_MODE` reader"]
pub type R = crate::R<DOUT_MODE_SPEC>;
#[doc = "Register `DOUT_MODE` writer"]
pub type W = crate::W<DOUT_MODE_SPEC>;
#[doc = "Field `DOUT_MODE(0-7)` reader - "]
pub type DOUT_MODE_R = crate::BitReader;
#[doc = "Field `DOUT_MODE(0-7)` writer - "]
pub type DOUT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTS_MODE` reader - "]
pub type DOUTS_MODE_R = crate::BitReader;
#[doc = "Field `DOUTS_MODE` writer - "]
pub type DOUTS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&self, n: u8) -> DOUT_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DOUT_MODE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn dout_mode_iter(&self) -> impl Iterator<Item = DOUT_MODE_R> + '_ {
        (0..8).map(move |n| DOUT_MODE_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DOUT0_MODE"]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOUT1_MODE"]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DOUT2_MODE"]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DOUT3_MODE"]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DOUT4_MODE"]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DOUT5_MODE"]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOUT6_MODE"]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DOUT7_MODE"]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn douts_mode(&self) -> DOUTS_MODE_R {
        DOUTS_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT_MODE")
            .field("dout0_mode", &self.dout0_mode())
            .field("dout1_mode", &self.dout1_mode())
            .field("dout2_mode", &self.dout2_mode())
            .field("dout3_mode", &self.dout3_mode())
            .field("dout4_mode", &self.dout4_mode())
            .field("dout5_mode", &self.dout5_mode())
            .field("dout6_mode", &self.dout6_mode())
            .field("dout7_mode", &self.dout7_mode())
            .field("douts_mode", &self.douts_mode())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&mut self, n: u8) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DOUT_MODE_W::new(self, n)
    }
    #[doc = "Bit 0 - DOUT0_MODE"]
    #[inline(always)]
    pub fn dout0_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DOUT1_MODE"]
    #[inline(always)]
    pub fn dout1_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DOUT2_MODE"]
    #[inline(always)]
    pub fn dout2_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DOUT3_MODE"]
    #[inline(always)]
    pub fn dout3_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - DOUT4_MODE"]
    #[inline(always)]
    pub fn dout4_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - DOUT5_MODE"]
    #[inline(always)]
    pub fn dout5_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - DOUT6_MODE"]
    #[inline(always)]
    pub fn dout6_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - DOUT7_MODE"]
    #[inline(always)]
    pub fn dout7_mode(&mut self) -> DOUT_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn douts_mode(&mut self) -> DOUTS_MODE_W<'_, DOUT_MODE_SPEC> {
        DOUTS_MODE_W::new(self, 8)
    }
}
#[doc = "MSPI flash output timing adjustment control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dout_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout_mode::R`](R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dout_mode::W`](W) writer structure"]
impl crate::Writable for DOUT_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {}
