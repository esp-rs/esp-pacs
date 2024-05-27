///Register `HP_POR_RST_BYPASS_CTRL` reader
pub type R = crate::R<HP_POR_RST_BYPASS_CTRL_SPEC>;
///Register `HP_POR_RST_BYPASS_CTRL` writer
pub type W = crate::W<HP_POR_RST_BYPASS_CTRL_SPEC>;
///Field `HP_PO_CNNT_RSTN_BYPASS_CTRL` reader - 15\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \[14\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \[13\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \[12\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \[11\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \[10\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \[9\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \[8\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
pub type HP_PO_CNNT_RSTN_BYPASS_CTRL_R = crate::FieldReader;
///Field `HP_PO_CNNT_RSTN_BYPASS_CTRL` writer - 15\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \[14\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \[13\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \[12\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \[11\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \[10\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \[9\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \[8\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
pub type HP_PO_CNNT_RSTN_BYPASS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HP_PO_RSTN_BYPASS_CTRL` reader - 31\] 1'b1: po_rstn bypass sys_sw_rstn \[30\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \[29\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \[28\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \[27\] 1'b1: po_rstn bypass usb_jtag_chip_rst \[26\] 1'b1: po_rstn bypass usb_uart_chip_rst \[25\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \[24\] 1'b1: po_rstn bypass efuse_err_rstn
pub type HP_PO_RSTN_BYPASS_CTRL_R = crate::FieldReader;
///Field `HP_PO_RSTN_BYPASS_CTRL` writer - 31\] 1'b1: po_rstn bypass sys_sw_rstn \[30\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \[29\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \[28\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \[27\] 1'b1: po_rstn bypass usb_jtag_chip_rst \[26\] 1'b1: po_rstn bypass usb_uart_chip_rst \[25\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \[24\] 1'b1: po_rstn bypass efuse_err_rstn
pub type HP_PO_RSTN_BYPASS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 8:15 - 15\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \[14\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \[13\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \[12\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \[11\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \[10\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \[9\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \[8\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(&self) -> HP_PO_CNNT_RSTN_BYPASS_CTRL_R {
        HP_PO_CNNT_RSTN_BYPASS_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 24:31 - 31\] 1'b1: po_rstn bypass sys_sw_rstn \[30\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \[29\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \[28\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \[27\] 1'b1: po_rstn bypass usb_jtag_chip_rst \[26\] 1'b1: po_rstn bypass usb_uart_chip_rst \[25\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \[24\] 1'b1: po_rstn bypass efuse_err_rstn
    #[inline(always)]
    pub fn hp_po_rstn_bypass_ctrl(&self) -> HP_PO_RSTN_BYPASS_CTRL_R {
        HP_PO_RSTN_BYPASS_CTRL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_POR_RST_BYPASS_CTRL")
            .field(
                "hp_po_cnnt_rstn_bypass_ctrl",
                &self.hp_po_cnnt_rstn_bypass_ctrl(),
            )
            .field("hp_po_rstn_bypass_ctrl", &self.hp_po_rstn_bypass_ctrl())
            .finish()
    }
}
impl W {
    ///Bits 8:15 - 15\] 1'b1: po_cnnt_rstn bypass sys_sw_rstn \[14\] 1'b1: po_cnnt_rstn bypass hp_wdt_sys_rstn \[13\] 1'b1: po_cnnt_rstn bypass hp_cpu_intrusion_rstn \[12\] 1'b1: po_cnnt_rstn bypass hp_sdio_sys_rstn \[11\] 1'b1: po_cnnt_rstn bypass usb_jtag_chip_rst \[10\] 1'b1: po_cnnt_rstn bypass usb_uart_chip_rst \[9\] 1'b1: po_cnnt_rstn bypass lp_wdt_hp_sys_rstn \[8\] 1'b1: po_cnnt_rstn bypass efuse_err_rstn
    #[inline(always)]
    #[must_use]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(
        &mut self,
    ) -> HP_PO_CNNT_RSTN_BYPASS_CTRL_W<HP_POR_RST_BYPASS_CTRL_SPEC> {
        HP_PO_CNNT_RSTN_BYPASS_CTRL_W::new(self, 8)
    }
    ///Bits 24:31 - 31\] 1'b1: po_rstn bypass sys_sw_rstn \[30\] 1'b1: po_rstn bypass hp_wdt_sys_rstn \[29\] 1'b1: po_rstn bypass hp_cpu_intrusion_rstn \[28\] 1'b1: po_rstn bypass hp_sdio_sys_rstn \[27\] 1'b1: po_rstn bypass usb_jtag_chip_rst \[26\] 1'b1: po_rstn bypass usb_uart_chip_rst \[25\] 1'b1: po_rstn bypass lp_wdt_hp_sys_rstn \[24\] 1'b1: po_rstn bypass efuse_err_rstn
    #[inline(always)]
    #[must_use]
    pub fn hp_po_rstn_bypass_ctrl(
        &mut self,
    ) -> HP_PO_RSTN_BYPASS_CTRL_W<HP_POR_RST_BYPASS_CTRL_SPEC> {
        HP_PO_RSTN_BYPASS_CTRL_W::new(self, 24)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_por_rst_bypass_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_por_rst_bypass_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_POR_RST_BYPASS_CTRL_SPEC;
impl crate::RegisterSpec for HP_POR_RST_BYPASS_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hp_por_rst_bypass_ctrl::R`](R) reader structure
impl crate::Readable for HP_POR_RST_BYPASS_CTRL_SPEC {}
///`write(|w| ..)` method takes [`hp_por_rst_bypass_ctrl::W`](W) writer structure
impl crate::Writable for HP_POR_RST_BYPASS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_POR_RST_BYPASS_CTRL to value 0xff00_ff00
impl crate::Resettable for HP_POR_RST_BYPASS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xff00_ff00;
}
