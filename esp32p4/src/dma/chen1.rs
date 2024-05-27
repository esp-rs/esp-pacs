#[doc = "Register `CHEN1` reader"]
pub type R = crate::R<CHEN1_SPEC>;
#[doc = "Register `CHEN1` writer"]
pub type W = crate::W<CHEN1_SPEC>;
#[doc = "Field `CH1_ABORT` reader - NA"]
pub type CH1_ABORT_R = crate::BitReader;
#[doc = "Field `CH1_ABORT` writer - NA"]
pub type CH1_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_ABORT` reader - NA"]
pub type CH2_ABORT_R = crate::BitReader;
#[doc = "Field `CH2_ABORT` writer - NA"]
pub type CH2_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_ABORT` reader - NA"]
pub type CH3_ABORT_R = crate::BitReader;
#[doc = "Field `CH3_ABORT` writer - NA"]
pub type CH3_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_ABORT` reader - NA"]
pub type CH4_ABORT_R = crate::BitReader;
#[doc = "Field `CH4_ABORT` writer - NA"]
pub type CH4_ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_ABORT_WE` writer - NA"]
pub type CH1_ABORT_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_ABORT_WE` writer - NA"]
pub type CH2_ABORT_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_ABORT_WE` writer - NA"]
pub type CH3_ABORT_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_ABORT_WE` writer - NA"]
pub type CH4_ABORT_WE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_abort(&self) -> CH1_ABORT_R {
        CH1_ABORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch2_abort(&self) -> CH2_ABORT_R {
        CH2_ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch3_abort(&self) -> CH3_ABORT_R {
        CH3_ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch4_abort(&self) -> CH4_ABORT_R {
        CH4_ABORT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEN1")
            .field("ch1_abort", &self.ch1_abort())
            .field("ch2_abort", &self.ch2_abort())
            .field("ch3_abort", &self.ch3_abort())
            .field("ch4_abort", &self.ch4_abort())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_abort(&mut self) -> CH1_ABORT_W<CHEN1_SPEC> {
        CH1_ABORT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_abort(&mut self) -> CH2_ABORT_W<CHEN1_SPEC> {
        CH2_ABORT_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_abort(&mut self) -> CH3_ABORT_W<CHEN1_SPEC> {
        CH3_ABORT_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_abort(&mut self) -> CH4_ABORT_W<CHEN1_SPEC> {
        CH4_ABORT_W::new(self, 3)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_abort_we(&mut self) -> CH1_ABORT_WE_W<CHEN1_SPEC> {
        CH1_ABORT_WE_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_abort_we(&mut self) -> CH2_ABORT_WE_W<CHEN1_SPEC> {
        CH2_ABORT_WE_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_abort_we(&mut self) -> CH3_ABORT_WE_W<CHEN1_SPEC> {
        CH3_ABORT_WE_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_abort_we(&mut self) -> CH4_ABORT_WE_W<CHEN1_SPEC> {
        CH4_ABORT_WE_W::new(self, 11)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHEN1_SPEC;
impl crate::RegisterSpec for CHEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen1::R`](R) reader structure"]
impl crate::Readable for CHEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chen1::W`](W) writer structure"]
impl crate::Writable for CHEN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHEN1 to value 0"]
impl crate::Resettable for CHEN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
