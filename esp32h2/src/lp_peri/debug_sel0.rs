///Register `DEBUG_SEL0` reader
pub type R = crate::R<DEBUG_SEL0_SPEC>;
///Register `DEBUG_SEL0` writer
pub type W = crate::W<DEBUG_SEL0_SPEC>;
///Field `DEBUG_SEL0` reader - need des
pub type DEBUG_SEL0_R = crate::FieldReader;
///Field `DEBUG_SEL0` writer - need des
pub type DEBUG_SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DEBUG_SEL1` reader - need des
pub type DEBUG_SEL1_R = crate::FieldReader;
///Field `DEBUG_SEL1` writer - need des
pub type DEBUG_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DEBUG_SEL2` reader - need des
pub type DEBUG_SEL2_R = crate::FieldReader;
///Field `DEBUG_SEL2` writer - need des
pub type DEBUG_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DEBUG_SEL3` reader - need des
pub type DEBUG_SEL3_R = crate::FieldReader;
///Field `DEBUG_SEL3` writer - need des
pub type DEBUG_SEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - need des
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - need des
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:20 - need des
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    ///Bits 21:27 - need des
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL0")
            .field("debug_sel0", &self.debug_sel0())
            .field("debug_sel1", &self.debug_sel1())
            .field("debug_sel2", &self.debug_sel2())
            .field("debug_sel3", &self.debug_sel3())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - need des
    #[inline(always)]
    #[must_use]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W<DEBUG_SEL0_SPEC> {
        DEBUG_SEL0_W::new(self, 0)
    }
    ///Bits 7:13 - need des
    #[inline(always)]
    #[must_use]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W<DEBUG_SEL0_SPEC> {
        DEBUG_SEL1_W::new(self, 7)
    }
    ///Bits 14:20 - need des
    #[inline(always)]
    #[must_use]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W<DEBUG_SEL0_SPEC> {
        DEBUG_SEL2_W::new(self, 14)
    }
    ///Bits 21:27 - need des
    #[inline(always)]
    #[must_use]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W<DEBUG_SEL0_SPEC> {
        DEBUG_SEL3_W::new(self, 21)
    }
}
/**need des

You can [`read`](crate::generic::Reg::read) this register and get [`debug_sel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEBUG_SEL0_SPEC;
impl crate::RegisterSpec for DEBUG_SEL0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`debug_sel0::R`](R) reader structure
impl crate::Readable for DEBUG_SEL0_SPEC {}
///`write(|w| ..)` method takes [`debug_sel0::W`](W) writer structure
impl crate::Writable for DEBUG_SEL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEBUG_SEL0 to value 0
impl crate::Resettable for DEBUG_SEL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
