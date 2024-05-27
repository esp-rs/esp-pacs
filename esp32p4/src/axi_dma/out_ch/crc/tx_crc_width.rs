#[doc = "Register `TX_CRC_WIDTH` reader"]
pub type R = crate::R<TX_CRC_WIDTH_SPEC>;
#[doc = "Register `TX_CRC_WIDTH` writer"]
pub type W = crate::W<TX_CRC_WIDTH_SPEC>;
#[doc = "Field `TX_CRC_WIDTH` reader - reserved"]
pub type TX_CRC_WIDTH_R = crate::FieldReader;
#[doc = "Field `TX_CRC_WIDTH` writer - reserved"]
pub type TX_CRC_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_CRC_LAUTCH_FLGA` reader - reserved"]
pub type TX_CRC_LAUTCH_FLGA_R = crate::BitReader;
#[doc = "Field `TX_CRC_LAUTCH_FLGA` writer - reserved"]
pub type TX_CRC_LAUTCH_FLGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    pub fn tx_crc_width(&self) -> TX_CRC_WIDTH_R {
        TX_CRC_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn tx_crc_lautch_flga(&self) -> TX_CRC_LAUTCH_FLGA_R {
        TX_CRC_LAUTCH_FLGA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_WIDTH")
            .field("tx_crc_width", &self.tx_crc_width())
            .field("tx_crc_lautch_flga", &self.tx_crc_lautch_flga())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_width(&mut self) -> TX_CRC_WIDTH_W<TX_CRC_WIDTH_SPEC> {
        TX_CRC_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_lautch_flga(&mut self) -> TX_CRC_LAUTCH_FLGA_W<TX_CRC_WIDTH_SPEC> {
        TX_CRC_LAUTCH_FLGA_W::new(self, 2)
    }
}
#[doc = "This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_WIDTH_SPEC;
impl crate::RegisterSpec for TX_CRC_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_width::R`](R) reader structure"]
impl crate::Readable for TX_CRC_WIDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_width::W`](W) writer structure"]
impl crate::Writable for TX_CRC_WIDTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_CRC_WIDTH to value 0"]
impl crate::Resettable for TX_CRC_WIDTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
