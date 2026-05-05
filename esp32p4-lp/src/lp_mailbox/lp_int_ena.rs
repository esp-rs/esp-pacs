#[doc = "Register `LP_INT_ENA` reader"]
pub type R = crate::R<LP_INT_ENA_SPEC>;
#[doc = "Register `LP_INT_ENA` writer"]
pub type W = crate::W<LP_INT_ENA_SPEC>;
#[doc = "Field `LP_0_INT_ENA` reader - MB_LP_0_INT_ENA"]
pub type LP_0_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_0_INT_ENA` writer - MB_LP_0_INT_ENA"]
pub type LP_0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_1_INT_ENA` reader - MB_LP_1_INT_ENA"]
pub type LP_1_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_1_INT_ENA` writer - MB_LP_1_INT_ENA"]
pub type LP_1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_2_INT_ENA` reader - MB_LP_2_INT_ENA"]
pub type LP_2_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_2_INT_ENA` writer - MB_LP_2_INT_ENA"]
pub type LP_2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_3_INT_ENA` reader - MB_LP_3_INT_ENA"]
pub type LP_3_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_3_INT_ENA` writer - MB_LP_3_INT_ENA"]
pub type LP_3_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_4_INT_ENA` reader - MB_LP_4_INT_ENA"]
pub type LP_4_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_4_INT_ENA` writer - MB_LP_4_INT_ENA"]
pub type LP_4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_5_INT_ENA` reader - MB_LP_5_INT_ENA"]
pub type LP_5_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_5_INT_ENA` writer - MB_LP_5_INT_ENA"]
pub type LP_5_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_6_INT_ENA` reader - MB_LP_6_INT_ENA"]
pub type LP_6_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_6_INT_ENA` writer - MB_LP_6_INT_ENA"]
pub type LP_6_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_7_INT_ENA` reader - MB_LP_7_INT_ENA"]
pub type LP_7_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_7_INT_ENA` writer - MB_LP_7_INT_ENA"]
pub type LP_7_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_8_INT_ENA` reader - MB_LP_8_INT_ENA"]
pub type LP_8_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_8_INT_ENA` writer - MB_LP_8_INT_ENA"]
pub type LP_8_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_9_INT_ENA` reader - MB_LP_9_INT_ENA"]
pub type LP_9_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_9_INT_ENA` writer - MB_LP_9_INT_ENA"]
pub type LP_9_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_10_INT_ENA` reader - MB_LP_10_INT_ENA"]
pub type LP_10_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_10_INT_ENA` writer - MB_LP_10_INT_ENA"]
pub type LP_10_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_11_INT_ENA` reader - MB_LP_11_INT_ENA"]
pub type LP_11_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_11_INT_ENA` writer - MB_LP_11_INT_ENA"]
pub type LP_11_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_12_INT_ENA` reader - MB_LP_12_INT_ENA"]
pub type LP_12_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_12_INT_ENA` writer - MB_LP_12_INT_ENA"]
pub type LP_12_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_13_INT_ENA` reader - MB_LP_13_INT_ENA"]
pub type LP_13_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_13_INT_ENA` writer - MB_LP_13_INT_ENA"]
pub type LP_13_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_14_INT_ENA` reader - MB_LP_14_INT_ENA"]
pub type LP_14_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_14_INT_ENA` writer - MB_LP_14_INT_ENA"]
pub type LP_14_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_15_INT_ENA` reader - MB_LP_15_INT_ENA"]
pub type LP_15_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_15_INT_ENA` writer - MB_LP_15_INT_ENA"]
pub type LP_15_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MB_LP_0_INT_ENA"]
    #[inline(always)]
    pub fn lp_0_int_ena(&self) -> LP_0_INT_ENA_R {
        LP_0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MB_LP_1_INT_ENA"]
    #[inline(always)]
    pub fn lp_1_int_ena(&self) -> LP_1_INT_ENA_R {
        LP_1_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MB_LP_2_INT_ENA"]
    #[inline(always)]
    pub fn lp_2_int_ena(&self) -> LP_2_INT_ENA_R {
        LP_2_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MB_LP_3_INT_ENA"]
    #[inline(always)]
    pub fn lp_3_int_ena(&self) -> LP_3_INT_ENA_R {
        LP_3_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MB_LP_4_INT_ENA"]
    #[inline(always)]
    pub fn lp_4_int_ena(&self) -> LP_4_INT_ENA_R {
        LP_4_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MB_LP_5_INT_ENA"]
    #[inline(always)]
    pub fn lp_5_int_ena(&self) -> LP_5_INT_ENA_R {
        LP_5_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MB_LP_6_INT_ENA"]
    #[inline(always)]
    pub fn lp_6_int_ena(&self) -> LP_6_INT_ENA_R {
        LP_6_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MB_LP_7_INT_ENA"]
    #[inline(always)]
    pub fn lp_7_int_ena(&self) -> LP_7_INT_ENA_R {
        LP_7_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MB_LP_8_INT_ENA"]
    #[inline(always)]
    pub fn lp_8_int_ena(&self) -> LP_8_INT_ENA_R {
        LP_8_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MB_LP_9_INT_ENA"]
    #[inline(always)]
    pub fn lp_9_int_ena(&self) -> LP_9_INT_ENA_R {
        LP_9_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MB_LP_10_INT_ENA"]
    #[inline(always)]
    pub fn lp_10_int_ena(&self) -> LP_10_INT_ENA_R {
        LP_10_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MB_LP_11_INT_ENA"]
    #[inline(always)]
    pub fn lp_11_int_ena(&self) -> LP_11_INT_ENA_R {
        LP_11_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MB_LP_12_INT_ENA"]
    #[inline(always)]
    pub fn lp_12_int_ena(&self) -> LP_12_INT_ENA_R {
        LP_12_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MB_LP_13_INT_ENA"]
    #[inline(always)]
    pub fn lp_13_int_ena(&self) -> LP_13_INT_ENA_R {
        LP_13_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MB_LP_14_INT_ENA"]
    #[inline(always)]
    pub fn lp_14_int_ena(&self) -> LP_14_INT_ENA_R {
        LP_14_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MB_LP_15_INT_ENA"]
    #[inline(always)]
    pub fn lp_15_int_ena(&self) -> LP_15_INT_ENA_R {
        LP_15_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ENA")
            .field("lp_0_int_ena", &self.lp_0_int_ena())
            .field("lp_1_int_ena", &self.lp_1_int_ena())
            .field("lp_2_int_ena", &self.lp_2_int_ena())
            .field("lp_3_int_ena", &self.lp_3_int_ena())
            .field("lp_4_int_ena", &self.lp_4_int_ena())
            .field("lp_5_int_ena", &self.lp_5_int_ena())
            .field("lp_6_int_ena", &self.lp_6_int_ena())
            .field("lp_7_int_ena", &self.lp_7_int_ena())
            .field("lp_8_int_ena", &self.lp_8_int_ena())
            .field("lp_9_int_ena", &self.lp_9_int_ena())
            .field("lp_10_int_ena", &self.lp_10_int_ena())
            .field("lp_11_int_ena", &self.lp_11_int_ena())
            .field("lp_12_int_ena", &self.lp_12_int_ena())
            .field("lp_13_int_ena", &self.lp_13_int_ena())
            .field("lp_14_int_ena", &self.lp_14_int_ena())
            .field("lp_15_int_ena", &self.lp_15_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MB_LP_0_INT_ENA"]
    #[inline(always)]
    pub fn lp_0_int_ena(&mut self) -> LP_0_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - MB_LP_1_INT_ENA"]
    #[inline(always)]
    pub fn lp_1_int_ena(&mut self) -> LP_1_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_1_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - MB_LP_2_INT_ENA"]
    #[inline(always)]
    pub fn lp_2_int_ena(&mut self) -> LP_2_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_2_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - MB_LP_3_INT_ENA"]
    #[inline(always)]
    pub fn lp_3_int_ena(&mut self) -> LP_3_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_3_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - MB_LP_4_INT_ENA"]
    #[inline(always)]
    pub fn lp_4_int_ena(&mut self) -> LP_4_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_4_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - MB_LP_5_INT_ENA"]
    #[inline(always)]
    pub fn lp_5_int_ena(&mut self) -> LP_5_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_5_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - MB_LP_6_INT_ENA"]
    #[inline(always)]
    pub fn lp_6_int_ena(&mut self) -> LP_6_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_6_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - MB_LP_7_INT_ENA"]
    #[inline(always)]
    pub fn lp_7_int_ena(&mut self) -> LP_7_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_7_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - MB_LP_8_INT_ENA"]
    #[inline(always)]
    pub fn lp_8_int_ena(&mut self) -> LP_8_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_8_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - MB_LP_9_INT_ENA"]
    #[inline(always)]
    pub fn lp_9_int_ena(&mut self) -> LP_9_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_9_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - MB_LP_10_INT_ENA"]
    #[inline(always)]
    pub fn lp_10_int_ena(&mut self) -> LP_10_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_10_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - MB_LP_11_INT_ENA"]
    #[inline(always)]
    pub fn lp_11_int_ena(&mut self) -> LP_11_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_11_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - MB_LP_12_INT_ENA"]
    #[inline(always)]
    pub fn lp_12_int_ena(&mut self) -> LP_12_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_12_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - MB_LP_13_INT_ENA"]
    #[inline(always)]
    pub fn lp_13_int_ena(&mut self) -> LP_13_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_13_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - MB_LP_14_INT_ENA"]
    #[inline(always)]
    pub fn lp_14_int_ena(&mut self) -> LP_14_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_14_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - MB_LP_15_INT_ENA"]
    #[inline(always)]
    pub fn lp_15_int_ena(&mut self) -> LP_15_INT_ENA_W<'_, LP_INT_ENA_SPEC> {
        LP_15_INT_ENA_W::new(self, 15)
    }
}
#[doc = "MB_LP_INT_ENA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ENA_SPEC;
impl crate::RegisterSpec for LP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LP_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LP_INT_ENA_SPEC {}
