#[doc = "Register `DEBUG_SEL_CFG1` reader"]
pub type R = crate::R<DEBUG_SEL_CFG1_SPEC>;
#[doc = "Register `DEBUG_SEL_CFG1` writer"]
pub type W = crate::W<DEBUG_SEL_CFG1_SPEC>;
#[doc = "Field `DEBUG_FIELD4_SEL` reader - "]
pub type DEBUG_FIELD4_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD4_SEL` writer - "]
pub type DEBUG_FIELD4_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_FIELD5_SEL` reader - "]
pub type DEBUG_FIELD5_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD5_SEL` writer - "]
pub type DEBUG_FIELD5_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_FIELD6_SEL` reader - "]
pub type DEBUG_FIELD6_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD6_SEL` writer - "]
pub type DEBUG_FIELD6_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DEBUG_FIELD7_SEL` reader - "]
pub type DEBUG_FIELD7_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_FIELD7_SEL` writer - "]
pub type DEBUG_FIELD7_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn debug_field4_sel(&self) -> DEBUG_FIELD4_SEL_R {
        DEBUG_FIELD4_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn debug_field5_sel(&self) -> DEBUG_FIELD5_SEL_R {
        DEBUG_FIELD5_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn debug_field6_sel(&self) -> DEBUG_FIELD6_SEL_R {
        DEBUG_FIELD6_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn debug_field7_sel(&self) -> DEBUG_FIELD7_SEL_R {
        DEBUG_FIELD7_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL_CFG1")
            .field("debug_field4_sel", &self.debug_field4_sel())
            .field("debug_field5_sel", &self.debug_field5_sel())
            .field("debug_field6_sel", &self.debug_field6_sel())
            .field("debug_field7_sel", &self.debug_field7_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn debug_field4_sel(&mut self) -> DEBUG_FIELD4_SEL_W<DEBUG_SEL_CFG1_SPEC> {
        DEBUG_FIELD4_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn debug_field5_sel(&mut self) -> DEBUG_FIELD5_SEL_W<DEBUG_SEL_CFG1_SPEC> {
        DEBUG_FIELD5_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn debug_field6_sel(&mut self) -> DEBUG_FIELD6_SEL_W<DEBUG_SEL_CFG1_SPEC> {
        DEBUG_FIELD6_SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    #[must_use]
    pub fn debug_field7_sel(&mut self) -> DEBUG_FIELD7_SEL_W<DEBUG_SEL_CFG1_SPEC> {
        DEBUG_FIELD7_SEL_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_sel_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_SEL_CFG1_SPEC;
impl crate::RegisterSpec for DEBUG_SEL_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_sel_cfg1::R`](R) reader structure"]
impl crate::Readable for DEBUG_SEL_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_sel_cfg1::W`](W) writer structure"]
impl crate::Writable for DEBUG_SEL_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL_CFG1 to value 0"]
impl crate::Resettable for DEBUG_SEL_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
