#[doc = "Register `EMACGMIIADDR` reader"]
pub type R = crate::R<EMACGMIIADDR_SPEC>;
#[doc = "Register `EMACGMIIADDR` writer"]
pub type W = crate::W<EMACGMIIADDR_SPEC>;
#[doc = "Field `MIIBUSY` reader - This bit should read logic 0 before writing to PHY Addr Register and PHY data Register.During a PHY register access the software sets this bit to 1'b1 to indicate that a Read or Write access is in progress. PHY data Register is invalid until this bit is cleared by the MAC. Therefore PHY data Register (MII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed there is no change in the functionality of this bit even when the PHY is not Present."]
pub type MIIBUSY_R = crate::BitReader;
#[doc = "Field `MIIBUSY` writer - This bit should read logic 0 before writing to PHY Addr Register and PHY data Register.During a PHY register access the software sets this bit to 1'b1 to indicate that a Read or Write access is in progress. PHY data Register is invalid until this bit is cleared by the MAC. Therefore PHY data Register (MII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed there is no change in the functionality of this bit even when the PHY is not Present."]
pub type MIIBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIIWRITE` reader - When set this bit indicates to the PHY that this is a Write operation using the MII Data register. If this bit is not set it indicates that this is a Read operation that is placing the data in the MII Data register."]
pub type MIIWRITE_R = crate::BitReader;
#[doc = "Field `MIIWRITE` writer - When set this bit indicates to the PHY that this is a Write operation using the MII Data register. If this bit is not set it indicates that this is a Read operation that is placing the data in the MII Data register."]
pub type MIIWRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIICSRCLK` reader - CSR clock range: 1.0 MHz ~ 2.5 MHz. 4'b0000: When the APB clock frequency is 80 MHz the MDC clock frequency is APB CLK/42 4'b0011: When the APB clock frequency is 40 MHz the MDC clock frequency is APB CLK/26."]
pub type MIICSRCLK_R = crate::FieldReader;
#[doc = "Field `MIICSRCLK` writer - CSR clock range: 1.0 MHz ~ 2.5 MHz. 4'b0000: When the APB clock frequency is 80 MHz the MDC clock frequency is APB CLK/42 4'b0011: When the APB clock frequency is 40 MHz the MDC clock frequency is APB CLK/26."]
pub type MIICSRCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIIREG` reader - These bits select the desired MII register in the selected PHY device."]
pub type MIIREG_R = crate::FieldReader;
#[doc = "Field `MIIREG` writer - These bits select the desired MII register in the selected PHY device."]
pub type MIIREG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MIIDEV` reader - This field indicates which of the 32 possible PHY devices are being accessed."]
pub type MIIDEV_R = crate::FieldReader;
#[doc = "Field `MIIDEV` writer - This field indicates which of the 32 possible PHY devices are being accessed."]
pub type MIIDEV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - This bit should read logic 0 before writing to PHY Addr Register and PHY data Register.During a PHY register access the software sets this bit to 1'b1 to indicate that a Read or Write access is in progress. PHY data Register is invalid until this bit is cleared by the MAC. Therefore PHY data Register (MII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed there is no change in the functionality of this bit even when the PHY is not Present."]
    #[inline(always)]
    pub fn miibusy(&self) -> MIIBUSY_R {
        MIIBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set this bit indicates to the PHY that this is a Write operation using the MII Data register. If this bit is not set it indicates that this is a Read operation that is placing the data in the MII Data register."]
    #[inline(always)]
    pub fn miiwrite(&self) -> MIIWRITE_R {
        MIIWRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CSR clock range: 1.0 MHz ~ 2.5 MHz. 4'b0000: When the APB clock frequency is 80 MHz the MDC clock frequency is APB CLK/42 4'b0011: When the APB clock frequency is 40 MHz the MDC clock frequency is APB CLK/26."]
    #[inline(always)]
    pub fn miicsrclk(&self) -> MIICSRCLK_R {
        MIICSRCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - These bits select the desired MII register in the selected PHY device."]
    #[inline(always)]
    pub fn miireg(&self) -> MIIREG_R {
        MIIREG_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - This field indicates which of the 32 possible PHY devices are being accessed."]
    #[inline(always)]
    pub fn miidev(&self) -> MIIDEV_R {
        MIIDEV_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACGMIIADDR")
            .field("miibusy", &self.miibusy())
            .field("miiwrite", &self.miiwrite())
            .field("miicsrclk", &self.miicsrclk())
            .field("miireg", &self.miireg())
            .field("miidev", &self.miidev())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit should read logic 0 before writing to PHY Addr Register and PHY data Register.During a PHY register access the software sets this bit to 1'b1 to indicate that a Read or Write access is in progress. PHY data Register is invalid until this bit is cleared by the MAC. Therefore PHY data Register (MII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed there is no change in the functionality of this bit even when the PHY is not Present."]
    #[inline(always)]
    #[must_use]
    pub fn miibusy(&mut self) -> MIIBUSY_W<EMACGMIIADDR_SPEC> {
        MIIBUSY_W::new(self, 0)
    }
    #[doc = "Bit 1 - When set this bit indicates to the PHY that this is a Write operation using the MII Data register. If this bit is not set it indicates that this is a Read operation that is placing the data in the MII Data register."]
    #[inline(always)]
    #[must_use]
    pub fn miiwrite(&mut self) -> MIIWRITE_W<EMACGMIIADDR_SPEC> {
        MIIWRITE_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - CSR clock range: 1.0 MHz ~ 2.5 MHz. 4'b0000: When the APB clock frequency is 80 MHz the MDC clock frequency is APB CLK/42 4'b0011: When the APB clock frequency is 40 MHz the MDC clock frequency is APB CLK/26."]
    #[inline(always)]
    #[must_use]
    pub fn miicsrclk(&mut self) -> MIICSRCLK_W<EMACGMIIADDR_SPEC> {
        MIICSRCLK_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - These bits select the desired MII register in the selected PHY device."]
    #[inline(always)]
    #[must_use]
    pub fn miireg(&mut self) -> MIIREG_W<EMACGMIIADDR_SPEC> {
        MIIREG_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - This field indicates which of the 32 possible PHY devices are being accessed."]
    #[inline(always)]
    #[must_use]
    pub fn miidev(&mut self) -> MIIDEV_W<EMACGMIIADDR_SPEC> {
        MIIDEV_W::new(self, 11)
    }
}
#[doc = "PHY configuration access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacgmiiaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacgmiiaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACGMIIADDR_SPEC;
impl crate::RegisterSpec for EMACGMIIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacgmiiaddr::R`](R) reader structure"]
impl crate::Readable for EMACGMIIADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacgmiiaddr::W`](W) writer structure"]
impl crate::Writable for EMACGMIIADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMACGMIIADDR to value 0"]
impl crate::Resettable for EMACGMIIADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
