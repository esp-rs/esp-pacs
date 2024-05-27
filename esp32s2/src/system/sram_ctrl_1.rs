///Register `SRAM_CTRL_1` reader
pub type R = crate::R<SRAM_CTRL_1_SPEC>;
///Register `SRAM_CTRL_1` writer
pub type W = crate::W<SRAM_CTRL_1_SPEC>;
///Field `SRAM_FORCE_PD` reader - This field is used to power down internal SRAM.
pub type SRAM_FORCE_PD_R = crate::FieldReader<u32>;
///Field `SRAM_FORCE_PD` writer - This field is used to power down internal SRAM.
pub type SRAM_FORCE_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - This field is used to power down internal SRAM.
    #[inline(always)]
    pub fn sram_force_pd(&self) -> SRAM_FORCE_PD_R {
        SRAM_FORCE_PD_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL_1")
            .field("sram_force_pd", &self.sram_force_pd())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - This field is used to power down internal SRAM.
    #[inline(always)]
    #[must_use]
    pub fn sram_force_pd(&mut self) -> SRAM_FORCE_PD_W<SRAM_CTRL_1_SPEC> {
        SRAM_FORCE_PD_W::new(self, 0)
    }
}
/**System SRAM configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRAM_CTRL_1_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sram_ctrl_1::R`](R) reader structure
impl crate::Readable for SRAM_CTRL_1_SPEC {}
///`write(|w| ..)` method takes [`sram_ctrl_1::W`](W) writer structure
impl crate::Writable for SRAM_CTRL_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRAM_CTRL_1 to value 0
impl crate::Resettable for SRAM_CTRL_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
