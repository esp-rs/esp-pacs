#[doc = "Register `LP_INT_CLR` writer"]
pub type W = crate::W<LP_INT_CLR_SPEC>;
#[doc = "Field `LP_0_INT_CLR` writer - MB_LP_0_INT_CLR"]
pub type LP_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_1_INT_CLR` writer - MB_LP_1_INT_CLR"]
pub type LP_1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_2_INT_CLR` writer - MB_LP_2_INT_CLR"]
pub type LP_2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_3_INT_CLR` writer - MB_LP_3_INT_CLR"]
pub type LP_3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_4_INT_CLR` writer - MB_LP_4_INT_CLR"]
pub type LP_4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_5_INT_CLR` writer - MB_LP_5_INT_CLR"]
pub type LP_5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_6_INT_CLR` writer - MB_LP_6_INT_CLR"]
pub type LP_6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_7_INT_CLR` writer - MB_LP_7_INT_CLR"]
pub type LP_7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_8_INT_CLR` writer - MB_LP_8_INT_CLR"]
pub type LP_8_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_9_INT_CLR` writer - MB_LP_9_INT_CLR"]
pub type LP_9_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_10_INT_CLR` writer - MB_LP_10_INT_CLR"]
pub type LP_10_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_11_INT_CLR` writer - MB_LP_11_INT_CLR"]
pub type LP_11_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_12_INT_CLR` writer - MB_LP_12_INT_CLR"]
pub type LP_12_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_13_INT_CLR` writer - MB_LP_13_INT_CLR"]
pub type LP_13_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_14_INT_CLR` writer - MB_LP_14_INT_CLR"]
pub type LP_14_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_15_INT_CLR` writer - MB_LP_15_INT_CLR"]
pub type LP_15_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - MB_LP_0_INT_CLR"]
    #[inline(always)]
    pub fn lp_0_int_clr(&mut self) -> LP_0_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - MB_LP_1_INT_CLR"]
    #[inline(always)]
    pub fn lp_1_int_clr(&mut self) -> LP_1_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - MB_LP_2_INT_CLR"]
    #[inline(always)]
    pub fn lp_2_int_clr(&mut self) -> LP_2_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_2_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - MB_LP_3_INT_CLR"]
    #[inline(always)]
    pub fn lp_3_int_clr(&mut self) -> LP_3_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_3_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - MB_LP_4_INT_CLR"]
    #[inline(always)]
    pub fn lp_4_int_clr(&mut self) -> LP_4_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_4_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - MB_LP_5_INT_CLR"]
    #[inline(always)]
    pub fn lp_5_int_clr(&mut self) -> LP_5_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_5_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - MB_LP_6_INT_CLR"]
    #[inline(always)]
    pub fn lp_6_int_clr(&mut self) -> LP_6_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_6_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - MB_LP_7_INT_CLR"]
    #[inline(always)]
    pub fn lp_7_int_clr(&mut self) -> LP_7_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_7_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - MB_LP_8_INT_CLR"]
    #[inline(always)]
    pub fn lp_8_int_clr(&mut self) -> LP_8_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_8_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - MB_LP_9_INT_CLR"]
    #[inline(always)]
    pub fn lp_9_int_clr(&mut self) -> LP_9_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_9_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - MB_LP_10_INT_CLR"]
    #[inline(always)]
    pub fn lp_10_int_clr(&mut self) -> LP_10_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_10_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - MB_LP_11_INT_CLR"]
    #[inline(always)]
    pub fn lp_11_int_clr(&mut self) -> LP_11_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_11_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - MB_LP_12_INT_CLR"]
    #[inline(always)]
    pub fn lp_12_int_clr(&mut self) -> LP_12_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_12_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - MB_LP_13_INT_CLR"]
    #[inline(always)]
    pub fn lp_13_int_clr(&mut self) -> LP_13_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_13_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - MB_LP_14_INT_CLR"]
    #[inline(always)]
    pub fn lp_14_int_clr(&mut self) -> LP_14_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_14_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - MB_LP_15_INT_CLR"]
    #[inline(always)]
    pub fn lp_15_int_clr(&mut self) -> LP_15_INT_CLR_W<'_, LP_INT_CLR_SPEC> {
        LP_15_INT_CLR_W::new(self, 15)
    }
}
#[doc = "MB_LP_INT_CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_int_clr::W`](W) writer structure"]
impl crate::Writable for LP_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_CLR to value 0"]
impl crate::Resettable for LP_INT_CLR_SPEC {}
