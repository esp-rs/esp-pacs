///Register `CARDTHRCTL` reader
pub type R = crate::R<CARDTHRCTL_SPEC>;
///Register `CARDTHRCTL` writer
pub type W = crate::W<CARDTHRCTL_SPEC>;
///Field `CARDRDTHREN` reader - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled.
pub type CARDRDTHREN_R = crate::BitReader;
///Field `CARDRDTHREN` writer - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled.
pub type CARDRDTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARDCLRINTEN` reader - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled.
pub type CARDCLRINTEN_R = crate::BitReader;
///Field `CARDCLRINTEN` writer - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled.
pub type CARDCLRINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARDWRTHREN` reader - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled.
pub type CARDWRTHREN_R = crate::BitReader;
///Field `CARDWRTHREN` writer - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled.
pub type CARDWRTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARDTHRESHOLD` reader - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
pub type CARDTHRESHOLD_R = crate::FieldReader<u16>;
///Field `CARDTHRESHOLD` writer - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
pub type CARDTHRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled.
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled.
    #[inline(always)]
    pub fn cardclrinten(&self) -> CARDCLRINTEN_R {
        CARDCLRINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled.
    #[inline(always)]
    pub fn cardwrthren(&self) -> CARDWRTHREN_R {
        CARDWRTHREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    #[inline(always)]
    pub fn cardthreshold(&self) -> CARDTHRESHOLD_R {
        CARDTHRESHOLD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARDTHRCTL")
            .field("cardrdthren", &self.cardrdthren())
            .field("cardclrinten", &self.cardclrinten())
            .field("cardwrthren", &self.cardwrthren())
            .field("cardthreshold", &self.cardthreshold())
            .finish()
    }
}
impl W {
    ///Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled.
    #[inline(always)]
    #[must_use]
    pub fn cardrdthren(&mut self) -> CARDRDTHREN_W<CARDTHRCTL_SPEC> {
        CARDRDTHREN_W::new(self, 0)
    }
    ///Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled.
    #[inline(always)]
    #[must_use]
    pub fn cardclrinten(&mut self) -> CARDCLRINTEN_W<CARDTHRCTL_SPEC> {
        CARDCLRINTEN_W::new(self, 1)
    }
    ///Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled.
    #[inline(always)]
    #[must_use]
    pub fn cardwrthren(&mut self) -> CARDWRTHREN_W<CARDTHRCTL_SPEC> {
        CARDWRTHREN_W::new(self, 2)
    }
    ///Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1.
    #[inline(always)]
    #[must_use]
    pub fn cardthreshold(&mut self) -> CARDTHRESHOLD_W<CARDTHRCTL_SPEC> {
        CARDTHRESHOLD_W::new(self, 16)
    }
}
/**Card Threshold Control register

You can [`read`](crate::generic::Reg::read) this register and get [`cardthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cardthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CARDTHRCTL_SPEC;
impl crate::RegisterSpec for CARDTHRCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cardthrctl::R`](R) reader structure
impl crate::Readable for CARDTHRCTL_SPEC {}
///`write(|w| ..)` method takes [`cardthrctl::W`](W) writer structure
impl crate::Writable for CARDTHRCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CARDTHRCTL to value 0
impl crate::Resettable for CARDTHRCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
