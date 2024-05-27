///Register `DEBUG_SEL_CFG0` reader
pub type R = crate::R<DEBUG_SEL_CFG0_SPEC>;
///Register `DEBUG_SEL_CFG0` writer
pub type W = crate::W<DEBUG_SEL_CFG0_SPEC>;
///Field `DEBUG_FIELD0_SEL` reader -
pub type DEBUG_FIELD0_SEL_R = crate::FieldReader;
///Field `DEBUG_FIELD0_SEL` writer -
pub type DEBUG_FIELD0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEBUG_FIELD1_SEL` reader -
pub type DEBUG_FIELD1_SEL_R = crate::FieldReader;
///Field `DEBUG_FIELD1_SEL` writer -
pub type DEBUG_FIELD1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEBUG_FIELD2_SEL` reader -
pub type DEBUG_FIELD2_SEL_R = crate::FieldReader;
///Field `DEBUG_FIELD2_SEL` writer -
pub type DEBUG_FIELD2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEBUG_FIELD3_SEL` reader -
pub type DEBUG_FIELD3_SEL_R = crate::FieldReader;
///Field `DEBUG_FIELD3_SEL` writer -
pub type DEBUG_FIELD3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn debug_field0_sel(&self) -> DEBUG_FIELD0_SEL_R {
        DEBUG_FIELD0_SEL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12
    #[inline(always)]
    pub fn debug_field1_sel(&self) -> DEBUG_FIELD1_SEL_R {
        DEBUG_FIELD1_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20
    #[inline(always)]
    pub fn debug_field2_sel(&self) -> DEBUG_FIELD2_SEL_R {
        DEBUG_FIELD2_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28
    #[inline(always)]
    pub fn debug_field3_sel(&self) -> DEBUG_FIELD3_SEL_R {
        DEBUG_FIELD3_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL_CFG0")
            .field("debug_field0_sel", &self.debug_field0_sel())
            .field("debug_field1_sel", &self.debug_field1_sel())
            .field("debug_field2_sel", &self.debug_field2_sel())
            .field("debug_field3_sel", &self.debug_field3_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    #[must_use]
    pub fn debug_field0_sel(&mut self) -> DEBUG_FIELD0_SEL_W<DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD0_SEL_W::new(self, 0)
    }
    ///Bits 8:12
    #[inline(always)]
    #[must_use]
    pub fn debug_field1_sel(&mut self) -> DEBUG_FIELD1_SEL_W<DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD1_SEL_W::new(self, 8)
    }
    ///Bits 16:20
    #[inline(always)]
    #[must_use]
    pub fn debug_field2_sel(&mut self) -> DEBUG_FIELD2_SEL_W<DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD2_SEL_W::new(self, 16)
    }
    ///Bits 24:28
    #[inline(always)]
    #[must_use]
    pub fn debug_field3_sel(&mut self) -> DEBUG_FIELD3_SEL_W<DEBUG_SEL_CFG0_SPEC> {
        DEBUG_FIELD3_SEL_W::new(self, 24)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`debug_sel_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEBUG_SEL_CFG0_SPEC;
impl crate::RegisterSpec for DEBUG_SEL_CFG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`debug_sel_cfg0::R`](R) reader structure
impl crate::Readable for DEBUG_SEL_CFG0_SPEC {}
///`write(|w| ..)` method takes [`debug_sel_cfg0::W`](W) writer structure
impl crate::Writable for DEBUG_SEL_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEBUG_SEL_CFG0 to value 0
impl crate::Resettable for DEBUG_SEL_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
