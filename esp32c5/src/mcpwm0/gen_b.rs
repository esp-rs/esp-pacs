#[doc = "Register `GEN%s_B` reader"]
pub type R = crate::R<GEN_B_SPEC>;
#[doc = "Register `GEN%s_B` writer"]
pub type W = crate::W<GEN_B_SPEC>;
#[doc = "Field `UTEZ` reader - Configures action on PWM%s B triggered by event TEZ when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEZ_R = crate::FieldReader;
#[doc = "Field `UTEZ` writer - Configures action on PWM%s B triggered by event TEZ when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTEP` reader - Configures action on PWM%s B triggered by event TEP when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEP_R = crate::FieldReader;
#[doc = "Field `UTEP` writer - Configures action on PWM%s B triggered by event TEP when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTEA` reader - Configures action on PWM%s B triggered by event TEA when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEA_R = crate::FieldReader;
#[doc = "Field `UTEA` writer - Configures action on PWM%s B triggered by event TEA when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTEB` reader - Configures action on PWM%s B triggered by event TEB when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEB_R = crate::FieldReader;
#[doc = "Field `UTEB` writer - Configures action on PWM%s B triggered by event TEB when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UTEB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UT0` reader - Configures action on PWM%s B triggered by event_t0 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UT0_R = crate::FieldReader;
#[doc = "Field `UT0` writer - Configures action on PWM%s B triggered by event_t0 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UT1` reader - Configures action on PWM%s B triggered by event_t1 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UT1_R = crate::FieldReader;
#[doc = "Field `UT1` writer - Configures action on PWM%s B triggered by event_t1 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type UT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEZ` reader - Configures action on PWM%s B triggered by event TEZ when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEZ_R = crate::FieldReader;
#[doc = "Field `DTEZ` writer - Configures action on PWM%s B triggered by event TEZ when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEP` reader - Configures action on PWM%s B triggered by event TEP when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEP_R = crate::FieldReader;
#[doc = "Field `DTEP` writer - Configures action on PWM%s B triggered by event TEP when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEA` reader - Configures action on PWM%s B triggered by event TEA when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEA_R = crate::FieldReader;
#[doc = "Field `DTEA` writer - Configures action on PWM%s B triggered by event TEA when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEB` reader - Configures action on PWM%s B triggered by event TEB when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEB_R = crate::FieldReader;
#[doc = "Field `DTEB` writer - Configures action on PWM%s B triggered by event TEB when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DTEB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DT0` reader - Configures action on PWM%s B triggered by event_t0 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DT0_R = crate::FieldReader;
#[doc = "Field `DT0` writer - Configures action on PWM%s B triggered by event_t0 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DT1` reader - Configures action on PWM%s B triggered by event_t1 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DT1_R = crate::FieldReader;
#[doc = "Field `DT1` writer - Configures action on PWM%s B triggered by event_t1 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
pub type DT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Configures action on PWM%s B triggered by event TEZ when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configures action on PWM%s B triggered by event TEP when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configures action on PWM%s B triggered by event TEA when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configures action on PWM%s B triggered by event TEB when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Configures action on PWM%s B triggered by event_t0 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configures action on PWM%s B triggered by event_t1 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Configures action on PWM%s B triggered by event TEZ when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Configures action on PWM%s B triggered by event TEP when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Configures action on PWM%s B triggered by event TEA when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Configures action on PWM%s B triggered by event TEB when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Configures action on PWM%s B triggered by event_t0 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Configures action on PWM%s B triggered by event_t1 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_B")
            .field("utez", &self.utez())
            .field("utep", &self.utep())
            .field("utea", &self.utea())
            .field("uteb", &self.uteb())
            .field("ut0", &self.ut0())
            .field("ut1", &self.ut1())
            .field("dtez", &self.dtez())
            .field("dtep", &self.dtep())
            .field("dtea", &self.dtea())
            .field("dteb", &self.dteb())
            .field("dt0", &self.dt0())
            .field("dt1", &self.dt1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures action on PWM%s B triggered by event TEZ when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn utez(&mut self) -> UTEZ_W<GEN_B_SPEC> {
        UTEZ_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configures action on PWM%s B triggered by event TEP when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn utep(&mut self) -> UTEP_W<GEN_B_SPEC> {
        UTEP_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures action on PWM%s B triggered by event TEA when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn utea(&mut self) -> UTEA_W<GEN_B_SPEC> {
        UTEA_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configures action on PWM%s B triggered by event TEB when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn uteb(&mut self) -> UTEB_W<GEN_B_SPEC> {
        UTEB_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configures action on PWM%s B triggered by event_t0 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn ut0(&mut self) -> UT0_W<GEN_B_SPEC> {
        UT0_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Configures action on PWM%s B triggered by event_t1 when timer increasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W<GEN_B_SPEC> {
        UT1_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Configures action on PWM%s B triggered by event TEZ when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dtez(&mut self) -> DTEZ_W<GEN_B_SPEC> {
        DTEZ_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Configures action on PWM%s B triggered by event TEP when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dtep(&mut self) -> DTEP_W<GEN_B_SPEC> {
        DTEP_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Configures action on PWM%s B triggered by event TEA when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dtea(&mut self) -> DTEA_W<GEN_B_SPEC> {
        DTEA_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Configures action on PWM%s B triggered by event TEB when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dteb(&mut self) -> DTEB_W<GEN_B_SPEC> {
        DTEB_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Configures action on PWM%s B triggered by event_t0 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W<GEN_B_SPEC> {
        DT0_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Configures action on PWM%s B triggered by event_t1 when timer decreasing.\\\\0: No change\\\\1: Low\\\\2: High\\\\3: Toggle"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W<GEN_B_SPEC> {
        DT1_W::new(self, 22)
    }
}
#[doc = "PWM%s output signal B actions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_B_SPEC;
impl crate::RegisterSpec for GEN_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_b::R`](R) reader structure"]
impl crate::Readable for GEN_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_b::W`](W) writer structure"]
impl crate::Writable for GEN_B_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN%s_B to value 0"]
impl crate::Resettable for GEN_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
