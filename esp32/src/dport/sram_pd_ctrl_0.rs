///Register `SRAM_PD_CTRL_0` reader
pub type R = crate::R<SRAM_PD_CTRL_0_SPEC>;
///Register `SRAM_PD_CTRL_0` writer
pub type W = crate::W<SRAM_PD_CTRL_0_SPEC>;
///Field `SRAM_PD_0` reader -
pub type SRAM_PD_0_R = crate::FieldReader<u32>;
///Field `SRAM_PD_0` writer -
pub type SRAM_PD_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn sram_pd_0(&self) -> SRAM_PD_0_R {
        SRAM_PD_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_PD_CTRL_0")
            .field("sram_pd_0", &self.sram_pd_0())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn sram_pd_0(&mut self) -> SRAM_PD_0_W<SRAM_PD_CTRL_0_SPEC> {
        SRAM_PD_0_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sram_pd_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_pd_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRAM_PD_CTRL_0_SPEC;
impl crate::RegisterSpec for SRAM_PD_CTRL_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sram_pd_ctrl_0::R`](R) reader structure
impl crate::Readable for SRAM_PD_CTRL_0_SPEC {}
///`write(|w| ..)` method takes [`sram_pd_ctrl_0::W`](W) writer structure
impl crate::Writable for SRAM_PD_CTRL_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRAM_PD_CTRL_0 to value 0
impl crate::Resettable for SRAM_PD_CTRL_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
