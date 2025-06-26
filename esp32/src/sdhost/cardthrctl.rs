#[doc = "Register `CARDTHRCTL` reader"]
pub type R = crate::R<CARDTHRCTL_SPEC>;
#[doc = "Register `CARDTHRCTL` writer"]
pub type W = crate::W<CARDTHRCTL_SPEC>;
#[doc = "Field `CARDRDTHREN` reader - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
pub type CARDRDTHREN_R = crate::BitReader;
#[doc = "Field `CARDRDTHREN` writer - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
pub type CARDRDTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDCLRINTEN` reader - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
pub type CARDCLRINTEN_R = crate::BitReader;
#[doc = "Field `CARDCLRINTEN` writer - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
pub type CARDCLRINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDWRTHREN` reader - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
pub type CARDWRTHREN_R = crate::BitReader;
#[doc = "Field `CARDWRTHREN` writer - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
pub type CARDWRTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDTHRESHOLD` reader - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
pub type CARDTHRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `CARDTHRESHOLD` writer - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
pub type CARDTHRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
    #[inline(always)]
    pub fn cardclrinten(&self) -> CARDCLRINTEN_R {
        CARDCLRINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
    #[inline(always)]
    pub fn cardwrthren(&self) -> CARDWRTHREN_R {
        CARDWRTHREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
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
    #[doc = "Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
    #[inline(always)]
    pub fn cardrdthren(&mut self) -> CARDRDTHREN_W<CARDTHRCTL_SPEC> {
        CARDRDTHREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
    #[inline(always)]
    pub fn cardclrinten(&mut self) -> CARDCLRINTEN_W<CARDTHRCTL_SPEC> {
        CARDCLRINTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
    #[inline(always)]
    pub fn cardwrthren(&mut self) -> CARDWRTHREN_W<CARDTHRCTL_SPEC> {
        CARDWRTHREN_W::new(self, 2)
    }
    #[doc = "Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
    #[inline(always)]
    pub fn cardthreshold(&mut self) -> CARDTHRESHOLD_W<CARDTHRCTL_SPEC> {
        CARDTHRESHOLD_W::new(self, 16)
    }
}
#[doc = "Card Threshold Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cardthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cardthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARDTHRCTL_SPEC;
impl crate::RegisterSpec for CARDTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cardthrctl::R`](R) reader structure"]
impl crate::Readable for CARDTHRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cardthrctl::W`](W) writer structure"]
impl crate::Writable for CARDTHRCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CARDTHRCTL to value 0"]
impl crate::Resettable for CARDTHRCTL_SPEC {}
