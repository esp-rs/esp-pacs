#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `PAD_COMP_CHANNEL_3_NEG_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_3_NEG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_3_POS_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_3_POS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_3_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_3_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_2_NEG_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_2_NEG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_2_POS_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_2_POS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_2_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_1_NEG_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_1_NEG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_1_POS_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_1_POS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_1_INT_CLR` writer - reserved"]
pub type PAD_COMP_CHANNEL_1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_NEG_INT_CLR` writer - reserved"]
pub type PAD_COMP_NEG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_POS_INT_CLR` writer - reserved"]
pub type PAD_COMP_POS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_INT_CLR` writer - reserved"]
pub type PAD_COMP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_neg_int_clr(
        &mut self,
    ) -> PAD_COMP_CHANNEL_3_NEG_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_3_NEG_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_pos_int_clr(
        &mut self,
    ) -> PAD_COMP_CHANNEL_3_POS_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_3_POS_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_int_clr(&mut self) -> PAD_COMP_CHANNEL_3_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_3_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_neg_int_clr(
        &mut self,
    ) -> PAD_COMP_CHANNEL_2_NEG_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_2_NEG_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_pos_int_clr(
        &mut self,
    ) -> PAD_COMP_CHANNEL_2_POS_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_2_POS_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_int_clr(&mut self) -> PAD_COMP_CHANNEL_2_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_2_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_neg_int_clr(
        &mut self,
    ) -> PAD_COMP_CHANNEL_1_NEG_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_1_NEG_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_pos_int_clr(
        &mut self,
    ) -> PAD_COMP_CHANNEL_1_POS_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_1_POS_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_int_clr(&mut self) -> PAD_COMP_CHANNEL_1_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_CHANNEL_1_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn pad_comp_neg_int_clr(&mut self) -> PAD_COMP_NEG_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_NEG_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn pad_comp_pos_int_clr(&mut self) -> PAD_COMP_POS_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_POS_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn pad_comp_int_clr(&mut self) -> PAD_COMP_INT_CLR_W<'_, INT_CLR_SPEC> {
        PAD_COMP_INT_CLR_W::new(self, 11)
    }
}
#[doc = "zero det int clr\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
