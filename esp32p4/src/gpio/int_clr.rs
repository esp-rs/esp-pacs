#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `COMP0_NEG` writer - analog comparator pos edge interrupt clear"]
pub type COMP0_NEG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP0_POS` writer - analog comparator neg edge interrupt clear"]
pub type COMP0_POS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP0_ALL` writer - analog comparator neg or pos edge interrupt clear"]
pub type COMP0_ALL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1_NEG` writer - analog comparator pos edge interrupt clear"]
pub type COMP1_NEG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1_POS` writer - analog comparator neg edge interrupt clear"]
pub type COMP1_POS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `COMP1_ALL` writer - analog comparator neg or pos edge interrupt clear"]
pub type COMP1_ALL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BISTOK` writer - pad bistok interrupt enable"]
pub type BISTOK_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BISTFAIL` writer - pad bistfail interrupt enable"]
pub type BISTFAIL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp0_neg(&mut self) -> COMP0_NEG_W<INT_CLR_SPEC> {
        COMP0_NEG_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn comp0_pos(&mut self) -> COMP0_POS_W<INT_CLR_SPEC> {
        COMP0_POS_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp0_all(&mut self) -> COMP0_ALL_W<INT_CLR_SPEC> {
        COMP0_ALL_W::new(self, 2)
    }
    #[doc = "Bit 3 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp1_neg(&mut self) -> COMP1_NEG_W<INT_CLR_SPEC> {
        COMP1_NEG_W::new(self, 3)
    }
    #[doc = "Bit 4 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn comp1_pos(&mut self) -> COMP1_POS_W<INT_CLR_SPEC> {
        COMP1_POS_W::new(self, 4)
    }
    #[doc = "Bit 5 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn comp1_all(&mut self) -> COMP1_ALL_W<INT_CLR_SPEC> {
        COMP1_ALL_W::new(self, 5)
    }
    #[doc = "Bit 6 - pad bistok interrupt enable"]
    #[inline(always)]
    pub fn bistok(&mut self) -> BISTOK_W<INT_CLR_SPEC> {
        BISTOK_W::new(self, 6)
    }
    #[doc = "Bit 7 - pad bistfail interrupt enable"]
    #[inline(always)]
    pub fn bistfail(&mut self) -> BISTFAIL_W<INT_CLR_SPEC> {
        BISTFAIL_W::new(self, 7)
    }
}
#[doc = "analog comparator interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
