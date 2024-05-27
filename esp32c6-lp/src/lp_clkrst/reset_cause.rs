///Register `RESET_CAUSE` reader
pub type R = crate::R<RESET_CAUSE_SPEC>;
///Register `RESET_CAUSE` writer
pub type W = crate::W<RESET_CAUSE_SPEC>;
///Field `RESET_CAUSE` reader - need_des
pub type RESET_CAUSE_R = crate::FieldReader;
///Field `CORE0_RESET_FLAG` reader - need_des
pub type CORE0_RESET_FLAG_R = crate::BitReader;
///Field `CORE0_RESET_CAUSE_CLR` writer - need_des
pub type CORE0_RESET_CAUSE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_RESET_FLAG_SET` writer - need_des
pub type CORE0_RESET_FLAG_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE0_RESET_FLAG_CLR` writer - need_des
pub type CORE0_RESET_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - need_des
    #[inline(always)]
    pub fn reset_cause(&self) -> RESET_CAUSE_R {
        RESET_CAUSE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    pub fn core0_reset_flag(&self) -> CORE0_RESET_FLAG_R {
        CORE0_RESET_FLAG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_CAUSE")
            .field("reset_cause", &self.reset_cause())
            .field("core0_reset_flag", &self.core0_reset_flag())
            .finish()
    }
}
impl W {
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn core0_reset_cause_clr(&mut self) -> CORE0_RESET_CAUSE_CLR_W<RESET_CAUSE_SPEC> {
        CORE0_RESET_CAUSE_CLR_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn core0_reset_flag_set(&mut self) -> CORE0_RESET_FLAG_SET_W<RESET_CAUSE_SPEC> {
        CORE0_RESET_FLAG_SET_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn core0_reset_flag_clr(&mut self) -> CORE0_RESET_FLAG_CLR_W<RESET_CAUSE_SPEC> {
        CORE0_RESET_FLAG_CLR_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`reset_cause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_cause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RESET_CAUSE_SPEC;
impl crate::RegisterSpec for RESET_CAUSE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`reset_cause::R`](R) reader structure
impl crate::Readable for RESET_CAUSE_SPEC {}
///`write(|w| ..)` method takes [`reset_cause::W`](W) writer structure
impl crate::Writable for RESET_CAUSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RESET_CAUSE to value 0x20
impl crate::Resettable for RESET_CAUSE_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
