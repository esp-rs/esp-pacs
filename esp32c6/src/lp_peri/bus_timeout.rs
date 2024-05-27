///Register `BUS_TIMEOUT` reader
pub type R = crate::R<BUS_TIMEOUT_SPEC>;
///Register `BUS_TIMEOUT` writer
pub type W = crate::W<BUS_TIMEOUT_SPEC>;
///Field `LP_PERI_TIMEOUT_THRES` reader - need_des
pub type LP_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
///Field `LP_PERI_TIMEOUT_THRES` writer - need_des
pub type LP_PERI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `LP_PERI_TIMEOUT_INT_CLEAR` writer - need_des
pub type LP_PERI_TIMEOUT_INT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LP_PERI_TIMEOUT_PROTECT_EN` reader - need_des
pub type LP_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
///Field `LP_PERI_TIMEOUT_PROTECT_EN` writer - need_des
pub type LP_PERI_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 14:29 - need_des
    #[inline(always)]
    pub fn lp_peri_timeout_thres(&self) -> LP_PERI_TIMEOUT_THRES_R {
        LP_PERI_TIMEOUT_THRES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_peri_timeout_protect_en(&self) -> LP_PERI_TIMEOUT_PROTECT_EN_R {
        LP_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMEOUT")
            .field("lp_peri_timeout_thres", &self.lp_peri_timeout_thres())
            .field(
                "lp_peri_timeout_protect_en",
                &self.lp_peri_timeout_protect_en(),
            )
            .finish()
    }
}
impl W {
    ///Bits 14:29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_thres(&mut self) -> LP_PERI_TIMEOUT_THRES_W<BUS_TIMEOUT_SPEC> {
        LP_PERI_TIMEOUT_THRES_W::new(self, 14)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_int_clear(&mut self) -> LP_PERI_TIMEOUT_INT_CLEAR_W<BUS_TIMEOUT_SPEC> {
        LP_PERI_TIMEOUT_INT_CLEAR_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_protect_en(&mut self) -> LP_PERI_TIMEOUT_PROTECT_EN_W<BUS_TIMEOUT_SPEC> {
        LP_PERI_TIMEOUT_PROTECT_EN_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUS_TIMEOUT_SPEC;
impl crate::RegisterSpec for BUS_TIMEOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bus_timeout::R`](R) reader structure
impl crate::Readable for BUS_TIMEOUT_SPEC {}
///`write(|w| ..)` method takes [`bus_timeout::W`](W) writer structure
impl crate::Writable for BUS_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BUS_TIMEOUT to value 0xbfff_c000
impl crate::Resettable for BUS_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0xbfff_c000;
}
