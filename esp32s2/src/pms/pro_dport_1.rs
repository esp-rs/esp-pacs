#[doc = "Register `PRO_DPORT_1` reader"]
pub type R = crate::R<PRO_DPORT_1_SPEC>;
#[doc = "Register `PRO_DPORT_1` writer"]
pub type W = crate::W<PRO_DPORT_1_SPEC>;
#[doc = "Field `PRO_DPORT_APB_PERIPHERAL_FORBID` reader - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
pub type PRO_DPORT_APB_PERIPHERAL_FORBID_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_APB_PERIPHERAL_FORBID` writer - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
pub type PRO_DPORT_APB_PERIPHERAL_FORBID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_RTCSLOW_SPLTADDR` reader - Configure the split address of RTC FAST for PeriBus1 access."]
pub type PRO_DPORT_RTCSLOW_SPLTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_DPORT_RTCSLOW_SPLTADDR` writer - Configure the split address of RTC FAST for PeriBus1 access."]
pub type PRO_DPORT_RTCSLOW_SPLTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_R` reader - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_R_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_R` writer - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_W` reader - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_W_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_L_W` writer - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
pub type PRO_DPORT_RTCSLOW_L_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_R` reader - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_R_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_R` writer - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_W` reader - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_W_R = crate::BitReader;
#[doc = "Field `PRO_DPORT_RTCSLOW_H_W` writer - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
pub type PRO_DPORT_RTCSLOW_H_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_VALID` reader - Configure whether to enable read protection for user-configured FIFO address."]
pub type PRO_DPORT_RESERVE_FIFO_VALID_R = crate::FieldReader;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_VALID` writer - Configure whether to enable read protection for user-configured FIFO address."]
pub type PRO_DPORT_RESERVE_FIFO_VALID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
    #[inline(always)]
    pub fn pro_dport_apb_peripheral_forbid(&self) -> PRO_DPORT_APB_PERIPHERAL_FORBID_R {
        PRO_DPORT_APB_PERIPHERAL_FORBID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - Configure the split address of RTC FAST for PeriBus1 access."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_spltaddr(&self) -> PRO_DPORT_RTCSLOW_SPLTADDR_R {
        PRO_DPORT_RTCSLOW_SPLTADDR_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_l_r(&self) -> PRO_DPORT_RTCSLOW_L_R_R {
        PRO_DPORT_RTCSLOW_L_R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_l_w(&self) -> PRO_DPORT_RTCSLOW_L_W_R {
        PRO_DPORT_RTCSLOW_L_W_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_h_r(&self) -> PRO_DPORT_RTCSLOW_H_R_R {
        PRO_DPORT_RTCSLOW_H_R_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_h_w(&self) -> PRO_DPORT_RTCSLOW_H_W_R {
        PRO_DPORT_RTCSLOW_H_W_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Configure whether to enable read protection for user-configured FIFO address."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_valid(&self) -> PRO_DPORT_RESERVE_FIFO_VALID_R {
        PRO_DPORT_RESERVE_FIFO_VALID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_1")
            .field(
                "pro_dport_apb_peripheral_forbid",
                &self.pro_dport_apb_peripheral_forbid(),
            )
            .field(
                "pro_dport_rtcslow_spltaddr",
                &self.pro_dport_rtcslow_spltaddr(),
            )
            .field("pro_dport_rtcslow_l_r", &self.pro_dport_rtcslow_l_r())
            .field("pro_dport_rtcslow_l_w", &self.pro_dport_rtcslow_l_w())
            .field("pro_dport_rtcslow_h_r", &self.pro_dport_rtcslow_h_r())
            .field("pro_dport_rtcslow_h_w", &self.pro_dport_rtcslow_h_w())
            .field(
                "pro_dport_reserve_fifo_valid",
                &self.pro_dport_reserve_fifo_valid(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 denies PeriBus1 bus???s access to APB peripheral."]
    #[inline(always)]
    pub fn pro_dport_apb_peripheral_forbid(
        &mut self,
    ) -> PRO_DPORT_APB_PERIPHERAL_FORBID_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_APB_PERIPHERAL_FORBID_W::new(self, 0)
    }
    #[doc = "Bits 1:11 - Configure the split address of RTC FAST for PeriBus1 access."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_spltaddr(&mut self) -> PRO_DPORT_RTCSLOW_SPLTADDR_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_RTCSLOW_SPLTADDR_W::new(self, 1)
    }
    #[doc = "Bit 12 - Setting to 1 grants PeriBus1 permission to read RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_l_r(&mut self) -> PRO_DPORT_RTCSLOW_L_R_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_RTCSLOW_L_R_W::new(self, 12)
    }
    #[doc = "Bit 13 - Setting to 1 grants PeriBus1 permission to write RTC FAST low address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_l_w(&mut self) -> PRO_DPORT_RTCSLOW_L_W_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_RTCSLOW_L_W_W::new(self, 13)
    }
    #[doc = "Bit 14 - Setting to 1 grants PeriBus1 permission to read RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_h_r(&mut self) -> PRO_DPORT_RTCSLOW_H_R_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_RTCSLOW_H_R_W::new(self, 14)
    }
    #[doc = "Bit 15 - Setting to 1 grants PeriBus1 permission to write RTC FAST high address region."]
    #[inline(always)]
    pub fn pro_dport_rtcslow_h_w(&mut self) -> PRO_DPORT_RTCSLOW_H_W_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_RTCSLOW_H_W_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Configure whether to enable read protection for user-configured FIFO address."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_valid(
        &mut self,
    ) -> PRO_DPORT_RESERVE_FIFO_VALID_W<PRO_DPORT_1_SPEC> {
        PRO_DPORT_RESERVE_FIFO_VALID_W::new(self, 16)
    }
}
#[doc = "PeriBus1 permission control register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dport_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dport_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_1_SPEC;
impl crate::RegisterSpec for PRO_DPORT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_1::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_1::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_1 to value 0xf000"]
impl crate::Resettable for PRO_DPORT_1_SPEC {
    const RESET_VALUE: u32 = 0xf000;
}
