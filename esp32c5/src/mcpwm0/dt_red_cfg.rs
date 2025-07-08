#[doc = "Register `DT%s_RED_CFG` reader"]
pub type R = crate::R<DT_RED_CFG_SPEC>;
#[doc = "Register `DT%s_RED_CFG` writer"]
pub type W = crate::W<DT_RED_CFG_SPEC>;
#[doc = "Field `DB_RED` reader - Configures shadow register for RED."]
pub type DB_RED_R = crate::FieldReader<u16>;
#[doc = "Field `DB_RED` writer - Configures shadow register for RED."]
pub type DB_RED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures shadow register for RED."]
    #[inline(always)]
    pub fn db_red(&self) -> DB_RED_R {
        DB_RED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT_RED_CFG")
            .field("db_red", &self.db_red())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures shadow register for RED."]
    #[inline(always)]
    pub fn db_red(&mut self) -> DB_RED_W<DT_RED_CFG_SPEC> {
        DB_RED_W::new(self, 0)
    }
}
#[doc = "Rising edge delay (RED) shadow register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt_red_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt_red_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_RED_CFG_SPEC;
impl crate::RegisterSpec for DT_RED_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt_red_cfg::R`](R) reader structure"]
impl crate::Readable for DT_RED_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt_red_cfg::W`](W) writer structure"]
impl crate::Writable for DT_RED_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT%s_RED_CFG to value 0"]
impl crate::Resettable for DT_RED_CFG_SPEC {}
