#[doc = "Register `HP_INT_CLR` writer"]
pub type W = crate::W<HP_INT_CLR_SPEC>;
#[doc = "Field `HP_0_INT_CLR` writer - MB_HP_0_INT_CLR"]
pub type HP_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_1_INT_CLR` writer - MB_HP_1_INT_CLR"]
pub type HP_1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_2_INT_CLR` writer - MB_HP_2_INT_CLR"]
pub type HP_2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_3_INT_CLR` writer - MB_HP_3_INT_CLR"]
pub type HP_3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_4_INT_CLR` writer - MB_HP_4_INT_CLR"]
pub type HP_4_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_5_INT_CLR` writer - MB_HP_5_INT_CLR"]
pub type HP_5_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_6_INT_CLR` writer - MB_HP_6_INT_CLR"]
pub type HP_6_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_7_INT_CLR` writer - MB_HP_7_INT_CLR"]
pub type HP_7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_8_INT_CLR` writer - MB_HP_8_INT_CLR"]
pub type HP_8_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_9_INT_CLR` writer - MB_HP_9_INT_CLR"]
pub type HP_9_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_10_INT_CLR` writer - MB_HP_10_INT_CLR"]
pub type HP_10_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_11_INT_CLR` writer - MB_HP_11_INT_CLR"]
pub type HP_11_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_12_INT_CLR` writer - MB_HP_12_INT_CLR"]
pub type HP_12_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_13_INT_CLR` writer - MB_HP_13_INT_CLR"]
pub type HP_13_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_14_INT_CLR` writer - MB_HP_14_INT_CLR"]
pub type HP_14_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_15_INT_CLR` writer - MB_HP_15_INT_CLR"]
pub type HP_15_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - MB_HP_0_INT_CLR"]
    #[inline(always)]
    pub fn hp_0_int_clr(&mut self) -> HP_0_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - MB_HP_1_INT_CLR"]
    #[inline(always)]
    pub fn hp_1_int_clr(&mut self) -> HP_1_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - MB_HP_2_INT_CLR"]
    #[inline(always)]
    pub fn hp_2_int_clr(&mut self) -> HP_2_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_2_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - MB_HP_3_INT_CLR"]
    #[inline(always)]
    pub fn hp_3_int_clr(&mut self) -> HP_3_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_3_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - MB_HP_4_INT_CLR"]
    #[inline(always)]
    pub fn hp_4_int_clr(&mut self) -> HP_4_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_4_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - MB_HP_5_INT_CLR"]
    #[inline(always)]
    pub fn hp_5_int_clr(&mut self) -> HP_5_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_5_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - MB_HP_6_INT_CLR"]
    #[inline(always)]
    pub fn hp_6_int_clr(&mut self) -> HP_6_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_6_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - MB_HP_7_INT_CLR"]
    #[inline(always)]
    pub fn hp_7_int_clr(&mut self) -> HP_7_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_7_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - MB_HP_8_INT_CLR"]
    #[inline(always)]
    pub fn hp_8_int_clr(&mut self) -> HP_8_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_8_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - MB_HP_9_INT_CLR"]
    #[inline(always)]
    pub fn hp_9_int_clr(&mut self) -> HP_9_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_9_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - MB_HP_10_INT_CLR"]
    #[inline(always)]
    pub fn hp_10_int_clr(&mut self) -> HP_10_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_10_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - MB_HP_11_INT_CLR"]
    #[inline(always)]
    pub fn hp_11_int_clr(&mut self) -> HP_11_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_11_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - MB_HP_12_INT_CLR"]
    #[inline(always)]
    pub fn hp_12_int_clr(&mut self) -> HP_12_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_12_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - MB_HP_13_INT_CLR"]
    #[inline(always)]
    pub fn hp_13_int_clr(&mut self) -> HP_13_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_13_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - MB_HP_14_INT_CLR"]
    #[inline(always)]
    pub fn hp_14_int_clr(&mut self) -> HP_14_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_14_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - MB_HP_15_INT_CLR"]
    #[inline(always)]
    pub fn hp_15_int_clr(&mut self) -> HP_15_INT_CLR_W<'_, HP_INT_CLR_SPEC> {
        HP_15_INT_CLR_W::new(self, 15)
    }
}
#[doc = "MB_HP_INT_CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_int_clr::W`](W) writer structure"]
impl crate::Writable for HP_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_INT_CLR to value 0"]
impl crate::Resettable for HP_INT_CLR_SPEC {}
