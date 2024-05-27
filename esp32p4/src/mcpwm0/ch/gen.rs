///Register `GEN%s` reader
pub type R = crate::R<GEN_SPEC>;
///Register `GEN%s` writer
pub type W = crate::W<GEN_SPEC>;
///Field `UTEZ` reader - Configures action on PWM%s A triggered by event TEZ when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEZ_R = crate::FieldReader;
///Field `UTEZ` writer - Configures action on PWM%s A triggered by event TEZ when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UTEP` reader - Configures action on PWM%s A triggered by event TEP when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEP_R = crate::FieldReader;
///Field `UTEP` writer - Configures action on PWM%s A triggered by event TEP when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UTEA` reader - Configures action on PWM%s A triggered by event TEA when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEA_R = crate::FieldReader;
///Field `UTEA` writer - Configures action on PWM%s A triggered by event TEA when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UTEB` reader - Configures action on PWM%s A triggered by event TEB when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEB_R = crate::FieldReader;
///Field `UTEB` writer - Configures action on PWM%s A triggered by event TEB when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UTEB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UT0` reader - Configures action on PWM%s A triggered by event_t0 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UT0_R = crate::FieldReader;
///Field `UT0` writer - Configures action on PWM%s A triggered by event_t0 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UT1` reader - Configures action on PWM%s A triggered by event_t1 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UT1_R = crate::FieldReader;
///Field `UT1` writer - Configures action on PWM%s A triggered by event_t1 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type UT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTEZ` reader - Configures action on PWM%s A triggered by event TEZ when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEZ_R = crate::FieldReader;
///Field `DTEZ` writer - Configures action on PWM%s A triggered by event TEZ when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTEP` reader - Configures action on PWM%s A triggered by event TEP when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEP_R = crate::FieldReader;
///Field `DTEP` writer - Configures action on PWM%s A triggered by event TEP when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTEA` reader - Configures action on PWM%s A triggered by event TEA when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEA_R = crate::FieldReader;
///Field `DTEA` writer - Configures action on PWM%s A triggered by event TEA when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTEB` reader - Configures action on PWM%s A triggered by event TEB when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEB_R = crate::FieldReader;
///Field `DTEB` writer - Configures action on PWM%s A triggered by event TEB when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DTEB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DT0` reader - Configures action on PWM%s A triggered by event_t0 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DT0_R = crate::FieldReader;
///Field `DT0` writer - Configures action on PWM%s A triggered by event_t0 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DT1` reader - Configures action on PWM%s A triggered by event_t1 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DT1_R = crate::FieldReader;
///Field `DT1` writer - Configures action on PWM%s A triggered by event_t1 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
pub type DT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Configures action on PWM%s A triggered by event TEZ when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Configures action on PWM%s A triggered by event TEP when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Configures action on PWM%s A triggered by event TEA when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Configures action on PWM%s A triggered by event TEB when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Configures action on PWM%s A triggered by event_t0 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Configures action on PWM%s A triggered by event_t1 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Configures action on PWM%s A triggered by event TEZ when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Configures action on PWM%s A triggered by event TEP when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Configures action on PWM%s A triggered by event TEA when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Configures action on PWM%s A triggered by event TEB when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Configures action on PWM%s A triggered by event_t0 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Configures action on PWM%s A triggered by event_t1 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN")
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
    ///Bits 0:1 - Configures action on PWM%s A triggered by event TEZ when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn utez(&mut self) -> UTEZ_W<GEN_SPEC> {
        UTEZ_W::new(self, 0)
    }
    ///Bits 2:3 - Configures action on PWM%s A triggered by event TEP when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn utep(&mut self) -> UTEP_W<GEN_SPEC> {
        UTEP_W::new(self, 2)
    }
    ///Bits 4:5 - Configures action on PWM%s A triggered by event TEA when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn utea(&mut self) -> UTEA_W<GEN_SPEC> {
        UTEA_W::new(self, 4)
    }
    ///Bits 6:7 - Configures action on PWM%s A triggered by event TEB when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn uteb(&mut self) -> UTEB_W<GEN_SPEC> {
        UTEB_W::new(self, 6)
    }
    ///Bits 8:9 - Configures action on PWM%s A triggered by event_t0 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn ut0(&mut self) -> UT0_W<GEN_SPEC> {
        UT0_W::new(self, 8)
    }
    ///Bits 10:11 - Configures action on PWM%s A triggered by event_t1 when timer increasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<GEN_SPEC> {
        UT1_W::new(self, 10)
    }
    ///Bits 12:13 - Configures action on PWM%s A triggered by event TEZ when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn dtez(&mut self) -> DTEZ_W<GEN_SPEC> {
        DTEZ_W::new(self, 12)
    }
    ///Bits 14:15 - Configures action on PWM%s A triggered by event TEP when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn dtep(&mut self) -> DTEP_W<GEN_SPEC> {
        DTEP_W::new(self, 14)
    }
    ///Bits 16:17 - Configures action on PWM%s A triggered by event TEA when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn dtea(&mut self) -> DTEA_W<GEN_SPEC> {
        DTEA_W::new(self, 16)
    }
    ///Bits 18:19 - Configures action on PWM%s A triggered by event TEB when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn dteb(&mut self) -> DTEB_W<GEN_SPEC> {
        DTEB_W::new(self, 18)
    }
    ///Bits 20:21 - Configures action on PWM%s A triggered by event_t0 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn dt0(&mut self) -> DT0_W<GEN_SPEC> {
        DT0_W::new(self, 20)
    }
    ///Bits 22:23 - Configures action on PWM%s A triggered by event_t1 when timer decreasing.\\0: No change\\1: Low\\2: High\\3: Toggle
    #[inline(always)]
    #[must_use]
    pub fn dt1(&mut self) -> DT1_W<GEN_SPEC> {
        DT1_W::new(self, 22)
    }
}
/**Actions triggered by events on PWMx%s

You can [`read`](crate::generic::Reg::read) this register and get [`gen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GEN_SPEC;
impl crate::RegisterSpec for GEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gen::R`](R) reader structure
impl crate::Readable for GEN_SPEC {}
///`write(|w| ..)` method takes [`gen::W`](W) writer structure
impl crate::Writable for GEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GEN%s to value 0
impl crate::Resettable for GEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
