#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `PAD_COMP_CHANNEL_3_NEG_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_3_NEG_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_3_NEG_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_3_NEG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_3_POS_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_3_POS_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_3_POS_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_3_POS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_3_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_3_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_3_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_3_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_2_NEG_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_2_NEG_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_2_NEG_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_2_NEG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_2_POS_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_2_POS_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_2_POS_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_2_POS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_2_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_2_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_2_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_1_NEG_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_1_NEG_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_1_NEG_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_1_NEG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_1_POS_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_1_POS_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_1_POS_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_1_POS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_CHANNEL_1_INT_RAW` reader - reserved"]
pub type PAD_COMP_CHANNEL_1_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_1_INT_RAW` writer - reserved"]
pub type PAD_COMP_CHANNEL_1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_NEG_INT_RAW` reader - reserved"]
pub type PAD_COMP_NEG_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_NEG_INT_RAW` writer - reserved"]
pub type PAD_COMP_NEG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_POS_INT_RAW` reader - reserved"]
pub type PAD_COMP_POS_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_POS_INT_RAW` writer - reserved"]
pub type PAD_COMP_POS_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_INT_RAW` reader - reserved"]
pub type PAD_COMP_INT_RAW_R = crate::BitReader;
#[doc = "Field `PAD_COMP_INT_RAW` writer - reserved"]
pub type PAD_COMP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_neg_int_raw(&self) -> PAD_COMP_CHANNEL_3_NEG_INT_RAW_R {
        PAD_COMP_CHANNEL_3_NEG_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_pos_int_raw(&self) -> PAD_COMP_CHANNEL_3_POS_INT_RAW_R {
        PAD_COMP_CHANNEL_3_POS_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_int_raw(&self) -> PAD_COMP_CHANNEL_3_INT_RAW_R {
        PAD_COMP_CHANNEL_3_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_neg_int_raw(&self) -> PAD_COMP_CHANNEL_2_NEG_INT_RAW_R {
        PAD_COMP_CHANNEL_2_NEG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_pos_int_raw(&self) -> PAD_COMP_CHANNEL_2_POS_INT_RAW_R {
        PAD_COMP_CHANNEL_2_POS_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_int_raw(&self) -> PAD_COMP_CHANNEL_2_INT_RAW_R {
        PAD_COMP_CHANNEL_2_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_neg_int_raw(&self) -> PAD_COMP_CHANNEL_1_NEG_INT_RAW_R {
        PAD_COMP_CHANNEL_1_NEG_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_pos_int_raw(&self) -> PAD_COMP_CHANNEL_1_POS_INT_RAW_R {
        PAD_COMP_CHANNEL_1_POS_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_int_raw(&self) -> PAD_COMP_CHANNEL_1_INT_RAW_R {
        PAD_COMP_CHANNEL_1_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn pad_comp_neg_int_raw(&self) -> PAD_COMP_NEG_INT_RAW_R {
        PAD_COMP_NEG_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn pad_comp_pos_int_raw(&self) -> PAD_COMP_POS_INT_RAW_R {
        PAD_COMP_POS_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn pad_comp_int_raw(&self) -> PAD_COMP_INT_RAW_R {
        PAD_COMP_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "pad_comp_channel_3_neg_int_raw",
                &self.pad_comp_channel_3_neg_int_raw(),
            )
            .field(
                "pad_comp_channel_3_pos_int_raw",
                &self.pad_comp_channel_3_pos_int_raw(),
            )
            .field(
                "pad_comp_channel_3_int_raw",
                &self.pad_comp_channel_3_int_raw(),
            )
            .field(
                "pad_comp_channel_2_neg_int_raw",
                &self.pad_comp_channel_2_neg_int_raw(),
            )
            .field(
                "pad_comp_channel_2_pos_int_raw",
                &self.pad_comp_channel_2_pos_int_raw(),
            )
            .field(
                "pad_comp_channel_2_int_raw",
                &self.pad_comp_channel_2_int_raw(),
            )
            .field(
                "pad_comp_channel_1_neg_int_raw",
                &self.pad_comp_channel_1_neg_int_raw(),
            )
            .field(
                "pad_comp_channel_1_pos_int_raw",
                &self.pad_comp_channel_1_pos_int_raw(),
            )
            .field(
                "pad_comp_channel_1_int_raw",
                &self.pad_comp_channel_1_int_raw(),
            )
            .field("pad_comp_neg_int_raw", &self.pad_comp_neg_int_raw())
            .field("pad_comp_pos_int_raw", &self.pad_comp_pos_int_raw())
            .field("pad_comp_int_raw", &self.pad_comp_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_neg_int_raw(
        &mut self,
    ) -> PAD_COMP_CHANNEL_3_NEG_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_3_NEG_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_pos_int_raw(
        &mut self,
    ) -> PAD_COMP_CHANNEL_3_POS_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_3_POS_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_int_raw(&mut self) -> PAD_COMP_CHANNEL_3_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_3_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_neg_int_raw(
        &mut self,
    ) -> PAD_COMP_CHANNEL_2_NEG_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_2_NEG_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_pos_int_raw(
        &mut self,
    ) -> PAD_COMP_CHANNEL_2_POS_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_2_POS_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_int_raw(&mut self) -> PAD_COMP_CHANNEL_2_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_2_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_neg_int_raw(
        &mut self,
    ) -> PAD_COMP_CHANNEL_1_NEG_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_1_NEG_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_pos_int_raw(
        &mut self,
    ) -> PAD_COMP_CHANNEL_1_POS_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_1_POS_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_int_raw(&mut self) -> PAD_COMP_CHANNEL_1_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_CHANNEL_1_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn pad_comp_neg_int_raw(&mut self) -> PAD_COMP_NEG_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_NEG_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn pad_comp_pos_int_raw(&mut self) -> PAD_COMP_POS_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_POS_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn pad_comp_int_raw(&mut self) -> PAD_COMP_INT_RAW_W<'_, INT_RAW_SPEC> {
        PAD_COMP_INT_RAW_W::new(self, 11)
    }
}
#[doc = "zero det int raw\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
