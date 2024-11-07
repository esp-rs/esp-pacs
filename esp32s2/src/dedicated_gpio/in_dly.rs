#[doc = "Register `IN_DLY` reader"]
pub type R = crate::R<IN_DLY_SPEC>;
#[doc = "Register `IN_DLY` writer"]
pub type W = crate::W<IN_DLY_SPEC>;
#[doc = "Field `CH(0-7)` reader - Configure GPIO%s input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH_R = crate::FieldReader;
#[doc = "Field `CH(0-7)` writer - Configure GPIO%s input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Configure GPIO(0-7) input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0` field.</div>"]
    #[inline(always)]
    pub fn ch(&self, n: u8) -> CH_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configure GPIO(0-7) input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = CH_R> + '_ {
        (0..8).map(move |n| CH_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch0(&self) -> CH_R {
        CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch1(&self) -> CH_R {
        CH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch2(&self) -> CH_R {
        CH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch3(&self) -> CH_R {
        CH_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch4(&self) -> CH_R {
        CH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch5(&self) -> CH_R {
        CH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch6(&self) -> CH_R {
        CH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch7(&self) -> CH_R {
        CH_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DLY")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .finish()
    }
}
impl W {
    #[doc = "Configure GPIO(0-7) input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0` field.</div>"]
    #[inline(always)]
    pub fn ch(&mut self, n: u8) -> CH_W<IN_DLY_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH_W<IN_DLY_SPEC> {
        CH_W::new(self, 14)
    }
}
#[doc = "Dedicated GPIO input delay configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DLY_SPEC;
impl crate::RegisterSpec for IN_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dly::R`](R) reader structure"]
impl crate::Readable for IN_DLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_dly::W`](W) writer structure"]
impl crate::Writable for IN_DLY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_DLY to value 0"]
impl crate::Resettable for IN_DLY_SPEC {
    const RESET_VALUE: u32 = 0;
}
