///Register `LP_PROBEB_CTRL` reader
pub type R = crate::R<LP_PROBEB_CTRL_SPEC>;
///Register `LP_PROBEB_CTRL` writer
pub type W = crate::W<LP_PROBEB_CTRL_SPEC>;
///Field `PROBE_B_MOD_SEL` reader - need_des
pub type PROBE_B_MOD_SEL_R = crate::FieldReader<u16>;
///Field `PROBE_B_MOD_SEL` writer - need_des
pub type PROBE_B_MOD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PROBE_B_TOP_SEL` reader - need_des
pub type PROBE_B_TOP_SEL_R = crate::FieldReader;
///Field `PROBE_B_TOP_SEL` writer - need_des
pub type PROBE_B_TOP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PROBE_B_EN` reader - need_des
pub type PROBE_B_EN_R = crate::BitReader;
///Field `PROBE_B_EN` writer - need_des
pub type PROBE_B_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - need_des
    #[inline(always)]
    pub fn probe_b_mod_sel(&self) -> PROBE_B_MOD_SEL_R {
        PROBE_B_MOD_SEL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - need_des
    #[inline(always)]
    pub fn probe_b_top_sel(&self) -> PROBE_B_TOP_SEL_R {
        PROBE_B_TOP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - need_des
    #[inline(always)]
    pub fn probe_b_en(&self) -> PROBE_B_EN_R {
        PROBE_B_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PROBEB_CTRL")
            .field("probe_b_mod_sel", &self.probe_b_mod_sel())
            .field("probe_b_top_sel", &self.probe_b_top_sel())
            .field("probe_b_en", &self.probe_b_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_b_mod_sel(&mut self) -> PROBE_B_MOD_SEL_W<LP_PROBEB_CTRL_SPEC> {
        PROBE_B_MOD_SEL_W::new(self, 0)
    }
    ///Bits 16:23 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_b_top_sel(&mut self) -> PROBE_B_TOP_SEL_W<LP_PROBEB_CTRL_SPEC> {
        PROBE_B_TOP_SEL_W::new(self, 16)
    }
    ///Bit 24 - need_des
    #[inline(always)]
    #[must_use]
    pub fn probe_b_en(&mut self) -> PROBE_B_EN_W<LP_PROBEB_CTRL_SPEC> {
        PROBE_B_EN_W::new(self, 24)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_probeb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_probeb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_PROBEB_CTRL_SPEC;
impl crate::RegisterSpec for LP_PROBEB_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_probeb_ctrl::R`](R) reader structure
impl crate::Readable for LP_PROBEB_CTRL_SPEC {}
///`write(|w| ..)` method takes [`lp_probeb_ctrl::W`](W) writer structure
impl crate::Writable for LP_PROBEB_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_PROBEB_CTRL to value 0
impl crate::Resettable for LP_PROBEB_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
