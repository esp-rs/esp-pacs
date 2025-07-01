#[doc = "Register `XTAL_SLP` reader"]
pub type R = crate::R<XTAL_SLP_SPEC>;
#[doc = "Register `XTAL_SLP` writer"]
pub type W = crate::W<XTAL_SLP_SPEC>;
#[doc = "Field `CNT_TARGET` reader - need_des"]
pub type CNT_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_TARGET` writer - need_des"]
pub type CNT_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn cnt_target(&self) -> CNT_TARGET_R {
        CNT_TARGET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_SLP")
            .field("cnt_target", &self.cnt_target())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn cnt_target(&mut self) -> CNT_TARGET_W<XTAL_SLP_SPEC> {
        CNT_TARGET_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_slp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_slp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTAL_SLP_SPEC;
impl crate::RegisterSpec for XTAL_SLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtal_slp::R`](R) reader structure"]
impl crate::Readable for XTAL_SLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xtal_slp::W`](W) writer structure"]
impl crate::Writable for XTAL_SLP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL_SLP to value 0x000f_0000"]
impl crate::Resettable for XTAL_SLP_SPEC {
    const RESET_VALUE: u32 = 0x000f_0000;
}
