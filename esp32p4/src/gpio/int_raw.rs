///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `COMP0_NEG` reader - analog comparator pos edge interrupt raw
pub type COMP0_NEG_R = crate::BitReader;
///Field `COMP0_NEG` writer - analog comparator pos edge interrupt raw
pub type COMP0_NEG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP0_POS` reader - analog comparator neg edge interrupt raw
pub type COMP0_POS_R = crate::BitReader;
///Field `COMP0_POS` writer - analog comparator neg edge interrupt raw
pub type COMP0_POS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP0_ALL` reader - analog comparator neg or pos edge interrupt raw
pub type COMP0_ALL_R = crate::BitReader;
///Field `COMP0_ALL` writer - analog comparator neg or pos edge interrupt raw
pub type COMP0_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_NEG` reader - analog comparator pos edge interrupt raw
pub type COMP1_NEG_R = crate::BitReader;
///Field `COMP1_NEG` writer - analog comparator pos edge interrupt raw
pub type COMP1_NEG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_POS` reader - analog comparator neg edge interrupt raw
pub type COMP1_POS_R = crate::BitReader;
///Field `COMP1_POS` writer - analog comparator neg edge interrupt raw
pub type COMP1_POS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMP1_ALL` reader - analog comparator neg or pos edge interrupt raw
pub type COMP1_ALL_R = crate::BitReader;
///Field `COMP1_ALL` writer - analog comparator neg or pos edge interrupt raw
pub type COMP1_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BISTOK` reader - pad bistok interrupt raw
pub type BISTOK_R = crate::BitReader;
///Field `BISTOK` writer - pad bistok interrupt raw
pub type BISTOK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BISTFAIL` reader - pad bistfail interrupt raw
pub type BISTFAIL_R = crate::BitReader;
///Field `BISTFAIL` writer - pad bistfail interrupt raw
pub type BISTFAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - analog comparator pos edge interrupt raw
    #[inline(always)]
    pub fn comp0_neg(&self) -> COMP0_NEG_R {
        COMP0_NEG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - analog comparator neg edge interrupt raw
    #[inline(always)]
    pub fn comp0_pos(&self) -> COMP0_POS_R {
        COMP0_POS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - analog comparator neg or pos edge interrupt raw
    #[inline(always)]
    pub fn comp0_all(&self) -> COMP0_ALL_R {
        COMP0_ALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - analog comparator pos edge interrupt raw
    #[inline(always)]
    pub fn comp1_neg(&self) -> COMP1_NEG_R {
        COMP1_NEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - analog comparator neg edge interrupt raw
    #[inline(always)]
    pub fn comp1_pos(&self) -> COMP1_POS_R {
        COMP1_POS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - analog comparator neg or pos edge interrupt raw
    #[inline(always)]
    pub fn comp1_all(&self) -> COMP1_ALL_R {
        COMP1_ALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - pad bistok interrupt raw
    #[inline(always)]
    pub fn bistok(&self) -> BISTOK_R {
        BISTOK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - pad bistfail interrupt raw
    #[inline(always)]
    pub fn bistfail(&self) -> BISTFAIL_R {
        BISTFAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("comp0_neg", &self.comp0_neg())
            .field("comp0_pos", &self.comp0_pos())
            .field("comp0_all", &self.comp0_all())
            .field("comp1_neg", &self.comp1_neg())
            .field("comp1_pos", &self.comp1_pos())
            .field("comp1_all", &self.comp1_all())
            .field("bistok", &self.bistok())
            .field("bistfail", &self.bistfail())
            .finish()
    }
}
impl W {
    ///Bit 0 - analog comparator pos edge interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn comp0_neg(&mut self) -> COMP0_NEG_W<INT_RAW_SPEC> {
        COMP0_NEG_W::new(self, 0)
    }
    ///Bit 1 - analog comparator neg edge interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn comp0_pos(&mut self) -> COMP0_POS_W<INT_RAW_SPEC> {
        COMP0_POS_W::new(self, 1)
    }
    ///Bit 2 - analog comparator neg or pos edge interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn comp0_all(&mut self) -> COMP0_ALL_W<INT_RAW_SPEC> {
        COMP0_ALL_W::new(self, 2)
    }
    ///Bit 3 - analog comparator pos edge interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn comp1_neg(&mut self) -> COMP1_NEG_W<INT_RAW_SPEC> {
        COMP1_NEG_W::new(self, 3)
    }
    ///Bit 4 - analog comparator neg edge interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn comp1_pos(&mut self) -> COMP1_POS_W<INT_RAW_SPEC> {
        COMP1_POS_W::new(self, 4)
    }
    ///Bit 5 - analog comparator neg or pos edge interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn comp1_all(&mut self) -> COMP1_ALL_W<INT_RAW_SPEC> {
        COMP1_ALL_W::new(self, 5)
    }
    ///Bit 6 - pad bistok interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn bistok(&mut self) -> BISTOK_W<INT_RAW_SPEC> {
        BISTOK_W::new(self, 6)
    }
    ///Bit 7 - pad bistfail interrupt raw
    #[inline(always)]
    #[must_use]
    pub fn bistfail(&mut self) -> BISTFAIL_W<INT_RAW_SPEC> {
        BISTFAIL_W::new(self, 7)
    }
}
/**analog comparator interrupt raw

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`int_raw::W`](W) writer structure
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
