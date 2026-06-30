#[doc = "Register `REGISTER15_INTERRUPTMASKREGISTER` reader"]
pub type R = crate::R<REGISTER15_INTERRUPTMASKREGISTER_SPEC>;
#[doc = "Register `REGISTER15_INTERRUPTMASKREGISTER` writer"]
pub type W = crate::W<REGISTER15_INTERRUPTMASKREGISTER_SPEC>;
#[doc = "Field `RGSMIIIM` reader - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 _Interrupt Status Register_"]
pub type RGSMIIIM_R = crate::BitReader;
#[doc = "Field `RGSMIIIM` writer - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 _Interrupt Status Register_"]
pub type RGSMIIIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSLCHGIM` reader - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Linkstatus changed bit in Register 14 _Interrupt Status Register_"]
pub type PCSLCHGIM_R = crate::BitReader;
#[doc = "Field `PCSLCHGIM` writer - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Linkstatus changed bit in Register 14 _Interrupt Status Register_"]
pub type PCSLCHGIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSANCIM` reader - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Autonegotiation complete bit in Register 14 _Interrupt Status Register_"]
pub type PCSANCIM_R = crate::BitReader;
#[doc = "Field `PCSANCIM` writer - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Autonegotiation complete bit in Register 14 _Interrupt Status Register_"]
pub type PCSANCIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMTIM` reader - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
pub type PMTIM_R = crate::BitReader;
#[doc = "Field `PMTIM` writer - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
pub type PMTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
pub type TSIM_R = crate::BitReader;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
pub type TSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIIM` reader - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
pub type LPIIM_R = crate::BitReader;
#[doc = "Field `LPIIM` writer - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
pub type LPIIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn rgsmiiim(&self) -> RGSMIIIM_R {
        RGSMIIIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Linkstatus changed bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pcslchgim(&self) -> PCSLCHGIM_R {
        PCSLCHGIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Autonegotiation complete bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pcsancim(&self) -> PCSANCIM_R {
        PCSANCIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn lpiim(&self) -> LPIIM_R {
        LPIIM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER15_INTERRUPTMASKREGISTER")
            .field("rgsmiiim", &self.rgsmiiim())
            .field("pcslchgim", &self.pcslchgim())
            .field("pcsancim", &self.pcsancim())
            .field("pmtim", &self.pmtim())
            .field("tsim", &self.tsim())
            .field("lpiim", &self.lpiim())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn rgsmiiim(&mut self) -> RGSMIIIM_W<'_, REGISTER15_INTERRUPTMASKREGISTER_SPEC> {
        RGSMIIIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Linkstatus changed bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pcslchgim(&mut self) -> PCSLCHGIM_W<'_, REGISTER15_INTERRUPTMASKREGISTER_SPEC> {
        PCSLCHGIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Autonegotiation complete bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pcsancim(&mut self) -> PCSANCIM_W<'_, REGISTER15_INTERRUPTMASKREGISTER_SPEC> {
        PCSANCIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W<'_, REGISTER15_INTERRUPTMASKREGISTER_SPEC> {
        PMTIM_W::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W<'_, REGISTER15_INTERRUPTMASKREGISTER_SPEC> {
        TSIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn lpiim(&mut self) -> LPIIM_W<'_, REGISTER15_INTERRUPTMASKREGISTER_SPEC> {
        LPIIM_W::new(self, 10)
    }
}
#[doc = "Contains the masks for generating the interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`register15_interruptmaskregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register15_interruptmaskregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER15_INTERRUPTMASKREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER15_INTERRUPTMASKREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register15_interruptmaskregister::R`](R) reader structure"]
impl crate::Readable for REGISTER15_INTERRUPTMASKREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register15_interruptmaskregister::W`](W) writer structure"]
impl crate::Writable for REGISTER15_INTERRUPTMASKREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER15_INTERRUPTMASKREGISTER to value 0"]
impl crate::Resettable for REGISTER15_INTERRUPTMASKREGISTER_SPEC {}
