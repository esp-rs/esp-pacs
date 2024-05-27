///Register `XTAL_SLP` reader
pub type R = crate::R<XTAL_SLP_SPEC>;
///Register `XTAL_SLP` writer
pub type W = crate::W<XTAL_SLP_SPEC>;
///Field `CNT_TARGET` reader - need_des
pub type CNT_TARGET_R = crate::FieldReader<u16>;
///Field `CNT_TARGET` writer - need_des
pub type CNT_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - need_des
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
    ///Bits 16:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn cnt_target(&mut self) -> CNT_TARGET_W<XTAL_SLP_SPEC> {
        CNT_TARGET_W::new(self, 16)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_slp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_slp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XTAL_SLP_SPEC;
impl crate::RegisterSpec for XTAL_SLP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`xtal_slp::R`](R) reader structure
impl crate::Readable for XTAL_SLP_SPEC {}
///`write(|w| ..)` method takes [`xtal_slp::W`](W) writer structure
impl crate::Writable for XTAL_SLP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets XTAL_SLP to value 0x000f_0000
impl crate::Resettable for XTAL_SLP_SPEC {
    const RESET_VALUE: u32 = 0x000f_0000;
}
