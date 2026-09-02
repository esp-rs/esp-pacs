#[doc = "Register `GMIIADDR` reader"]
pub type R = crate::R<GMIIADDR_SPEC>;
#[doc = "Register `GMIIADDR` writer"]
pub type W = crate::W<GMIIADDR_SPEC>;
#[doc = "Field `MIIBUSY` reader - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5 During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress Register 5 is invalid until this bit is cleared by the MAC Therefore, Register 5 _GMII Data_ should be kept valid until the MAC clears this bit during a PHY Write operation Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared The subsequent read or write operation should happen only after the previous operation is complete Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present"]
pub type MIIBUSY_R = crate::BitReader;
#[doc = "Field `MIIBUSY` writer - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5 During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress Register 5 is invalid until this bit is cleared by the MAC Therefore, Register 5 _GMII Data_ should be kept valid until the MAC clears this bit during a PHY Write operation Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared The subsequent read or write operation should happen only after the previous operation is complete Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present"]
pub type MIIBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIIWRITE` reader - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register"]
pub type MIIWRITE_R = crate::BitReader;
#[doc = "Field `MIIWRITE` writer - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register"]
pub type MIIWRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIICSRCLK` reader - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design The CSR clock corresponding to different GMAC configurations is given in Table 92 on page 564 The suggested range of CSR clock frequency applicable for each value _when Bit\\[5\\] = 0_ ensures that the MDC clock is approximately between the frequency range 10 MHz25 MHz 0000: The CSR clock frequency is 60100 MHz and the MDC clock frequency is CSR clock/42 0001: The CSR clock frequency is 100150 MHz and the MDC clock frequency is CSR clock/62 0010: The CSR clock frequency is 2035 MHz and the MDC clock frequency is CSR clock/16 0011: The CSR clock frequency is 3560 MHz and the MDC clock frequency is CSR clock/26 0100: The CSR clock frequency is 150250 MHz and the MDC clock frequency is CSR clock/102 0101: The CSR clock frequency is 250300 MHz and the MDC clock is CSR clock/124 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 25 MHz _specified in the IEEE Std 8023_ and program a clock divider of lower value For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 125 MHz which is outside the limit of IEEE 8023 specified range Program the following values only if the interfacing chips support faster MDC clocks 1000: CSR clock/4 1001: CSR clock/6 1010: CSR clock/8 1011: CSR clock/10 1100: CSR clock/12 1101: CSR clock/14 1110: CSR clock/16 1111: CSR clock/18 These bits are not used for accessing RevMII These bits are readonly if the RevMII interface is selected as single PHY interface"]
pub type MIICSRCLK_R = crate::FieldReader;
#[doc = "Field `MIICSRCLK` writer - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design The CSR clock corresponding to different GMAC configurations is given in Table 92 on page 564 The suggested range of CSR clock frequency applicable for each value _when Bit\\[5\\] = 0_ ensures that the MDC clock is approximately between the frequency range 10 MHz25 MHz 0000: The CSR clock frequency is 60100 MHz and the MDC clock frequency is CSR clock/42 0001: The CSR clock frequency is 100150 MHz and the MDC clock frequency is CSR clock/62 0010: The CSR clock frequency is 2035 MHz and the MDC clock frequency is CSR clock/16 0011: The CSR clock frequency is 3560 MHz and the MDC clock frequency is CSR clock/26 0100: The CSR clock frequency is 150250 MHz and the MDC clock frequency is CSR clock/102 0101: The CSR clock frequency is 250300 MHz and the MDC clock is CSR clock/124 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 25 MHz _specified in the IEEE Std 8023_ and program a clock divider of lower value For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 125 MHz which is outside the limit of IEEE 8023 specified range Program the following values only if the interfacing chips support faster MDC clocks 1000: CSR clock/4 1001: CSR clock/6 1010: CSR clock/8 1011: CSR clock/10 1100: CSR clock/12 1101: CSR clock/14 1110: CSR clock/16 1111: CSR clock/18 These bits are not used for accessing RevMII These bits are readonly if the RevMII interface is selected as single PHY interface"]
pub type MIICSRCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIIREG` reader - GMII Register These bits select the desired GMII register in the selected PHY device For RevMII, these bits select the desired CSR register in the RevMII Registers set"]
pub type MIIREG_R = crate::FieldReader;
#[doc = "Field `MIIREG` writer - GMII Register These bits select the desired GMII register in the selected PHY device For RevMII, these bits select the desired CSR register in the RevMII Registers set"]
pub type MIIREG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MIIDEV` reader - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed For RevMII, this field gives the PHY Address of the RevMII module"]
pub type MIIDEV_R = crate::FieldReader;
#[doc = "Field `MIIDEV` writer - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed For RevMII, this field gives the PHY Address of the RevMII module"]
pub type MIIDEV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5 During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress Register 5 is invalid until this bit is cleared by the MAC Therefore, Register 5 _GMII Data_ should be kept valid until the MAC clears this bit during a PHY Write operation Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared The subsequent read or write operation should happen only after the previous operation is complete Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present"]
    #[inline(always)]
    pub fn miibusy(&self) -> MIIBUSY_R {
        MIIBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register"]
    #[inline(always)]
    pub fn miiwrite(&self) -> MIIWRITE_R {
        MIIWRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design The CSR clock corresponding to different GMAC configurations is given in Table 92 on page 564 The suggested range of CSR clock frequency applicable for each value _when Bit\\[5\\] = 0_ ensures that the MDC clock is approximately between the frequency range 10 MHz25 MHz 0000: The CSR clock frequency is 60100 MHz and the MDC clock frequency is CSR clock/42 0001: The CSR clock frequency is 100150 MHz and the MDC clock frequency is CSR clock/62 0010: The CSR clock frequency is 2035 MHz and the MDC clock frequency is CSR clock/16 0011: The CSR clock frequency is 3560 MHz and the MDC clock frequency is CSR clock/26 0100: The CSR clock frequency is 150250 MHz and the MDC clock frequency is CSR clock/102 0101: The CSR clock frequency is 250300 MHz and the MDC clock is CSR clock/124 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 25 MHz _specified in the IEEE Std 8023_ and program a clock divider of lower value For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 125 MHz which is outside the limit of IEEE 8023 specified range Program the following values only if the interfacing chips support faster MDC clocks 1000: CSR clock/4 1001: CSR clock/6 1010: CSR clock/8 1011: CSR clock/10 1100: CSR clock/12 1101: CSR clock/14 1110: CSR clock/16 1111: CSR clock/18 These bits are not used for accessing RevMII These bits are readonly if the RevMII interface is selected as single PHY interface"]
    #[inline(always)]
    pub fn miicsrclk(&self) -> MIICSRCLK_R {
        MIICSRCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register These bits select the desired GMII register in the selected PHY device For RevMII, these bits select the desired CSR register in the RevMII Registers set"]
    #[inline(always)]
    pub fn miireg(&self) -> MIIREG_R {
        MIIREG_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed For RevMII, this field gives the PHY Address of the RevMII module"]
    #[inline(always)]
    pub fn miidev(&self) -> MIIDEV_R {
        MIIDEV_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMIIADDR")
            .field("miibusy", &self.miibusy())
            .field("miiwrite", &self.miiwrite())
            .field("miicsrclk", &self.miicsrclk())
            .field("miireg", &self.miireg())
            .field("miidev", &self.miidev())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5 During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress Register 5 is invalid until this bit is cleared by the MAC Therefore, Register 5 _GMII Data_ should be kept valid until the MAC clears this bit during a PHY Write operation Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared The subsequent read or write operation should happen only after the previous operation is complete Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present"]
    #[inline(always)]
    pub fn miibusy(&mut self) -> MIIBUSY_W<'_, GMIIADDR_SPEC> {
        MIIBUSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register"]
    #[inline(always)]
    pub fn miiwrite(&mut self) -> MIIWRITE_W<'_, GMIIADDR_SPEC> {
        MIIWRITE_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design The CSR clock corresponding to different GMAC configurations is given in Table 92 on page 564 The suggested range of CSR clock frequency applicable for each value _when Bit\\[5\\] = 0_ ensures that the MDC clock is approximately between the frequency range 10 MHz25 MHz 0000: The CSR clock frequency is 60100 MHz and the MDC clock frequency is CSR clock/42 0001: The CSR clock frequency is 100150 MHz and the MDC clock frequency is CSR clock/62 0010: The CSR clock frequency is 2035 MHz and the MDC clock frequency is CSR clock/16 0011: The CSR clock frequency is 3560 MHz and the MDC clock frequency is CSR clock/26 0100: The CSR clock frequency is 150250 MHz and the MDC clock frequency is CSR clock/102 0101: The CSR clock frequency is 250300 MHz and the MDC clock is CSR clock/124 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 25 MHz _specified in the IEEE Std 8023_ and program a clock divider of lower value For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 125 MHz which is outside the limit of IEEE 8023 specified range Program the following values only if the interfacing chips support faster MDC clocks 1000: CSR clock/4 1001: CSR clock/6 1010: CSR clock/8 1011: CSR clock/10 1100: CSR clock/12 1101: CSR clock/14 1110: CSR clock/16 1111: CSR clock/18 These bits are not used for accessing RevMII These bits are readonly if the RevMII interface is selected as single PHY interface"]
    #[inline(always)]
    pub fn miicsrclk(&mut self) -> MIICSRCLK_W<'_, GMIIADDR_SPEC> {
        MIICSRCLK_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - GMII Register These bits select the desired GMII register in the selected PHY device For RevMII, these bits select the desired CSR register in the RevMII Registers set"]
    #[inline(always)]
    pub fn miireg(&mut self) -> MIIREG_W<'_, GMIIADDR_SPEC> {
        MIIREG_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed For RevMII, this field gives the PHY Address of the RevMII module"]
    #[inline(always)]
    pub fn miidev(&mut self) -> MIIDEV_W<'_, GMIIADDR_SPEC> {
        MIIDEV_W::new(self, 11)
    }
}
#[doc = "PHY configuration access\n\nYou can [`read`](crate::Reg::read) this register and get [`gmiiaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmiiaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMIIADDR_SPEC;
impl crate::RegisterSpec for GMIIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmiiaddr::R`](R) reader structure"]
impl crate::Readable for GMIIADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmiiaddr::W`](W) writer structure"]
impl crate::Writable for GMIIADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMIIADDR to value 0"]
impl crate::Resettable for GMIIADDR_SPEC {}
