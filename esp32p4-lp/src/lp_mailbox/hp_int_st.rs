#[doc = "Register `HP_INT_ST` reader"]
pub type R = crate::R<HP_INT_ST_SPEC>;
#[doc = "Field `HP_0_INT_ST` reader - MB_HP_0_INT_ST"]
pub type HP_0_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_1_INT_ST` reader - MB_HP_1_INT_ST"]
pub type HP_1_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_2_INT_ST` reader - MB_HP_2_INT_ST"]
pub type HP_2_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_3_INT_ST` reader - MB_HP_3_INT_ST"]
pub type HP_3_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_4_INT_ST` reader - MB_HP_4_INT_ST"]
pub type HP_4_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_5_INT_ST` reader - MB_HP_5_INT_ST"]
pub type HP_5_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_6_INT_ST` reader - MB_HP_6_INT_ST"]
pub type HP_6_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_7_INT_ST` reader - MB_HP_7_INT_ST"]
pub type HP_7_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_8_INT_ST` reader - MB_HP_8_INT_ST"]
pub type HP_8_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_9_INT_ST` reader - MB_HP_9_INT_ST"]
pub type HP_9_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_10_INT_ST` reader - MB_HP_10_INT_ST"]
pub type HP_10_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_11_INT_ST` reader - MB_HP_11_INT_ST"]
pub type HP_11_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_12_INT_ST` reader - MB_HP_12_INT_ST"]
pub type HP_12_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_13_INT_ST` reader - MB_HP_13_INT_ST"]
pub type HP_13_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_14_INT_ST` reader - MB_HP_14_INT_ST"]
pub type HP_14_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_15_INT_ST` reader - MB_HP_15_INT_ST"]
pub type HP_15_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MB_HP_0_INT_ST"]
    #[inline(always)]
    pub fn hp_0_int_st(&self) -> HP_0_INT_ST_R {
        HP_0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MB_HP_1_INT_ST"]
    #[inline(always)]
    pub fn hp_1_int_st(&self) -> HP_1_INT_ST_R {
        HP_1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MB_HP_2_INT_ST"]
    #[inline(always)]
    pub fn hp_2_int_st(&self) -> HP_2_INT_ST_R {
        HP_2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MB_HP_3_INT_ST"]
    #[inline(always)]
    pub fn hp_3_int_st(&self) -> HP_3_INT_ST_R {
        HP_3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MB_HP_4_INT_ST"]
    #[inline(always)]
    pub fn hp_4_int_st(&self) -> HP_4_INT_ST_R {
        HP_4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MB_HP_5_INT_ST"]
    #[inline(always)]
    pub fn hp_5_int_st(&self) -> HP_5_INT_ST_R {
        HP_5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MB_HP_6_INT_ST"]
    #[inline(always)]
    pub fn hp_6_int_st(&self) -> HP_6_INT_ST_R {
        HP_6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MB_HP_7_INT_ST"]
    #[inline(always)]
    pub fn hp_7_int_st(&self) -> HP_7_INT_ST_R {
        HP_7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MB_HP_8_INT_ST"]
    #[inline(always)]
    pub fn hp_8_int_st(&self) -> HP_8_INT_ST_R {
        HP_8_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MB_HP_9_INT_ST"]
    #[inline(always)]
    pub fn hp_9_int_st(&self) -> HP_9_INT_ST_R {
        HP_9_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MB_HP_10_INT_ST"]
    #[inline(always)]
    pub fn hp_10_int_st(&self) -> HP_10_INT_ST_R {
        HP_10_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MB_HP_11_INT_ST"]
    #[inline(always)]
    pub fn hp_11_int_st(&self) -> HP_11_INT_ST_R {
        HP_11_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MB_HP_12_INT_ST"]
    #[inline(always)]
    pub fn hp_12_int_st(&self) -> HP_12_INT_ST_R {
        HP_12_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MB_HP_13_INT_ST"]
    #[inline(always)]
    pub fn hp_13_int_st(&self) -> HP_13_INT_ST_R {
        HP_13_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MB_HP_14_INT_ST"]
    #[inline(always)]
    pub fn hp_14_int_st(&self) -> HP_14_INT_ST_R {
        HP_14_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MB_HP_15_INT_ST"]
    #[inline(always)]
    pub fn hp_15_int_st(&self) -> HP_15_INT_ST_R {
        HP_15_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_INT_ST")
            .field("hp_0_int_st", &self.hp_0_int_st())
            .field("hp_1_int_st", &self.hp_1_int_st())
            .field("hp_2_int_st", &self.hp_2_int_st())
            .field("hp_3_int_st", &self.hp_3_int_st())
            .field("hp_4_int_st", &self.hp_4_int_st())
            .field("hp_5_int_st", &self.hp_5_int_st())
            .field("hp_6_int_st", &self.hp_6_int_st())
            .field("hp_7_int_st", &self.hp_7_int_st())
            .field("hp_8_int_st", &self.hp_8_int_st())
            .field("hp_9_int_st", &self.hp_9_int_st())
            .field("hp_10_int_st", &self.hp_10_int_st())
            .field("hp_11_int_st", &self.hp_11_int_st())
            .field("hp_12_int_st", &self.hp_12_int_st())
            .field("hp_13_int_st", &self.hp_13_int_st())
            .field("hp_14_int_st", &self.hp_14_int_st())
            .field("hp_15_int_st", &self.hp_15_int_st())
            .finish()
    }
}
#[doc = "MB_HP_INT_ST\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_INT_ST_SPEC;
impl crate::RegisterSpec for HP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_int_st::R`](R) reader structure"]
impl crate::Readable for HP_INT_ST_SPEC {}
#[doc = "`reset()` method sets HP_INT_ST to value 0"]
impl crate::Resettable for HP_INT_ST_SPEC {}
