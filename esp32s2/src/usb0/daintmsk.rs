#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DAINTMSK_SPEC>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DAINTMSK_SPEC>;
#[doc = "Field `INEPMSK(0-6)` reader - "]
pub type INEPMSK_R = crate::BitReader;
#[doc = "Field `INEPMSK(0-6)` writer - "]
pub type INEPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEPMSK(0-6)` reader - "]
pub type OUTEPMSK_R = crate::BitReader;
#[doc = "Field `OUTEPMSK(0-6)` writer - "]
pub type OUTEPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `INEPMSK0` field"]
    #[inline(always)]
    pub fn inepmsk(&self, n: u8) -> INEPMSK_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        INEPMSK_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn inepmsk_iter(&self) -> impl Iterator<Item = INEPMSK_R> + '_ {
        (0..7).map(move |n| INEPMSK_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - INEPMSK0"]
    #[inline(always)]
    pub fn inepmsk0(&self) -> INEPMSK_R {
        INEPMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INEPMSK1"]
    #[inline(always)]
    pub fn inepmsk1(&self) -> INEPMSK_R {
        INEPMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INEPMSK2"]
    #[inline(always)]
    pub fn inepmsk2(&self) -> INEPMSK_R {
        INEPMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INEPMSK3"]
    #[inline(always)]
    pub fn inepmsk3(&self) -> INEPMSK_R {
        INEPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INEPMSK4"]
    #[inline(always)]
    pub fn inepmsk4(&self) -> INEPMSK_R {
        INEPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - INEPMSK5"]
    #[inline(always)]
    pub fn inepmsk5(&self) -> INEPMSK_R {
        INEPMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPMSK6"]
    #[inline(always)]
    pub fn inepmsk6(&self) -> INEPMSK_R {
        INEPMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OUTEPMSK0` field"]
    #[inline(always)]
    pub fn outepmsk(&self, n: u8) -> OUTEPMSK_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        OUTEPMSK_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn outepmsk_iter(&self) -> impl Iterator<Item = OUTEPMSK_R> + '_ {
        (0..7).map(move |n| OUTEPMSK_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - OUTEPMSK0"]
    #[inline(always)]
    pub fn outepmsk0(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OUTEPMSK1"]
    #[inline(always)]
    pub fn outepmsk1(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OUTEPMSK2"]
    #[inline(always)]
    pub fn outepmsk2(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUTEPMSK3"]
    #[inline(always)]
    pub fn outepmsk3(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OUTEPMSK4"]
    #[inline(always)]
    pub fn outepmsk4(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OUTEPMSK5"]
    #[inline(always)]
    pub fn outepmsk5(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OUTEPMSK6"]
    #[inline(always)]
    pub fn outepmsk6(&self) -> OUTEPMSK_R {
        OUTEPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINTMSK")
            .field("inepmsk0", &format_args!("{}", self.inepmsk0().bit()))
            .field("inepmsk1", &format_args!("{}", self.inepmsk1().bit()))
            .field("inepmsk2", &format_args!("{}", self.inepmsk2().bit()))
            .field("inepmsk3", &format_args!("{}", self.inepmsk3().bit()))
            .field("inepmsk4", &format_args!("{}", self.inepmsk4().bit()))
            .field("inepmsk5", &format_args!("{}", self.inepmsk5().bit()))
            .field("inepmsk6", &format_args!("{}", self.inepmsk6().bit()))
            .field("outepmsk0", &format_args!("{}", self.outepmsk0().bit()))
            .field("outepmsk1", &format_args!("{}", self.outepmsk1().bit()))
            .field("outepmsk2", &format_args!("{}", self.outepmsk2().bit()))
            .field("outepmsk3", &format_args!("{}", self.outepmsk3().bit()))
            .field("outepmsk4", &format_args!("{}", self.outepmsk4().bit()))
            .field("outepmsk5", &format_args!("{}", self.outepmsk5().bit()))
            .field("outepmsk6", &format_args!("{}", self.outepmsk6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DAINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `INEPMSK0` field"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk(&mut self, n: u8) -> INEPMSK_W<DAINTMSK_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        INEPMSK_W::new(self, n)
    }
    #[doc = "Bit 0 - INEPMSK0"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk0(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - INEPMSK1"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk1(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - INEPMSK2"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk2(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - INEPMSK3"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk3(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - INEPMSK4"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk4(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - INEPMSK5"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk5(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - INEPMSK6"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk6(&mut self) -> INEPMSK_W<DAINTMSK_SPEC> {
        INEPMSK_W::new(self, 6)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OUTEPMSK0` field"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk(&mut self, n: u8) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        OUTEPMSK_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - OUTEPMSK0"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk0(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 16)
    }
    #[doc = "Bit 17 - OUTEPMSK1"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk1(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 17)
    }
    #[doc = "Bit 18 - OUTEPMSK2"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk2(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 18)
    }
    #[doc = "Bit 19 - OUTEPMSK3"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk3(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 19)
    }
    #[doc = "Bit 20 - OUTEPMSK4"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk4(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 20)
    }
    #[doc = "Bit 21 - OUTEPMSK5"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk5(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 21)
    }
    #[doc = "Bit 22 - OUTEPMSK6"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk6(&mut self) -> OUTEPMSK_W<DAINTMSK_SPEC> {
        OUTEPMSK_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DAINTMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
