#[doc = "Register `RX_CRC_WIDTH` reader"]
pub type R = crate::R<RX_CRC_WIDTH_SPEC>;
#[doc = "Register `RX_CRC_WIDTH` writer"]
pub type W = crate::W<RX_CRC_WIDTH_SPEC>;
#[doc = "Field `RX_CRC_WIDTH` reader - reserved"]
pub type RX_CRC_WIDTH_R = crate::FieldReader;
#[doc = "Field `RX_CRC_WIDTH` writer - reserved"]
pub type RX_CRC_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_CRC_LAUTCH_FLGA` reader - reserved"]
pub type RX_CRC_LAUTCH_FLGA_R = crate::BitReader;
#[doc = "Field `RX_CRC_LAUTCH_FLGA` writer - reserved"]
pub type RX_CRC_LAUTCH_FLGA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    pub fn rx_crc_width(&self) -> RX_CRC_WIDTH_R {
        RX_CRC_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn rx_crc_lautch_flga(&self) -> RX_CRC_LAUTCH_FLGA_R {
        RX_CRC_LAUTCH_FLGA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_WIDTH")
            .field("rx_crc_width", &self.rx_crc_width())
            .field("rx_crc_lautch_flga", &self.rx_crc_lautch_flga())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    pub fn rx_crc_width(&mut self) -> RX_CRC_WIDTH_W<'_, RX_CRC_WIDTH_SPEC> {
        RX_CRC_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn rx_crc_lautch_flga(&mut self) -> RX_CRC_LAUTCH_FLGA_W<'_, RX_CRC_WIDTH_SPEC> {
        RX_CRC_LAUTCH_FLGA_W::new(self, 2)
    }
}
#[doc = "This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_width::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_width::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CRC_WIDTH_SPEC;
impl crate::RegisterSpec for RX_CRC_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_width::R`](R) reader structure"]
impl crate::Readable for RX_CRC_WIDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_width::W`](W) writer structure"]
impl crate::Writable for RX_CRC_WIDTH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_WIDTH to value 0"]
impl crate::Resettable for RX_CRC_WIDTH_SPEC {}
