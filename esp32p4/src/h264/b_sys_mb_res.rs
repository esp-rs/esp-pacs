///Register `B_SYS_MB_RES` reader
pub type R = crate::R<B_SYS_MB_RES_SPEC>;
///Register `B_SYS_MB_RES` writer
pub type W = crate::W<B_SYS_MB_RES_SPEC>;
///Field `B_SYS_TOTAL_MB_Y` reader - Configures video B vertical MB resolution.
pub type B_SYS_TOTAL_MB_Y_R = crate::FieldReader;
///Field `B_SYS_TOTAL_MB_Y` writer - Configures video B vertical MB resolution.
pub type B_SYS_TOTAL_MB_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `B_SYS_TOTAL_MB_X` reader - Configures video B horizontal MB resolution.
pub type B_SYS_TOTAL_MB_X_R = crate::FieldReader;
///Field `B_SYS_TOTAL_MB_X` writer - Configures video B horizontal MB resolution.
pub type B_SYS_TOTAL_MB_X_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Configures video B vertical MB resolution.
    #[inline(always)]
    pub fn b_sys_total_mb_y(&self) -> B_SYS_TOTAL_MB_Y_R {
        B_SYS_TOTAL_MB_Y_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - Configures video B horizontal MB resolution.
    #[inline(always)]
    pub fn b_sys_total_mb_x(&self) -> B_SYS_TOTAL_MB_X_R {
        B_SYS_TOTAL_MB_X_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_SYS_MB_RES")
            .field("b_sys_total_mb_y", &self.b_sys_total_mb_y())
            .field("b_sys_total_mb_x", &self.b_sys_total_mb_x())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Configures video B vertical MB resolution.
    #[inline(always)]
    #[must_use]
    pub fn b_sys_total_mb_y(&mut self) -> B_SYS_TOTAL_MB_Y_W<B_SYS_MB_RES_SPEC> {
        B_SYS_TOTAL_MB_Y_W::new(self, 0)
    }
    ///Bits 7:13 - Configures video B horizontal MB resolution.
    #[inline(always)]
    #[must_use]
    pub fn b_sys_total_mb_x(&mut self) -> B_SYS_TOTAL_MB_X_W<B_SYS_MB_RES_SPEC> {
        B_SYS_TOTAL_MB_X_W::new(self, 7)
    }
}
/**Video B horizontal and vertical MB resolution register.

You can [`read`](crate::generic::Reg::read) this register and get [`b_sys_mb_res::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_sys_mb_res::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct B_SYS_MB_RES_SPEC;
impl crate::RegisterSpec for B_SYS_MB_RES_SPEC {
    type Ux = u32;
}
///`read()` method returns [`b_sys_mb_res::R`](R) reader structure
impl crate::Readable for B_SYS_MB_RES_SPEC {}
///`write(|w| ..)` method takes [`b_sys_mb_res::W`](W) writer structure
impl crate::Writable for B_SYS_MB_RES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets B_SYS_MB_RES to value 0
impl crate::Resettable for B_SYS_MB_RES_SPEC {
    const RESET_VALUE: u32 = 0;
}
