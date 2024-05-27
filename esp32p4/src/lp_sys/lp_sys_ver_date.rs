///Register `LP_SYS_VER_DATE` reader
pub type R = crate::R<LP_SYS_VER_DATE_SPEC>;
///Register `LP_SYS_VER_DATE` writer
pub type W = crate::W<LP_SYS_VER_DATE_SPEC>;
///Field `VER_DATE` reader - need_des
pub type VER_DATE_R = crate::FieldReader<u32>;
///Field `VER_DATE` writer - need_des
pub type VER_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn ver_date(&self) -> VER_DATE_R {
        VER_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_SYS_VER_DATE")
            .field("ver_date", &self.ver_date())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ver_date(&mut self) -> VER_DATE_W<LP_SYS_VER_DATE_SPEC> {
        VER_DATE_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sys_ver_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sys_ver_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_SYS_VER_DATE_SPEC;
impl crate::RegisterSpec for LP_SYS_VER_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_sys_ver_date::R`](R) reader structure
impl crate::Readable for LP_SYS_VER_DATE_SPEC {}
///`write(|w| ..)` method takes [`lp_sys_ver_date::W`](W) writer structure
impl crate::Writable for LP_SYS_VER_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_SYS_VER_DATE to value 0x2023_0509
impl crate::Resettable for LP_SYS_VER_DATE_SPEC {
    const RESET_VALUE: u32 = 0x2023_0509;
}
