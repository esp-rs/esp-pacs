#[doc = "Register `GEN_FORCE` reader"]
pub type R = crate::R<GEN_FORCE_SPEC>;
#[doc = "Register `GEN_FORCE` writer"]
pub type W = crate::W<GEN_FORCE_SPEC>;
#[doc = "Field `CNTUFORCE_UPMETHOD` reader - Configures update method for continuous software force of PWM generator%s.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: TEA\\\\Bit3 is set to 1: TEB\\\\Bit4 is set to 1: Sync\\\\Bit5 is set to 1: Disable update. TEA/B here and below means an event generated when the timer's value equals to that of register A/B."]
pub type CNTUFORCE_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CNTUFORCE_UPMETHOD` writer - Configures update method for continuous software force of PWM generator%s.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: TEA\\\\Bit3 is set to 1: TEB\\\\Bit4 is set to 1: Sync\\\\Bit5 is set to 1: Disable update. TEA/B here and below means an event generated when the timer's value equals to that of register A/B."]
pub type CNTUFORCE_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_CNTUFORCE_MODE` reader - Configures continuous software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type A_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `A_CNTUFORCE_MODE` writer - Configures continuous software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type A_CNTUFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `B_CNTUFORCE_MODE` reader - Configures continuous software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type B_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `B_CNTUFORCE_MODE` writer - Configures continuous software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type B_CNTUFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `A_NCIFORCE` reader - Configures the generation of non-continuous immediate software-force event for PWM%s A, a toggle will trigger a force event."]
pub type A_NCIFORCE_R = crate::BitReader;
#[doc = "Field `A_NCIFORCE` writer - Configures the generation of non-continuous immediate software-force event for PWM%s A, a toggle will trigger a force event."]
pub type A_NCIFORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_NCIFORCE_MODE` reader - Configures non-continuous immediate software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type A_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `A_NCIFORCE_MODE` writer - Configures non-continuous immediate software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type A_NCIFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `B_NCIFORCE` reader - Configures the generation of non-continuous immediate software-force event for PWM%s B, a toggle will trigger a force event."]
pub type B_NCIFORCE_R = crate::BitReader;
#[doc = "Field `B_NCIFORCE` writer - Configures the generation of non-continuous immediate software-force event for PWM%s B, a toggle will trigger a force event."]
pub type B_NCIFORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_NCIFORCE_MODE` reader - Configures non-continuous immediate software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type B_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `B_NCIFORCE_MODE` writer - Configures non-continuous immediate software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
pub type B_NCIFORCE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - Configures update method for continuous software force of PWM generator%s.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: TEA\\\\Bit3 is set to 1: TEB\\\\Bit4 is set to 1: Sync\\\\Bit5 is set to 1: Disable update. TEA/B here and below means an event generated when the timer's value equals to that of register A/B."]
    #[inline(always)]
    pub fn cntuforce_upmethod(&self) -> CNTUFORCE_UPMETHOD_R {
        CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Configures continuous software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn a_cntuforce_mode(&self) -> A_CNTUFORCE_MODE_R {
        A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Configures continuous software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn b_cntuforce_mode(&self) -> B_CNTUFORCE_MODE_R {
        B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Configures the generation of non-continuous immediate software-force event for PWM%s A, a toggle will trigger a force event."]
    #[inline(always)]
    pub fn a_nciforce(&self) -> A_NCIFORCE_R {
        A_NCIFORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Configures non-continuous immediate software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn a_nciforce_mode(&self) -> A_NCIFORCE_MODE_R {
        A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Configures the generation of non-continuous immediate software-force event for PWM%s B, a toggle will trigger a force event."]
    #[inline(always)]
    pub fn b_nciforce(&self) -> B_NCIFORCE_R {
        B_NCIFORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Configures non-continuous immediate software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
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
    #[doc = "Bits 0:5 - Configures update method for continuous software force of PWM generator%s.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: TEA\\\\Bit3 is set to 1: TEB\\\\Bit4 is set to 1: Sync\\\\Bit5 is set to 1: Disable update. TEA/B here and below means an event generated when the timer's value equals to that of register A/B."]
    #[inline(always)]
    pub fn cntuforce_upmethod(&mut self) -> CNTUFORCE_UPMETHOD_W<GEN_FORCE_SPEC> {
        CNTUFORCE_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Configures continuous software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn a_cntuforce_mode(&mut self) -> A_CNTUFORCE_MODE_W<GEN_FORCE_SPEC> {
        A_CNTUFORCE_MODE_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configures continuous software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn b_cntuforce_mode(&mut self) -> B_CNTUFORCE_MODE_W<GEN_FORCE_SPEC> {
        B_CNTUFORCE_MODE_W::new(self, 8)
    }
    #[doc = "Bit 10 - Configures the generation of non-continuous immediate software-force event for PWM%s A, a toggle will trigger a force event."]
    #[inline(always)]
    pub fn a_nciforce(&mut self) -> A_NCIFORCE_W<GEN_FORCE_SPEC> {
        A_NCIFORCE_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Configures non-continuous immediate software force mode for PWM%s A.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn a_nciforce_mode(&mut self) -> A_NCIFORCE_MODE_W<GEN_FORCE_SPEC> {
        A_NCIFORCE_MODE_W::new(self, 11)
    }
    #[doc = "Bit 13 - Configures the generation of non-continuous immediate software-force event for PWM%s B, a toggle will trigger a force event."]
    #[inline(always)]
    pub fn b_nciforce(&mut self) -> B_NCIFORCE_W<GEN_FORCE_SPEC> {
        B_NCIFORCE_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Configures non-continuous immediate software force mode for PWM%s B.\\\\0: Disabled\\\\1: Low\\\\2: High\\\\3: Disabled"]
    #[inline(always)]
    pub fn b_nciforce_mode(&mut self) -> B_NCIFORCE_MODE_W<GEN_FORCE_SPEC> {
        B_NCIFORCE_MODE_W::new(self, 14)
    }
}
#[doc = "Generator0 output signal force mode register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
