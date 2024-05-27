///Register `DPA_CTRL0` reader
pub type R = crate::R<DPA_CTRL0_SPEC>;
///Register `DPA_CTRL0` writer
pub type W = crate::W<DPA_CTRL0_SPEC>;
///Field `SEC_DPA_LEVEL` reader - Reserved
pub type SEC_DPA_LEVEL_R = crate::FieldReader;
///Field `SEC_DPA_LEVEL` writer - Reserved
pub type SEC_DPA_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SEC_DPA_CFG_SEL` reader - Reserved
pub type SEC_DPA_CFG_SEL_R = crate::BitReader;
///Field `SEC_DPA_CFG_SEL` writer - Reserved
pub type SEC_DPA_CFG_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Reserved
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn sec_dpa_cfg_sel(&self) -> SEC_DPA_CFG_SEL_R {
        SEC_DPA_CFG_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPA_CTRL0")
            .field("sec_dpa_level", &self.sec_dpa_level())
            .field("sec_dpa_cfg_sel", &self.sec_dpa_cfg_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn sec_dpa_level(&mut self) -> SEC_DPA_LEVEL_W<DPA_CTRL0_SPEC> {
        SEC_DPA_LEVEL_W::new(self, 0)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn sec_dpa_cfg_sel(&mut self) -> SEC_DPA_CFG_SEL_W<DPA_CTRL0_SPEC> {
        SEC_DPA_CFG_SEL_W::new(self, 2)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`dpa_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpa_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPA_CTRL0_SPEC;
impl crate::RegisterSpec for DPA_CTRL0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpa_ctrl0::R`](R) reader structure
impl crate::Readable for DPA_CTRL0_SPEC {}
///`write(|w| ..)` method takes [`dpa_ctrl0::W`](W) writer structure
impl crate::Writable for DPA_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPA_CTRL0 to value 0
impl crate::Resettable for DPA_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
