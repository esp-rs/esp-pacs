#[doc = "Register `GEN1_FORCE` reader"]
pub type R = crate::R<GEN1_FORCE_SPEC>;
#[doc = "Register `GEN1_FORCE` writer"]
pub type W = crate::W<GEN1_FORCE_SPEC>;
#[doc = "Field `GEN1_CNTUFORCE_UPMETHOD` reader - "]
pub type GEN1_CNTUFORCE_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN1_CNTUFORCE_UPMETHOD` writer - "]
pub type GEN1_CNTUFORCE_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `GEN1_A_CNTUFORCE_MODE` reader - "]
pub type GEN1_A_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN1_A_CNTUFORCE_MODE` writer - "]
pub type GEN1_A_CNTUFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN1_B_CNTUFORCE_MODE` reader - "]
pub type GEN1_B_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN1_B_CNTUFORCE_MODE` writer - "]
pub type GEN1_B_CNTUFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN1_A_NCIFORCE` reader - "]
pub type GEN1_A_NCIFORCE_R = crate::BitReader;
#[doc = "Field `GEN1_A_NCIFORCE` writer - "]
pub type GEN1_A_NCIFORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN1_A_NCIFORCE_MODE` reader - "]
pub type GEN1_A_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN1_A_NCIFORCE_MODE` writer - "]
pub type GEN1_A_NCIFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN1_B_NCIFORCE` reader - "]
pub type GEN1_B_NCIFORCE_R = crate::BitReader;
#[doc = "Field `GEN1_B_NCIFORCE` writer - "]
pub type GEN1_B_NCIFORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN1_B_NCIFORCE_MODE` reader - "]
pub type GEN1_B_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN1_B_NCIFORCE_MODE` writer - "]
pub type GEN1_B_NCIFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gen1_cntuforce_upmethod(&self) -> GEN1_CNTUFORCE_UPMETHOD_R {
        GEN1_CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn gen1_a_cntuforce_mode(&self) -> GEN1_A_CNTUFORCE_MODE_R {
        GEN1_A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn gen1_b_cntuforce_mode(&self) -> GEN1_B_CNTUFORCE_MODE_R {
        GEN1_B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gen1_a_nciforce(&self) -> GEN1_A_NCIFORCE_R {
        GEN1_A_NCIFORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn gen1_a_nciforce_mode(&self) -> GEN1_A_NCIFORCE_MODE_R {
        GEN1_A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gen1_b_nciforce(&self) -> GEN1_B_NCIFORCE_R {
        GEN1_B_NCIFORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn gen1_b_nciforce_mode(&self) -> GEN1_B_NCIFORCE_MODE_R {
        GEN1_B_NCIFORCE_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN1_FORCE")
            .field(
                "gen1_cntuforce_upmethod",
                &format_args!("{}", self.gen1_cntuforce_upmethod().bits()),
            )
            .field(
                "gen1_a_cntuforce_mode",
                &format_args!("{}", self.gen1_a_cntuforce_mode().bits()),
            )
            .field(
                "gen1_b_cntuforce_mode",
                &format_args!("{}", self.gen1_b_cntuforce_mode().bits()),
            )
            .field(
                "gen1_a_nciforce",
                &format_args!("{}", self.gen1_a_nciforce().bit()),
            )
            .field(
                "gen1_a_nciforce_mode",
                &format_args!("{}", self.gen1_a_nciforce_mode().bits()),
            )
            .field(
                "gen1_b_nciforce",
                &format_args!("{}", self.gen1_b_nciforce().bit()),
            )
            .field(
                "gen1_b_nciforce_mode",
                &format_args!("{}", self.gen1_b_nciforce_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN1_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_cntuforce_upmethod(&mut self) -> GEN1_CNTUFORCE_UPMETHOD_W<GEN1_FORCE_SPEC> {
        GEN1_CNTUFORCE_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_a_cntuforce_mode(&mut self) -> GEN1_A_CNTUFORCE_MODE_W<GEN1_FORCE_SPEC> {
        GEN1_A_CNTUFORCE_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_b_cntuforce_mode(&mut self) -> GEN1_B_CNTUFORCE_MODE_W<GEN1_FORCE_SPEC> {
        GEN1_B_CNTUFORCE_MODE_W::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_a_nciforce(&mut self) -> GEN1_A_NCIFORCE_W<GEN1_FORCE_SPEC> {
        GEN1_A_NCIFORCE_W::new(self, 10)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_a_nciforce_mode(&mut self) -> GEN1_A_NCIFORCE_MODE_W<GEN1_FORCE_SPEC> {
        GEN1_A_NCIFORCE_MODE_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_b_nciforce(&mut self) -> GEN1_B_NCIFORCE_W<GEN1_FORCE_SPEC> {
        GEN1_B_NCIFORCE_W::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_b_nciforce_mode(&mut self) -> GEN1_B_NCIFORCE_MODE_W<GEN1_FORCE_SPEC> {
        GEN1_B_NCIFORCE_MODE_W::new(self, 14)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_force::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_force::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN1_FORCE_SPEC;
impl crate::RegisterSpec for GEN1_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen1_force::R`](R) reader structure"]
impl crate::Readable for GEN1_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen1_force::W`](W) writer structure"]
impl crate::Writable for GEN1_FORCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN1_FORCE to value 0x20"]
impl crate::Resettable for GEN1_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
