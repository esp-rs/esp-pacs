#[doc = "Register `GEN_FORCE` reader"]
pub type R = crate::R<GEN_FORCE_SPEC>;
#[doc = "Register `GEN_FORCE` writer"]
pub type W = crate::W<GEN_FORCE_SPEC>;
#[doc = "Field `CNTUFORCE_UPMETHOD` reader - "]
pub type CNTUFORCE_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CNTUFORCE_UPMETHOD` writer - "]
pub type CNTUFORCE_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_CNTUFORCE_MODE` reader - "]
pub type A_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `A_CNTUFORCE_MODE` writer - "]
pub type A_CNTUFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `B_CNTUFORCE_MODE` reader - "]
pub type B_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `B_CNTUFORCE_MODE` writer - "]
pub type B_CNTUFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `A_NCIFORCE` reader - "]
pub type A_NCIFORCE_R = crate::BitReader;
#[doc = "Field `A_NCIFORCE` writer - "]
pub type A_NCIFORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_NCIFORCE_MODE` reader - "]
pub type A_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `A_NCIFORCE_MODE` writer - "]
pub type A_NCIFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `B_NCIFORCE` reader - "]
pub type B_NCIFORCE_R = crate::BitReader;
#[doc = "Field `B_NCIFORCE` writer - "]
pub type B_NCIFORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_NCIFORCE_MODE` reader - "]
pub type B_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `B_NCIFORCE_MODE` writer - "]
pub type B_NCIFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cntuforce_upmethod(&self) -> CNTUFORCE_UPMETHOD_R {
        CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn a_cntuforce_mode(&self) -> A_CNTUFORCE_MODE_R {
        A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn b_cntuforce_mode(&self) -> B_CNTUFORCE_MODE_R {
        B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn a_nciforce(&self) -> A_NCIFORCE_R {
        A_NCIFORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn a_nciforce_mode(&self) -> A_NCIFORCE_MODE_R {
        A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn b_nciforce(&self) -> B_NCIFORCE_R {
        B_NCIFORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn b_nciforce_mode(&self) -> B_NCIFORCE_MODE_R {
        B_NCIFORCE_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_FORCE")
            .field("cntuforce_upmethod", &self.cntuforce_upmethod())
            .field("a_cntuforce_mode", &self.a_cntuforce_mode())
            .field("b_cntuforce_mode", &self.b_cntuforce_mode())
            .field("a_nciforce", &self.a_nciforce())
            .field("a_nciforce_mode", &self.a_nciforce_mode())
            .field("b_nciforce", &self.b_nciforce())
            .field("b_nciforce_mode", &self.b_nciforce_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn cntuforce_upmethod(&mut self) -> CNTUFORCE_UPMETHOD_W<GEN_FORCE_SPEC> {
        CNTUFORCE_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn a_cntuforce_mode(&mut self) -> A_CNTUFORCE_MODE_W<GEN_FORCE_SPEC> {
        A_CNTUFORCE_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn b_cntuforce_mode(&mut self) -> B_CNTUFORCE_MODE_W<GEN_FORCE_SPEC> {
        B_CNTUFORCE_MODE_W::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn a_nciforce(&mut self) -> A_NCIFORCE_W<GEN_FORCE_SPEC> {
        A_NCIFORCE_W::new(self, 10)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn a_nciforce_mode(&mut self) -> A_NCIFORCE_MODE_W<GEN_FORCE_SPEC> {
        A_NCIFORCE_MODE_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn b_nciforce(&mut self) -> B_NCIFORCE_W<GEN_FORCE_SPEC> {
        B_NCIFORCE_W::new(self, 13)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn b_nciforce_mode(&mut self) -> B_NCIFORCE_MODE_W<GEN_FORCE_SPEC> {
        B_NCIFORCE_MODE_W::new(self, 14)
    }
}
#[doc = "Permissives to force PWMxA and PWMxB outputs by software\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_force::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_force::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_FORCE_SPEC;
impl crate::RegisterSpec for GEN_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_force::R`](R) reader structure"]
impl crate::Readable for GEN_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_force::W`](W) writer structure"]
impl crate::Writable for GEN_FORCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_FORCE to value 0x20"]
impl crate::Resettable for GEN_FORCE_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
