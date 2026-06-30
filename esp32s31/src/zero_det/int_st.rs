#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `PAD_COMP_CHANNEL_3_NEG_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_3_NEG_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_3_POS_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_3_POS_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_3_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_3_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_2_NEG_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_2_NEG_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_2_POS_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_2_POS_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_2_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_2_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_1_NEG_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_1_NEG_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_1_POS_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_1_POS_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_CHANNEL_1_INT_ST` reader - reserved"]
pub type PAD_COMP_CHANNEL_1_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_NEG_INT_ST` reader - reserved"]
pub type PAD_COMP_NEG_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_POS_INT_ST` reader - reserved"]
pub type PAD_COMP_POS_INT_ST_R = crate::BitReader;
#[doc = "Field `PAD_COMP_INT_ST` reader - reserved"]
pub type PAD_COMP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_neg_int_st(&self) -> PAD_COMP_CHANNEL_3_NEG_INT_ST_R {
        PAD_COMP_CHANNEL_3_NEG_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_pos_int_st(&self) -> PAD_COMP_CHANNEL_3_POS_INT_ST_R {
        PAD_COMP_CHANNEL_3_POS_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_3_int_st(&self) -> PAD_COMP_CHANNEL_3_INT_ST_R {
        PAD_COMP_CHANNEL_3_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_neg_int_st(&self) -> PAD_COMP_CHANNEL_2_NEG_INT_ST_R {
        PAD_COMP_CHANNEL_2_NEG_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_pos_int_st(&self) -> PAD_COMP_CHANNEL_2_POS_INT_ST_R {
        PAD_COMP_CHANNEL_2_POS_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_2_int_st(&self) -> PAD_COMP_CHANNEL_2_INT_ST_R {
        PAD_COMP_CHANNEL_2_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_neg_int_st(&self) -> PAD_COMP_CHANNEL_1_NEG_INT_ST_R {
        PAD_COMP_CHANNEL_1_NEG_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_pos_int_st(&self) -> PAD_COMP_CHANNEL_1_POS_INT_ST_R {
        PAD_COMP_CHANNEL_1_POS_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn pad_comp_channel_1_int_st(&self) -> PAD_COMP_CHANNEL_1_INT_ST_R {
        PAD_COMP_CHANNEL_1_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn pad_comp_neg_int_st(&self) -> PAD_COMP_NEG_INT_ST_R {
        PAD_COMP_NEG_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn pad_comp_pos_int_st(&self) -> PAD_COMP_POS_INT_ST_R {
        PAD_COMP_POS_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn pad_comp_int_st(&self) -> PAD_COMP_INT_ST_R {
        PAD_COMP_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "pad_comp_channel_3_neg_int_st",
                &self.pad_comp_channel_3_neg_int_st(),
            )
            .field(
                "pad_comp_channel_3_pos_int_st",
                &self.pad_comp_channel_3_pos_int_st(),
            )
            .field(
                "pad_comp_channel_3_int_st",
                &self.pad_comp_channel_3_int_st(),
            )
            .field(
                "pad_comp_channel_2_neg_int_st",
                &self.pad_comp_channel_2_neg_int_st(),
            )
            .field(
                "pad_comp_channel_2_pos_int_st",
                &self.pad_comp_channel_2_pos_int_st(),
            )
            .field(
                "pad_comp_channel_2_int_st",
                &self.pad_comp_channel_2_int_st(),
            )
            .field(
                "pad_comp_channel_1_neg_int_st",
                &self.pad_comp_channel_1_neg_int_st(),
            )
            .field(
                "pad_comp_channel_1_pos_int_st",
                &self.pad_comp_channel_1_pos_int_st(),
            )
            .field(
                "pad_comp_channel_1_int_st",
                &self.pad_comp_channel_1_int_st(),
            )
            .field("pad_comp_neg_int_st", &self.pad_comp_neg_int_st())
            .field("pad_comp_pos_int_st", &self.pad_comp_pos_int_st())
            .field("pad_comp_int_st", &self.pad_comp_int_st())
            .finish()
    }
}
#[doc = "zero det int st\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
