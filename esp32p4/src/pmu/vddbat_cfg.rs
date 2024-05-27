///Register `VDDBAT_CFG` reader
pub type R = crate::R<VDDBAT_CFG_SPEC>;
///Register `VDDBAT_CFG` writer
pub type W = crate::W<VDDBAT_CFG_SPEC>;
///Field `ANA_VDDBAT_MODE` reader - need_des
pub type ANA_VDDBAT_MODE_R = crate::FieldReader;
///Field `VDDBAT_SW_UPDATE` writer - need_des
pub type VDDBAT_SW_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - need_des
    #[inline(always)]
    pub fn ana_vddbat_mode(&self) -> ANA_VDDBAT_MODE_R {
        ANA_VDDBAT_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDBAT_CFG")
            .field("ana_vddbat_mode", &self.ana_vddbat_mode())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vddbat_sw_update(&mut self) -> VDDBAT_SW_UPDATE_W<VDDBAT_CFG_SPEC> {
        VDDBAT_SW_UPDATE_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`vddbat_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddbat_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VDDBAT_CFG_SPEC;
impl crate::RegisterSpec for VDDBAT_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vddbat_cfg::R`](R) reader structure
impl crate::Readable for VDDBAT_CFG_SPEC {}
///`write(|w| ..)` method takes [`vddbat_cfg::W`](W) writer structure
impl crate::Writable for VDDBAT_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VDDBAT_CFG to value 0
impl crate::Resettable for VDDBAT_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
