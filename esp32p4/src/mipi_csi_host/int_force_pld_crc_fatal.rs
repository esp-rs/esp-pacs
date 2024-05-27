#[doc = "Register `INT_FORCE_PLD_CRC_FATAL` reader"]
pub type R = crate::R<INT_FORCE_PLD_CRC_FATAL_SPEC>;
#[doc = "Register `INT_FORCE_PLD_CRC_FATAL` writer"]
pub type W = crate::W<INT_FORCE_PLD_CRC_FATAL_SPEC>;
#[doc = "Field `FORCE_ERR_CRC_VC0` reader - NA"]
pub type FORCE_ERR_CRC_VC0_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC0` writer - NA"]
pub type FORCE_ERR_CRC_VC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC1` reader - NA"]
pub type FORCE_ERR_CRC_VC1_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC1` writer - NA"]
pub type FORCE_ERR_CRC_VC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC2` reader - NA"]
pub type FORCE_ERR_CRC_VC2_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC2` writer - NA"]
pub type FORCE_ERR_CRC_VC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC3` reader - NA"]
pub type FORCE_ERR_CRC_VC3_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC3` writer - NA"]
pub type FORCE_ERR_CRC_VC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC4` reader - NA"]
pub type FORCE_ERR_CRC_VC4_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC4` writer - NA"]
pub type FORCE_ERR_CRC_VC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC5` reader - NA"]
pub type FORCE_ERR_CRC_VC5_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC5` writer - NA"]
pub type FORCE_ERR_CRC_VC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC6` reader - NA"]
pub type FORCE_ERR_CRC_VC6_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC6` writer - NA"]
pub type FORCE_ERR_CRC_VC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC7` reader - NA"]
pub type FORCE_ERR_CRC_VC7_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC7` writer - NA"]
pub type FORCE_ERR_CRC_VC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC8` reader - NA"]
pub type FORCE_ERR_CRC_VC8_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC8` writer - NA"]
pub type FORCE_ERR_CRC_VC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC9` reader - NA"]
pub type FORCE_ERR_CRC_VC9_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC9` writer - NA"]
pub type FORCE_ERR_CRC_VC9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC10` reader - NA"]
pub type FORCE_ERR_CRC_VC10_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC10` writer - NA"]
pub type FORCE_ERR_CRC_VC10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC11` reader - NA"]
pub type FORCE_ERR_CRC_VC11_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC11` writer - NA"]
pub type FORCE_ERR_CRC_VC11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC12` reader - NA"]
pub type FORCE_ERR_CRC_VC12_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC12` writer - NA"]
pub type FORCE_ERR_CRC_VC12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC13` reader - NA"]
pub type FORCE_ERR_CRC_VC13_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC13` writer - NA"]
pub type FORCE_ERR_CRC_VC13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC14` reader - NA"]
pub type FORCE_ERR_CRC_VC14_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC14` writer - NA"]
pub type FORCE_ERR_CRC_VC14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ERR_CRC_VC15` reader - NA"]
pub type FORCE_ERR_CRC_VC15_R = crate::BitReader;
#[doc = "Field `FORCE_ERR_CRC_VC15` writer - NA"]
pub type FORCE_ERR_CRC_VC15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc0(&self) -> FORCE_ERR_CRC_VC0_R {
        FORCE_ERR_CRC_VC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc1(&self) -> FORCE_ERR_CRC_VC1_R {
        FORCE_ERR_CRC_VC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc2(&self) -> FORCE_ERR_CRC_VC2_R {
        FORCE_ERR_CRC_VC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc3(&self) -> FORCE_ERR_CRC_VC3_R {
        FORCE_ERR_CRC_VC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc4(&self) -> FORCE_ERR_CRC_VC4_R {
        FORCE_ERR_CRC_VC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc5(&self) -> FORCE_ERR_CRC_VC5_R {
        FORCE_ERR_CRC_VC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc6(&self) -> FORCE_ERR_CRC_VC6_R {
        FORCE_ERR_CRC_VC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc7(&self) -> FORCE_ERR_CRC_VC7_R {
        FORCE_ERR_CRC_VC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc8(&self) -> FORCE_ERR_CRC_VC8_R {
        FORCE_ERR_CRC_VC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc9(&self) -> FORCE_ERR_CRC_VC9_R {
        FORCE_ERR_CRC_VC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc10(&self) -> FORCE_ERR_CRC_VC10_R {
        FORCE_ERR_CRC_VC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc11(&self) -> FORCE_ERR_CRC_VC11_R {
        FORCE_ERR_CRC_VC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc12(&self) -> FORCE_ERR_CRC_VC12_R {
        FORCE_ERR_CRC_VC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc13(&self) -> FORCE_ERR_CRC_VC13_R {
        FORCE_ERR_CRC_VC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc14(&self) -> FORCE_ERR_CRC_VC14_R {
        FORCE_ERR_CRC_VC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn force_err_crc_vc15(&self) -> FORCE_ERR_CRC_VC15_R {
        FORCE_ERR_CRC_VC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_FORCE_PLD_CRC_FATAL")
            .field("force_err_crc_vc0", &self.force_err_crc_vc0())
            .field("force_err_crc_vc1", &self.force_err_crc_vc1())
            .field("force_err_crc_vc2", &self.force_err_crc_vc2())
            .field("force_err_crc_vc3", &self.force_err_crc_vc3())
            .field("force_err_crc_vc4", &self.force_err_crc_vc4())
            .field("force_err_crc_vc5", &self.force_err_crc_vc5())
            .field("force_err_crc_vc6", &self.force_err_crc_vc6())
            .field("force_err_crc_vc7", &self.force_err_crc_vc7())
            .field("force_err_crc_vc8", &self.force_err_crc_vc8())
            .field("force_err_crc_vc9", &self.force_err_crc_vc9())
            .field("force_err_crc_vc10", &self.force_err_crc_vc10())
            .field("force_err_crc_vc11", &self.force_err_crc_vc11())
            .field("force_err_crc_vc12", &self.force_err_crc_vc12())
            .field("force_err_crc_vc13", &self.force_err_crc_vc13())
            .field("force_err_crc_vc14", &self.force_err_crc_vc14())
            .field("force_err_crc_vc15", &self.force_err_crc_vc15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc0(&mut self) -> FORCE_ERR_CRC_VC0_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc1(&mut self) -> FORCE_ERR_CRC_VC1_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc2(&mut self) -> FORCE_ERR_CRC_VC2_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc3(&mut self) -> FORCE_ERR_CRC_VC3_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc4(&mut self) -> FORCE_ERR_CRC_VC4_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc5(&mut self) -> FORCE_ERR_CRC_VC5_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc6(&mut self) -> FORCE_ERR_CRC_VC6_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc7(&mut self) -> FORCE_ERR_CRC_VC7_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc8(&mut self) -> FORCE_ERR_CRC_VC8_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc9(&mut self) -> FORCE_ERR_CRC_VC9_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc10(&mut self) -> FORCE_ERR_CRC_VC10_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC10_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc11(&mut self) -> FORCE_ERR_CRC_VC11_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC11_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc12(&mut self) -> FORCE_ERR_CRC_VC12_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC12_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc13(&mut self) -> FORCE_ERR_CRC_VC13_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC13_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc14(&mut self) -> FORCE_ERR_CRC_VC14_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC14_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_err_crc_vc15(&mut self) -> FORCE_ERR_CRC_VC15_W<INT_FORCE_PLD_CRC_FATAL_SPEC> {
        FORCE_ERR_CRC_VC15_W::new(self, 15)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_pld_crc_fatal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_pld_crc_fatal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FORCE_PLD_CRC_FATAL_SPEC;
impl crate::RegisterSpec for INT_FORCE_PLD_CRC_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_pld_crc_fatal::R`](R) reader structure"]
impl crate::Readable for INT_FORCE_PLD_CRC_FATAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_force_pld_crc_fatal::W`](W) writer structure"]
impl crate::Writable for INT_FORCE_PLD_CRC_FATAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FORCE_PLD_CRC_FATAL to value 0"]
impl crate::Resettable for INT_FORCE_PLD_CRC_FATAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
