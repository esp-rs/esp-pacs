#[doc = "Register `REGISTER56_GENERALPURPOSEIOREGISTER` reader"]
pub type R = crate::R<REGISTER56_GENERALPURPOSEIOREGISTER_SPEC>;
#[doc = "Register `REGISTER56_GENERALPURPOSEIOREGISTER` writer"]
pub type W = crate::W<REGISTER56_GENERALPURPOSEIOREGISTER_SPEC>;
#[doc = "Field `GPIS` reader - General Purpose Input Status This field gives the status of the signals connected to the gpi_i input ports This field is of the following types based on the setting of the corresponding GPIT field of this register: Latchedlow _LL_: This field is cleared when the corresponding gpi_i input becomes low This field remains low until the host reads this field After this, this field reflects the current value of the gpi_i input Latchedhigh _LH_: This field is set when the corresponding gpi_i input becomes high This field remains high until the host reads this field After this, this field reflects the current value of the gpi_i input The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPIS_R = crate::FieldReader;
#[doc = "Field `GPIS` writer - General Purpose Input Status This field gives the status of the signals connected to the gpi_i input ports This field is of the following types based on the setting of the corresponding GPIT field of this register: Latchedlow _LL_: This field is cleared when the corresponding gpi_i input becomes low This field remains low until the host reads this field After this, this field reflects the current value of the gpi_i input Latchedhigh _LH_: This field is set when the corresponding gpi_i input becomes high This field remains high until the host reads this field After this, this field reflects the current value of the gpi_i input The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPIS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPO` reader - General Purpose Output When this bit is set, it directly drives the gpo_o output ports When this bit is reset, it does not directly drive the gpo_o output ports The number of bits available in this field depend on the GP Output Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPO_R = crate::FieldReader;
#[doc = "Field `GPO` writer - General Purpose Output When this bit is set, it directly drives the gpo_o output ports When this bit is reset, it does not directly drive the gpo_o output ports The number of bits available in this field depend on the GP Output Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIE` reader - GPI Interrupt Enable When this bit is set and the programmed event _LL or LH_ occurs on the corresponding GPIS bit, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is set Accordingly, the interrupt is generated on the mci_intr_o or sbd_intr_o The GPIIS bit is cleared when the host reads the Bits\\[7:0\\] of this register When reset, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is not set when any event occurs on the corresponding GPIS bits The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPIE_R = crate::FieldReader;
#[doc = "Field `GPIE` writer - GPI Interrupt Enable When this bit is set and the programmed event _LL or LH_ occurs on the corresponding GPIS bit, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is set Accordingly, the interrupt is generated on the mci_intr_o or sbd_intr_o The GPIIS bit is cleared when the host reads the Bits\\[7:0\\] of this register When reset, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is not set when any event occurs on the corresponding GPIS bits The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPIE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIT` reader - GPI Type When set, this bit indicates that the corresponding GPIS is of latchedlow _LL_ type When reset, this bit indicates that the corresponding GPIS is of latchedhigh _LH_ type The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPIT_R = crate::FieldReader;
#[doc = "Field `GPIT` writer - GPI Type When set, this bit indicates that the corresponding GPIS is of latchedlow _LL_ type When reset, this bit indicates that the corresponding GPIS is of latchedhigh _LH_ type The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
pub type GPIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - General Purpose Input Status This field gives the status of the signals connected to the gpi_i input ports This field is of the following types based on the setting of the corresponding GPIT field of this register: Latchedlow _LL_: This field is cleared when the corresponding gpi_i input becomes low This field remains low until the host reads this field After this, this field reflects the current value of the gpi_i input Latchedhigh _LH_: This field is set when the corresponding gpi_i input becomes high This field remains high until the host reads this field After this, this field reflects the current value of the gpi_i input The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpis(&self) -> GPIS_R {
        GPIS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - General Purpose Output When this bit is set, it directly drives the gpo_o output ports When this bit is reset, it does not directly drive the gpo_o output ports The number of bits available in this field depend on the GP Output Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpo(&self) -> GPO_R {
        GPO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPI Interrupt Enable When this bit is set and the programmed event _LL or LH_ occurs on the corresponding GPIS bit, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is set Accordingly, the interrupt is generated on the mci_intr_o or sbd_intr_o The GPIIS bit is cleared when the host reads the Bits\\[7:0\\] of this register When reset, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is not set when any event occurs on the corresponding GPIS bits The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpie(&self) -> GPIE_R {
        GPIE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPI Type When set, this bit indicates that the corresponding GPIS is of latchedlow _LL_ type When reset, this bit indicates that the corresponding GPIS is of latchedhigh _LH_ type The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpit(&self) -> GPIT_R {
        GPIT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER56_GENERALPURPOSEIOREGISTER")
            .field("gpis", &self.gpis())
            .field("gpo", &self.gpo())
            .field("gpie", &self.gpie())
            .field("gpit", &self.gpit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - General Purpose Input Status This field gives the status of the signals connected to the gpi_i input ports This field is of the following types based on the setting of the corresponding GPIT field of this register: Latchedlow _LL_: This field is cleared when the corresponding gpi_i input becomes low This field remains low until the host reads this field After this, this field reflects the current value of the gpi_i input Latchedhigh _LH_: This field is set when the corresponding gpi_i input becomes high This field remains high until the host reads this field After this, this field reflects the current value of the gpi_i input The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpis(&mut self) -> GPIS_W<'_, REGISTER56_GENERALPURPOSEIOREGISTER_SPEC> {
        GPIS_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - General Purpose Output When this bit is set, it directly drives the gpo_o output ports When this bit is reset, it does not directly drive the gpo_o output ports The number of bits available in this field depend on the GP Output Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpo(&mut self) -> GPO_W<'_, REGISTER56_GENERALPURPOSEIOREGISTER_SPEC> {
        GPO_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - GPI Interrupt Enable When this bit is set and the programmed event _LL or LH_ occurs on the corresponding GPIS bit, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is set Accordingly, the interrupt is generated on the mci_intr_o or sbd_intr_o The GPIIS bit is cleared when the host reads the Bits\\[7:0\\] of this register When reset, Bit 11 _GPIIS_ of Register 14 _Interrupt Status Register_ is not set when any event occurs on the corresponding GPIS bits The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpie(&mut self) -> GPIE_W<'_, REGISTER56_GENERALPURPOSEIOREGISTER_SPEC> {
        GPIE_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - GPI Type When set, this bit indicates that the corresponding GPIS is of latchedlow _LL_ type When reset, this bit indicates that the corresponding GPIS is of latchedhigh _LH_ type The number of bits available in this field depend on the GP Input Signal Width option Other bits are not used _reserved and always reset_"]
    #[inline(always)]
    pub fn gpit(&mut self) -> GPIT_W<'_, REGISTER56_GENERALPURPOSEIOREGISTER_SPEC> {
        GPIT_W::new(self, 24)
    }
}
#[doc = "Provides the control to drive up to 4 bits of output ports _GPO_ and also provides the status of up to 4 input ports _GPIS_\n\nYou can [`read`](crate::Reg::read) this register and get [`register56_generalpurposeioregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register56_generalpurposeioregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER56_GENERALPURPOSEIOREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER56_GENERALPURPOSEIOREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register56_generalpurposeioregister::R`](R) reader structure"]
impl crate::Readable for REGISTER56_GENERALPURPOSEIOREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register56_generalpurposeioregister::W`](W) writer structure"]
impl crate::Writable for REGISTER56_GENERALPURPOSEIOREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER56_GENERALPURPOSEIOREGISTER to value 0"]
impl crate::Resettable for REGISTER56_GENERALPURPOSEIOREGISTER_SPEC {}
