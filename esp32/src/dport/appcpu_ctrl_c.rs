///Register `APPCPU_CTRL_C` reader
pub type R = crate::R<APPCPU_CTRL_C_SPEC>;
///Register `APPCPU_CTRL_C` writer
pub type W = crate::W<APPCPU_CTRL_C_SPEC>;
///Field `APPCPU_RUNSTALL` reader -
pub type APPCPU_RUNSTALL_R = crate::BitReader;
///Field `APPCPU_RUNSTALL` writer -
pub type APPCPU_RUNSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn appcpu_runstall(&self) -> APPCPU_RUNSTALL_R {
        APPCPU_RUNSTALL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPCPU_CTRL_C")
            .field("appcpu_runstall", &self.appcpu_runstall())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn appcpu_runstall(&mut self) -> APPCPU_RUNSTALL_W<APPCPU_CTRL_C_SPEC> {
        APPCPU_RUNSTALL_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`appcpu_ctrl_c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appcpu_ctrl_c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APPCPU_CTRL_C_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_C_SPEC {
    type Ux = u32;
}
///`read()` method returns [`appcpu_ctrl_c::R`](R) reader structure
impl crate::Readable for APPCPU_CTRL_C_SPEC {}
///`write(|w| ..)` method takes [`appcpu_ctrl_c::W`](W) writer structure
impl crate::Writable for APPCPU_CTRL_C_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APPCPU_CTRL_C to value 0
impl crate::Resettable for APPCPU_CTRL_C_SPEC {
    const RESET_VALUE: u32 = 0;
}
