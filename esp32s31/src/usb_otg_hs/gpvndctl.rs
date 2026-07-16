#[doc = "Register `GPVNDCTL` reader"]
pub type R = crate::R<GPVNDCTL_SPEC>;
#[doc = "Register `GPVNDCTL` writer"]
pub type W = crate::W<GPVNDCTL_SPEC>;
#[doc = "Field `REGDATA` reader - Register Data (RegData) Contains the write data for register write. Read data for register read, valid when VStatus Done is set."]
pub type REGDATA_R = crate::FieldReader;
#[doc = "Field `REGDATA` writer - Register Data (RegData) Contains the write data for register write. Read data for register read, valid when VStatus Done is set."]
pub type REGDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VCTRL` reader - UTMI+ Vendor Control Register Address (VCtrl) The 4-bit register address a vendor defined 4-bit parallel output bus. Bits 11:8 of this field are placed on utmi_vcontrol\\[3:0\\]. ULPI Extended Register Address (ExtRegAddr) The 6-bit PHY extended register address."]
pub type VCTRL_R = crate::FieldReader;
#[doc = "Field `VCTRL` writer - UTMI+ Vendor Control Register Address (VCtrl) The 4-bit register address a vendor defined 4-bit parallel output bus. Bits 11:8 of this field are placed on utmi_vcontrol\\[3:0\\]. ULPI Extended Register Address (ExtRegAddr) The 6-bit PHY extended register address."]
pub type VCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGADDR` reader - Register Address (RegAddr) The 6-bit PHY register address for immediate PHY Register set access. Set to 6'h2F for Extended PHY Register set access."]
pub type REGADDR_R = crate::FieldReader;
#[doc = "Field `REGADDR` writer - Register Address (RegAddr) The 6-bit PHY register address for immediate PHY Register set access. Set to 6'h2F for Extended PHY Register set access."]
pub type REGADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REGWR` reader - Register Write (RegWr) Set this bit for register writes, and clear it for register reads."]
pub type REGWR_R = crate::BitReader;
#[doc = "Field `REGWR` writer - Register Write (RegWr) Set this bit for register writes, and clear it for register reads."]
pub type REGWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEWREGREQ` reader - New Register Request (NewRegReq) The application sets this bit for a new vendor control access."]
pub type NEWREGREQ_R = crate::BitReader;
#[doc = "Field `NEWREGREQ` writer - New Register Request (NewRegReq) The application sets this bit for a new vendor control access."]
pub type NEWREGREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSTSBSY` reader - VStatus Busy (VStsBsy) The core sets this bit when the vendor control access is in progress and clears this bit when done."]
pub type VSTSBSY_R = crate::BitReader;
#[doc = "Field `VSTSDONE` reader - VStatus Done (VStsDone) The core sets this bit when the vendor control access is done. This bit is cleared by the core when the application sets the New Register Request bit (bit 25)."]
pub type VSTSDONE_R = crate::BitReader;
#[doc = "Field `VSTSDONE` writer - VStatus Done (VStsDone) The core sets this bit when the vendor control access is done. This bit is cleared by the core when the application sets the New Register Request bit (bit 25)."]
pub type VSTSDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISULPIDRVR` reader - Disable ULPI Drivers (DisUlpiDrvr) The application sets this bit when it has finished processing the ULPI Carkit Interrupt (GINTSTS.ULPICKINT). When set, the controller disables drivers for output signals and masks input signal for the ULPI interface. The controller clears this bit before enabling the ULPI interface."]
pub type DISULPIDRVR_R = crate::BitReader;
#[doc = "Field `DISULPIDRVR` writer - Disable ULPI Drivers (DisUlpiDrvr) The application sets this bit when it has finished processing the ULPI Carkit Interrupt (GINTSTS.ULPICKINT). When set, the controller disables drivers for output signals and masks input signal for the ULPI interface. The controller clears this bit before enabling the ULPI interface."]
pub type DISULPIDRVR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Register Data (RegData) Contains the write data for register write. Read data for register read, valid when VStatus Done is set."]
    #[inline(always)]
    pub fn regdata(&self) -> REGDATA_R {
        REGDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - UTMI+ Vendor Control Register Address (VCtrl) The 4-bit register address a vendor defined 4-bit parallel output bus. Bits 11:8 of this field are placed on utmi_vcontrol\\[3:0\\]. ULPI Extended Register Address (ExtRegAddr) The 6-bit PHY extended register address."]
    #[inline(always)]
    pub fn vctrl(&self) -> VCTRL_R {
        VCTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - Register Address (RegAddr) The 6-bit PHY register address for immediate PHY Register set access. Set to 6'h2F for Extended PHY Register set access."]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Register Write (RegWr) Set this bit for register writes, and clear it for register reads."]
    #[inline(always)]
    pub fn regwr(&self) -> REGWR_R {
        REGWR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - New Register Request (NewRegReq) The application sets this bit for a new vendor control access."]
    #[inline(always)]
    pub fn newregreq(&self) -> NEWREGREQ_R {
        NEWREGREQ_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - VStatus Busy (VStsBsy) The core sets this bit when the vendor control access is in progress and clears this bit when done."]
    #[inline(always)]
    pub fn vstsbsy(&self) -> VSTSBSY_R {
        VSTSBSY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - VStatus Done (VStsDone) The core sets this bit when the vendor control access is done. This bit is cleared by the core when the application sets the New Register Request bit (bit 25)."]
    #[inline(always)]
    pub fn vstsdone(&self) -> VSTSDONE_R {
        VSTSDONE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable ULPI Drivers (DisUlpiDrvr) The application sets this bit when it has finished processing the ULPI Carkit Interrupt (GINTSTS.ULPICKINT). When set, the controller disables drivers for output signals and masks input signal for the ULPI interface. The controller clears this bit before enabling the ULPI interface."]
    #[inline(always)]
    pub fn disulpidrvr(&self) -> DISULPIDRVR_R {
        DISULPIDRVR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPVNDCTL")
            .field("regdata", &self.regdata())
            .field("vctrl", &self.vctrl())
            .field("regaddr", &self.regaddr())
            .field("regwr", &self.regwr())
            .field("newregreq", &self.newregreq())
            .field("vstsbsy", &self.vstsbsy())
            .field("vstsdone", &self.vstsdone())
            .field("disulpidrvr", &self.disulpidrvr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Register Data (RegData) Contains the write data for register write. Read data for register read, valid when VStatus Done is set."]
    #[inline(always)]
    pub fn regdata(&mut self) -> REGDATA_W<'_, GPVNDCTL_SPEC> {
        REGDATA_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - UTMI+ Vendor Control Register Address (VCtrl) The 4-bit register address a vendor defined 4-bit parallel output bus. Bits 11:8 of this field are placed on utmi_vcontrol\\[3:0\\]. ULPI Extended Register Address (ExtRegAddr) The 6-bit PHY extended register address."]
    #[inline(always)]
    pub fn vctrl(&mut self) -> VCTRL_W<'_, GPVNDCTL_SPEC> {
        VCTRL_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Register Address (RegAddr) The 6-bit PHY register address for immediate PHY Register set access. Set to 6'h2F for Extended PHY Register set access."]
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W<'_, GPVNDCTL_SPEC> {
        REGADDR_W::new(self, 16)
    }
    #[doc = "Bit 22 - Register Write (RegWr) Set this bit for register writes, and clear it for register reads."]
    #[inline(always)]
    pub fn regwr(&mut self) -> REGWR_W<'_, GPVNDCTL_SPEC> {
        REGWR_W::new(self, 22)
    }
    #[doc = "Bit 25 - New Register Request (NewRegReq) The application sets this bit for a new vendor control access."]
    #[inline(always)]
    pub fn newregreq(&mut self) -> NEWREGREQ_W<'_, GPVNDCTL_SPEC> {
        NEWREGREQ_W::new(self, 25)
    }
    #[doc = "Bit 27 - VStatus Done (VStsDone) The core sets this bit when the vendor control access is done. This bit is cleared by the core when the application sets the New Register Request bit (bit 25)."]
    #[inline(always)]
    pub fn vstsdone(&mut self) -> VSTSDONE_W<'_, GPVNDCTL_SPEC> {
        VSTSDONE_W::new(self, 27)
    }
    #[doc = "Bit 31 - Disable ULPI Drivers (DisUlpiDrvr) The application sets this bit when it has finished processing the ULPI Carkit Interrupt (GINTSTS.ULPICKINT). When set, the controller disables drivers for output signals and masks input signal for the ULPI interface. The controller clears this bit before enabling the ULPI interface."]
    #[inline(always)]
    pub fn disulpidrvr(&mut self) -> DISULPIDRVR_W<'_, GPVNDCTL_SPEC> {
        DISULPIDRVR_W::new(self, 31)
    }
}
#[doc = "The application can use this register to access PHY registers. It is implemented only if Enable PHY Vendor Control Interface was selected during coreConsultant configuration (parameter OTG_VENDOR_CTL_INTERFACE = 1). For a UTMI+ PHY, the DWC_otg core uses the UTMI+ Vendor Control interface for PHY register access. For a ULPI PHY, the core uses the ULPI interface for PHY register access. The application sets Vendor Control register for PHY register access and times the PHY register access. The application polls the VStatus Done bit in this register for the completion of the PHY register access.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpvndctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpvndctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPVNDCTL_SPEC;
impl crate::RegisterSpec for GPVNDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpvndctl::R`](R) reader structure"]
impl crate::Readable for GPVNDCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpvndctl::W`](W) writer structure"]
impl crate::Writable for GPVNDCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPVNDCTL to value 0"]
impl crate::Resettable for GPVNDCTL_SPEC {}
