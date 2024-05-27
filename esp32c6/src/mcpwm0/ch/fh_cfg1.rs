///Register `FH_CFG1` reader
pub type R = crate::R<FH_CFG1_SPEC>;
///Register `FH_CFG1` writer
pub type W = crate::W<FH_CFG1_SPEC>;
///Field `CLR_OST` reader - a rising edge will clear on going one-shot mode action
pub type CLR_OST_R = crate::BitReader;
///Field `CLR_OST` writer - a rising edge will clear on going one-shot mode action
pub type CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP
pub type CBCPULSE_R = crate::FieldReader;
///Field `CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP
pub type CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action
pub type FORCE_CBC_R = crate::BitReader;
///Field `FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action
pub type FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action
pub type FORCE_OST_R = crate::BitReader;
///Field `FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action
pub type FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - a rising edge will clear on going one-shot mode action
    #[inline(always)]
    pub fn clr_ost(&self) -> CLR_OST_R {
        CLR_OST_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP
    #[inline(always)]
    pub fn cbcpulse(&self) -> CBCPULSE_R {
        CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - a toggle trigger a cycle-by-cycle mode action
    #[inline(always)]
    pub fn force_cbc(&self) -> FORCE_CBC_R {
        FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - a toggle (software negate its value) triggers a one-shot mode action
    #[inline(always)]
    pub fn force_ost(&self) -> FORCE_OST_R {
        FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH_CFG1")
            .field("clr_ost", &self.clr_ost())
            .field("cbcpulse", &self.cbcpulse())
            .field("force_cbc", &self.force_cbc())
            .field("force_ost", &self.force_ost())
            .finish()
    }
}
impl W {
    ///Bit 0 - a rising edge will clear on going one-shot mode action
    #[inline(always)]
    #[must_use]
    pub fn clr_ost(&mut self) -> CLR_OST_W<FH_CFG1_SPEC> {
        CLR_OST_W::new(self, 0)
    }
    ///Bits 1:2 - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP
    #[inline(always)]
    #[must_use]
    pub fn cbcpulse(&mut self) -> CBCPULSE_W<FH_CFG1_SPEC> {
        CBCPULSE_W::new(self, 1)
    }
    ///Bit 3 - a toggle trigger a cycle-by-cycle mode action
    #[inline(always)]
    #[must_use]
    pub fn force_cbc(&mut self) -> FORCE_CBC_W<FH_CFG1_SPEC> {
        FORCE_CBC_W::new(self, 3)
    }
    ///Bit 4 - a toggle (software negate its value) triggers a one-shot mode action
    #[inline(always)]
    #[must_use]
    pub fn force_ost(&mut self) -> FORCE_OST_W<FH_CFG1_SPEC> {
        FORCE_OST_W::new(self, 4)
    }
}
/**Software triggers for fault handler actions

You can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FH_CFG1_SPEC;
impl crate::RegisterSpec for FH_CFG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fh_cfg1::R`](R) reader structure
impl crate::Readable for FH_CFG1_SPEC {}
///`write(|w| ..)` method takes [`fh_cfg1::W`](W) writer structure
impl crate::Writable for FH_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FH_CFG1 to value 0
impl crate::Resettable for FH_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
