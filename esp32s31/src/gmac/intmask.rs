#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<INTMASK_SPEC>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<INTMASK_SPEC>;
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
#[doc = "Field `PMTINTMASK` reader - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
pub type PMTINTMASK_R = crate::BitReader;
#[doc = "Field `PMTINTMASK` writer - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
pub type PMTINTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
pub type TSIM_R = crate::BitReader;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
pub type TSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIINTMASK` reader - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
pub type LPIINTMASK_R = crate::BitReader;
#[doc = "Field `LPIINTMASK` writer - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
pub type LPIINTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn pmtintmask(&self) -> PMTINTMASK_R {
        PMTINTMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn lpiintmask(&self) -> LPIINTMASK_R {
        LPIINTMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMASK")
            .field("rgsmiiim", &self.rgsmiiim())
            .field("pcslchgim", &self.pcslchgim())
            .field("pcsancim", &self.pcsancim())
            .field("pmtintmask", &self.pmtintmask())
            .field("tsim", &self.tsim())
            .field("lpiintmask", &self.lpiintmask())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn rgsmiiim(&mut self) -> RGSMIIIM_W<'_, INTMASK_SPEC> {
        RGSMIIIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Linkstatus changed bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pcslchgim(&mut self) -> PCSLCHGIM_W<'_, INTMASK_SPEC> {
        PCSLCHGIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Autonegotiation complete bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pcsancim(&mut self) -> PCSANCIM_W<'_, INTMASK_SPEC> {
        PCSANCIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 _Interrupt Status Register_"]
    #[inline(always)]
    pub fn pmtintmask(&mut self) -> PMTINTMASK_W<'_, INTMASK_SPEC> {
        PMTINTMASK_W::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when IEEE1588 timestamping is enabled In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W<'_, INTMASK_SPEC> {
        TSIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 _Interrupt Status Register_ This bit is valid only when you select the Energy Efficient Ethernet feature during core configuration In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn lpiintmask(&mut self) -> LPIINTMASK_W<'_, INTMASK_SPEC> {
        LPIINTMASK_W::new(self, 10)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {}
