#[doc = "Register `GEN_PLD_DATA` reader"]
pub type R = crate::R<GEN_PLD_DATA_SPEC>;
#[doc = "Register `GEN_PLD_DATA` writer"]
pub type W = crate::W<GEN_PLD_DATA_SPEC>;
#[doc = "Field `GEN_PLD_B1` reader - NA"]
pub type GEN_PLD_B1_R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B1` writer - NA"]
pub type GEN_PLD_B1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_PLD_B2` reader - NA"]
pub type GEN_PLD_B2_R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B2` writer - NA"]
pub type GEN_PLD_B2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_PLD_B3` reader - NA"]
pub type GEN_PLD_B3_R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B3` writer - NA"]
pub type GEN_PLD_B3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GEN_PLD_B4` reader - NA"]
pub type GEN_PLD_B4_R = crate::FieldReader;
#[doc = "Field `GEN_PLD_B4` writer - NA"]
pub type GEN_PLD_B4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn gen_pld_b1(&self) -> GEN_PLD_B1_R {
        GEN_PLD_B1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn gen_pld_b2(&self) -> GEN_PLD_B2_R {
        GEN_PLD_B2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn gen_pld_b3(&self) -> GEN_PLD_B3_R {
        GEN_PLD_B3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    pub fn gen_pld_b4(&self) -> GEN_PLD_B4_R {
        GEN_PLD_B4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_PLD_DATA")
            .field("gen_pld_b1", &format_args!("{}", self.gen_pld_b1().bits()))
            .field("gen_pld_b2", &format_args!("{}", self.gen_pld_b2().bits()))
            .field("gen_pld_b3", &format_args!("{}", self.gen_pld_b3().bits()))
            .field("gen_pld_b4", &format_args!("{}", self.gen_pld_b4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN_PLD_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b1(&mut self) -> GEN_PLD_B1_W<GEN_PLD_DATA_SPEC> {
        GEN_PLD_B1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b2(&mut self) -> GEN_PLD_B2_W<GEN_PLD_DATA_SPEC> {
        GEN_PLD_B2_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b3(&mut self) -> GEN_PLD_B3_W<GEN_PLD_DATA_SPEC> {
        GEN_PLD_B3_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_pld_b4(&mut self) -> GEN_PLD_B4_W<GEN_PLD_DATA_SPEC> {
        GEN_PLD_B4_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_pld_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_pld_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_PLD_DATA_SPEC;
impl crate::RegisterSpec for GEN_PLD_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_pld_data::R`](R) reader structure"]
impl crate::Readable for GEN_PLD_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_pld_data::W`](W) writer structure"]
impl crate::Writable for GEN_PLD_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_PLD_DATA to value 0"]
impl crate::Resettable for GEN_PLD_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
