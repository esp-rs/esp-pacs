///Register `PERI1_APB_POSTW_EN` reader
pub type R = crate::R<PERI1_APB_POSTW_EN_SPEC>;
///Register `PERI1_APB_POSTW_EN` writer
pub type W = crate::W<PERI1_APB_POSTW_EN_SPEC>;
///Field `PERI1_APB_POSTW_EN` reader - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
pub type PERI1_APB_POSTW_EN_R = crate::BitReader;
///Field `PERI1_APB_POSTW_EN` writer - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
pub type PERI1_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    pub fn peri1_apb_postw_en(&self) -> PERI1_APB_POSTW_EN_R {
        PERI1_APB_POSTW_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI1_APB_POSTW_EN")
            .field("peri1_apb_postw_en", &self.peri1_apb_postw_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - hp_peri1 apb register interface post write enable, 1 will speed up write, but will take some time to update value to register
    #[inline(always)]
    #[must_use]
    pub fn peri1_apb_postw_en(&mut self) -> PERI1_APB_POSTW_EN_W<PERI1_APB_POSTW_EN_SPEC> {
        PERI1_APB_POSTW_EN_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`peri1_apb_postw_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri1_apb_postw_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI1_APB_POSTW_EN_SPEC;
impl crate::RegisterSpec for PERI1_APB_POSTW_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri1_apb_postw_en::R`](R) reader structure
impl crate::Readable for PERI1_APB_POSTW_EN_SPEC {}
///`write(|w| ..)` method takes [`peri1_apb_postw_en::W`](W) writer structure
impl crate::Writable for PERI1_APB_POSTW_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI1_APB_POSTW_EN to value 0
impl crate::Resettable for PERI1_APB_POSTW_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
