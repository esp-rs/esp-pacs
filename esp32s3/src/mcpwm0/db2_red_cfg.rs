#[doc = "Register `DB2_RED_CFG` reader"]
pub type R = crate::R<DB2_RED_CFG_SPEC>;
#[doc = "Register `DB2_RED_CFG` writer"]
pub type W = crate::W<DB2_RED_CFG_SPEC>;
#[doc = "Field `DB2_RED` reader - Shadow register for RED"]
pub type DB2_RED_R = crate::FieldReader<u16>;
#[doc = "Field `DB2_RED` writer - Shadow register for RED"]
pub type DB2_RED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow register for RED"]
    #[inline(always)]
    pub fn db2_red(&self) -> DB2_RED_R {
        DB2_RED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DB2_RED_CFG")
            .field("db2_red", &format_args!("{}", self.db2_red().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DB2_RED_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow register for RED"]
    #[inline(always)]
    #[must_use]
    pub fn db2_red(&mut self) -> DB2_RED_W<DB2_RED_CFG_SPEC> {
        DB2_RED_W::new(self, 0)
    }
}
#[doc = "Shadow register for rising edge delay (RED).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db2_red_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db2_red_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DB2_RED_CFG_SPEC;
impl crate::RegisterSpec for DB2_RED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db2_red_cfg::R`](R) reader structure"]
impl crate::Readable for DB2_RED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`db2_red_cfg::W`](W) writer structure"]
impl crate::Writable for DB2_RED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DB2_RED_CFG to value 0"]
impl crate::Resettable for DB2_RED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
